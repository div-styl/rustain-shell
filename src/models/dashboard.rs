use color_print::{cprint, cprintln};
use std::{
    io::{self, Write},
    process::exit,
};


use super::parse;
/**
 * *output - function that flush the prompt
 * @paramter: none
 * Return: none
 */
fn output() {
    io::stdout().flush().expect("failed to return prompt");
}

/**
 * *input - function that get the input from user
 * @cmdline: A mutable string which refurs to the output
 * Return: string input
 */
fn input(cmdline: &mut String) {
    io::stdin()
        .read_line(cmdline)
        .expect("failed to read the cmdline");
}

/**
* *dashboard - function which will hold most op of shell
* flag: works as infinite loop
* Return: void
*/
pub fn dashboard(flag: &mut bool) {
    let prompt: String = String::from("Rshell>> ");
    let mut cmd: String = String::new();

    while *flag {
        cprint!("<green>{}</>", prompt);
        output();
        cmd.clear();
        input(&mut cmd); // Read input from the user

        let (cmdline, argu) = parse::parsecmd(&cmd);
        
        if cmdline.is_empty() {
            cprintln!("<red><bold>BYE</bold></red>");
            exit(1);
        } else {
            println!("{} {:?}",cmdline, argu);
        }
    }
}