use std::fs::File;
use std::io::BufReader;
use code_timing_macros::time_snippet;
use anyhow::*;

#[macro_export]
macro_rules! day {
    () => {
        file!().replace("\\", "/")
            .rsplit_once("/").map(|(_, last)| last).unwrap()
            .rsplit_once('.').map(|(before, _)| before).unwrap()
    }
}

pub fn run_on_day_input<F, R>(day: &str, operation: F) -> Result<R>
where
    F: Fn(BufReader<File>) -> Result<R>,
    R: std::fmt::Display,
{
    let input_path = format!("input/{}.txt", day);
    let input_file = BufReader::new(File::open(input_path)?);
    let result = time_snippet!(operation(input_file)?);
    println!("Result = {}", result);
    Ok(result)
}