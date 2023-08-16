use color_print::{cprint, cprintln};
use std::{
    io::{self, Write},
    process::exit,
};

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
fn input(cmdline: &mut String) -> String {
    io::stdin()
        .read_line(cmdline)
        .expect("failed to read the command?");
    cmdline.clone()
}

pub fn dashboard(flag: &mut bool) {
    let prompt: String = String::from("root>> ");
    let mut cmd: String = String::new();

    // create while loop for prompting the input
    while *flag {
        cprint!("<green>{}</>", prompt);
        output();
        cmd.clear();

        input(&mut cmd);
        if cmd.is_empty() {
            cprintln!("<red><bold>BYE</bold></red>");
            exit(1);
        } else {
            print!("{}", cmd);
        }
    }
}
