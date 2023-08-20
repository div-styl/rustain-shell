use std::process::{Command, exit};

use color_print::cprintln;

pub fn execmd(cmdline: &str, argu: &[String]) {
    let output = Command::new(cmdline)
        .args(argu)
        .status();

    match cmdline {
        "exit" => {
            cprintln!("<green><bold> You Are Exiting Rustain Shell! I Will Miss You</>");
            exit(0);
        }
        _ => {
            // Handle non-exit command here
            if output.is_err(){
                cprintln!("<red><bold>Error executing command: {}</>", cmdline);
            }
        }
    }
}