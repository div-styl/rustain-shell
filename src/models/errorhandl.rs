use std::{io::{self, Write}, process::exit}; use color_print::{cprint, cprintln};
/**
 * *errorprompt - function that print the error prompt
 * *Return: void
*/
pub fn errorprompt()
{
    let prompt = String::from("〉");
    cprint!("<red!><bold>{}</> ",prompt);
    io::stdout().flush().expect("failed to return prompt");
}

pub fn exiting_shell(code: i32) {
    let bye = emojis::get("👋").unwrap();
    cprintln!("<cyan!><bold> {} You Are Exiting Rustain Shell! I Will Miss You</>", bye);
    exit(code);
}