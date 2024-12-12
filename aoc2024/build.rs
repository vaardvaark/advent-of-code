use std::process::Command;

fn main() {
    for day in 0..=25 {
        assert!(Command::new("touch")
            .arg(format!("src/input/day{day:02}.in"))
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
    }
}
