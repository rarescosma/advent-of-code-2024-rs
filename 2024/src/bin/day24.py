from collections import defaultdict
from typing import NamedTuple

HALF_ADDER_TT = [0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0]
FULL_ADDER_TT = [0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 0, 1]
LAST_BIT_TT = [0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1]
MAX_BITS = 45
BF_MAX_DEPTH = 6
DEBUG = True


class CycleError(RuntimeError):
    pass


class TestResult(NamedTuple):
    ok: bool
    matching_nodes: set[str]


circuit: dict[str, tuple[str, str, str] | int] = {}

for line in open(0).read().split("\n\n")[1].splitlines():
    (left, gate, right, _, output) = line.split()
    circuit[output] = (gate, left, right)


def dbg(*args):
    if DEBUG:
        print(*args)


def run(x: int, y: int) -> dict[str, int]:
    fill("x", x)
    fill("y", y)

    return {
        node: eval_(*circuit[node], seen=set())
        for node in circuit.keys()
        if not is_input(node)
    }


def fill(prefix: str, val: int) -> None:
    bit_pos = 0
    while val > 0:
        circuit[make_id(prefix, bit_pos)] = val & 1
        val = val >> 1
        bit_pos = bit_pos + 1
    for empty in range(bit_pos, MAX_BITS):
        circuit[make_id(prefix, empty)] = 0


def eval_(gate_: str, left_: str, right_: str, seen: set[str]) -> int:
    if left_ in seen or right_ in seen:
        raise CycleError

    left_ = (
        circuit[left_]
        if is_input(left_)
        else eval_(*circuit[left_], seen=seen | {left_})
    )
    right_ = (
        circuit[right_]
        if is_input(right_)
        else eval_(*circuit[right_], seen=seen | {right_})
    )

    if gate_ == "AND":
        return left_ & right_
    elif gate_ == "XOR":
        return left_ ^ right_
    elif gate_ == "OR":
        return left_ | right_

    raise ValueError("Unknown op")


def fix_bit(bit_pos: int) -> list[str]:
    test_result = mini_test(bit_pos)
    output_id = make_id("z", bit_pos)
    if test_result.ok:
        dbg(f"== {output_id} OK")
        return []

    if test_result.matching_nodes:
        node_id = test_result.matching_nodes.pop()
        dbg(f"===> Retest after ({node_id} <-> {output_id}) swap")
        swap(output_id, node_id)
        assert mini_test(bit_pos).ok
        dbg("<=== Success!")
        return [output_id, node_id]
    else:
        bf_nodes = [
            node
            for node in brute_force_cands(make_id("z", bit_pos + 1))
            if not is_input(node)
        ]
        dbg(f"===> Mini brute-forcing swaps between {bf_nodes}")
        for i in range(len(bf_nodes) - 1):
            for j in range(i + 1, len(bf_nodes)):
                swap(bf_nodes[i], bf_nodes[j])
                try:
                    if all(mini_test(bit_pos + off).ok for off in (-1, 0, 1)):
                        dbg(
                            "<=== Brute-forced a valid swap:"
                            f" ({bf_nodes[i]} <-> {bf_nodes[j]})"
                        )
                        return [bf_nodes[i], bf_nodes[j]]
                except CycleError:
                    pass
                # reverse the swap if the test failed
                swap(bf_nodes[i], bf_nodes[j])
    return []


def mini_test(bit_pos: int) -> TestResult:
    """
    Evaluate truth tables of all nodes for all binary combinations of
     - x[bit_pos], y[bit_pos]
     - x[bit_pos - 1], y[bit_pos - 1] (for carry)
    """
    truth_tables = defaultdict(list)

    for x in range(0, 4):
        x = x << max(bit_pos - 1, 0)
        for y in range(0, 4):
            y = y << max(bit_pos - 1, 0)
            for node, value in run(x, y).items():
                truth_tables[node].append(value)

    check_table = {0: HALF_ADDER_TT, MAX_BITS: LAST_BIT_TT}.get(bit_pos, FULL_ADDER_TT)
    matching_nodes = {
        node for node, table in truth_tables.items() if table == check_table
    }
    return TestResult(
        make_id("z", bit_pos) in matching_nodes,
        {node for node in matching_nodes if not node.startswith("z")},
    )


def brute_force_cands(start_node: str, depth: int = 0):
    if depth >= BF_MAX_DEPTH or is_input(start_node) or start_node not in circuit:
        return []

    (left_, right_) = circuit[start_node][1:]
    yield from (left_, right_)
    yield from brute_force_cands(left_, depth + 1)
    yield from brute_force_cands(right_, depth + 1)


def swap(x: str, y: str) -> None:
    circuit[x], circuit[y] = circuit[y], circuit[x]


def make_id(prefix: str, bit_pos: int) -> str:
    return prefix + str(bit_pos).zfill(2)


def is_input(node_id: str) -> bool:
    return node_id and node_id[0] in "xy"


swaps = [swap for bit in range(0, MAX_BITS + 1) for swap in fix_bit(bit)]

swaps.sort()
print(",".join(swaps))
