use color_print::{cprint, cprintln};
use std::{
    io::{self, Write},
    process::exit,
};
use crate::models::execute::execmd;
use super::parse;



/**
 * *input - function that get the input from user
 * @cmdline: A mutable string which refurs to the output
 * Return: string input
*/
pub fn input(cmdline: &mut String) {
    io::stdin()
        .read_line(cmdline)
        .expect("failed to read the cmdline");
}


/**
 * *prompt - function that print the prompt
 * *Return: void
*/
pub fn prompt () {
    let prompt = String::from("〉");
    cprint!("<magenta!><bold>{}</> ",prompt);
    io::stdout().flush().expect("failed to return prompt");
}


/**
* *dashboard - function which will hold most op of shell
* flag: works as infinite loop
* Return: void
*/
pub fn dashboard(flag: &mut bool) {
    let mut cmd: String = String::new();

    while *flag {
        prompt(); // print the prompt
        cmd.clear(); // clear the input for avoiding duplication or appending the input
        input(&mut cmd); // Read input from the user

        let (cmdline, argu) = parse::parsecmd(&cmd);
        
        // check if the input is empty or not (if empty then exit the shell using Ctrl+D)
        if cmdline == "exit" || cmdline == "quit" {
            cprintln!("<red><bold>BYE</bold></red>");
            exit(0);
        } else {
            // println!("{} {:?}",cmdline, argu);
            execmd(&cmdline, &argu); // execute the command
        }
    }
}
