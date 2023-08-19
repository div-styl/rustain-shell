pub fn parsecmd(cmdline: &str) -> (String, Vec<String>) {
    // Remove leading and trailing whitespace
    let cmdline = cmdline.trim();

    // Split the cmdline into command and arguments
    let mut parts = cmdline.split_whitespace();
    let command = parts.next().unwrap_or("").to_string();
    let arguments: Vec<String> = parts.map(|arg| arg.to_string()).collect();

    (command, arguments)
}