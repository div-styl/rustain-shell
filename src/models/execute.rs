use crate::models::dashboard::prompt;
use color_print::cprintln;
use std::env;
use std::process::Command;

/**
 * * execmd - function that execute the command
 * @cmdline: A mutable string which refurs to the output
 * @argu: A mutable string which refurs to the output
 * * Return: void
*/
pub fn execmd(cmdline: &str, argu: &[String]) {
    let output = Command::new(cmdline).args(argu).status();

    match cmdline {
        "" => {
            // *empty input do nothing!
        }
        "cd" => {
            let home = env::var("HOME").unwrap_or_default(); // fectch the home dir
            if argu.len() == 1 {
                if &argu[0] == "~" {
                    cprintln!("<green><bold>Changing to Home Directory</>");
                    if let Err(err) = env::set_current_dir(&home) {
                        prompt(false);
                        cprintln!("<red><bold>Error: Directory Change Failed: {}</>", err);
                    }
                } else {
                    if let Err(err) = env::set_current_dir(&argu[0]) {
                        prompt(false);
                        cprintln!("<red><bold>Error: Directory Change Failed 2: {}</>", err);
                    }
                }
            }
        }
        _ => {
            // Handle non-exit command here
            if output.is_err() {
                prompt(false);
                cprintln!("<red!><bold>Error: Command Not Found: {}</>", cmdline);
            }
        }
    }
}
