use super::{errorhandl::exiting_shell, parse};
use crate::models::execute::execmd;
use color_print::cprint;
use std::io::{self, Write};

/**
 * *input - function that get the input from user
 * @cmdline: A mutable string which refurs to the output
 * Return: string input
*/
pub fn input(cmdline: &mut String) {
    if io::stdin()
        .read_line(cmdline)
        .expect("failed to read the cmdline")
        == 0
    {
        exiting_shell(0);
    }
}

/**
 * *prompt - function that print the prompt
 * *Return: void
*/
pub fn prompt(check: bool) {
    let saluting = emojis::get("🫡").unwrap();
    let anger = emojis::get("😡").unwrap();
    if check == true
    {
        let prompt = String::from(" 〉");
        cprint!("<magenta!><bold>{} {}</> ",saluting, prompt);
        io::stdout().flush().expect("failed to return prompt");
    } 
    else if check == false
    {
        let prompt = String::from(" 〉");
        cprint!("<red!><bold>{}{}</> ",anger,prompt);
        io::stdout().flush().expect("failed to return prompt");
    }

}

/**
* *dashboard - function which will hold most op of shell
* flag: works as infinite loop
* Return: void
*/
pub fn dashboard(flag: &mut bool) {
    let mut cmd: String = String::new();

    while *flag {
        prompt(true); // print the prompt
        cmd.clear(); // clear the input for avoiding duplication or appending the input
        input(&mut cmd); // Read input from the user

        let (cmdline, argu) = parse::parsecmd(&cmd);

        // check if the input is empty or not (if empty then exit the shell using Ctrl+D)
        if cmdline == "exit" || cmdline == "quit" {
            exiting_shell(0);
        } else {
            // println!("{} {:?}",cmdline, argu);
            execmd(&cmdline, &argu); // execute the command
        }
    }
}