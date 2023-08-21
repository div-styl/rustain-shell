use std::io::{self, Write}; use color_print::cprint;
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