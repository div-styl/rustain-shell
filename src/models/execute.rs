use std::process::Command;
use color_print::cprintln;

use crate::models::dashboard::prompt;

/**
 * * execmd - function that execute the command
 * @cmdline: A mutable string which refurs to the output
 * @argu: A mutable string which refurs to the output
 * * Return: void
*/
pub fn execmd(cmdline: &str, argu: &[String]) {
    
    let output = Command::new(cmdline)
        .args(argu)
        .status();

    match cmdline {
        "" => {
            // *empty input do nothing!
        }
        _ => {
            // Handle non-exit command here
            if output.is_err(){
                prompt(false);
                cprintln!("<red!><bold>Error: Command Not Found: {}</>", cmdline );
            }
        }
    }
}
