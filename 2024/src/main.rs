use std::error::Error;
use std::process::Command;

fn extract_microseconds(output: &str) -> Result<usize, Box<dyn Error>> {
    let out = output.lines().last().unwrap();
    let time = if out.ends_with("ms") {
        out["Time: ".len()..out.len() - 2].parse::<usize>()? * 1000
    } else {
        out["Time: ".len()..out.len() - 3].parse::<usize>()?
    };
    Ok(time)
}

fn main() {
    let dot_dir = std::env::current_exe()
        .map(|x| x.parent().unwrap().to_owned())
        .expect("cannot get current exe");

    let total_time = (1..=14)
        .filter_map(|day_num| {
            let cmd = Command::new(dot_dir.join(format!("day{day_num:0>2}")))
                .output()
                .unwrap();
            let output = String::from_utf8(cmd.stdout).unwrap();
            println!("Day {day_num:0>2}:\n{output}");
            extract_microseconds(&output).ok()
        })
        .sum::<usize>();
    println!("Total time: {}ms", total_time / 1000);
}
