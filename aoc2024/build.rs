use std::process::Command;

fn main() {
    for day in 0..=25 {
        Command::new("touch")
            .arg(format!("src/input/day{day:02}.in"))
            .spawn()
            .unwrap();
    }
}
