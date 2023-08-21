/**
 * *parsecmd - function which parse cmdline into cmd and argu like (ls -l)
 * @cmdline: str which represent the main cmdline
 * * Return: string and vector of string type
 */


pub fn parsecmd(cmdline: &str) -> (String, Vec<String>) {
    // Remove leading and trailing whitespace
    let cmdline = cmdline.trim();

    // Split the cmdline into command and arguments
    let mut parts = cmdline.split_whitespace();
    let command = parts.next().unwrap_or("").to_string();
    let arguments: Vec<String> = parts.map(|arg| arg.to_string()).collect();

    (command, arguments)
}