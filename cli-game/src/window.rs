use std::process::Command;
use std::process::Output;

pub struct Window {
    x: i32,
    y: i32,
}

impl Window {
    pub fn new() -> Window {
        let cols = if cfg!(target_os = "windows") {
            Command::new("sh")
                .arg("-c")
                .arg("tput cols")
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("tput cols")
                .output()
                .expect("failed to execute process")
        };
        let lines = if cfg!(target_os = "windows") {
            Command::new("sh")
                .arg("-c")
                .arg("tput lines")
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("tput lines")
                .output()
                .expect("failed to execute process")
        };
        fn output2int(out: Output) -> i32 {
            String::from_utf8_lossy(&out.stdout).trim().parse::<i32>().unwrap()
        }
        Window {
            x: output2int(cols),
            y: output2int(lines)
        }
    }
    pub fn calc_v(&self) -> i32 {
        &self.x * &self.y
    }

}
