use std::process::{Command, exit};
use color_print::cprintln;

use crate::models::errorhandl::errorprompt;

/**
 * * execmd - function that execute the command
 * @cmdline: A mutable string which refurs to the output
 * @argu: A mutable string which refurs to the output
 * * Return: void
*/
pub fn execmd(cmdline: &str, argu: &[String]) {
    let wrogcmd = emojis::get("😡").unwrap();
    let output = Command::new(cmdline)
        .args(argu)
        .status();

    match cmdline {
        "exit" => {
            cprintln!("<green><bold> You Are Exiting Rustain Shell! I Will Miss You</>");
            exit(0);
        }
        "" => {
            
        }
        _ => {
            // Handle non-exit command here
            if output.is_err(){
                errorprompt();
                cprintln!("<red><bold>{} Error: Command Not Found: {}</>", wrogcmd, cmdline );
            }
        }
    }
}
