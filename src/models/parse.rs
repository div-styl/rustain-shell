pub fn parsecmd(cmdline: &str) -> (String, Vec<String>) {
    let trimmed_cmdline = cmdline.trim(); // Remove leading and trailing whitespace
    let mut parts = trimmed_cmdline.split_whitespace();
    let command = parts.next().unwrap_or("").to_string();
    let arguments: Vec<String> = parts.map(|s| s.to_string()).collect();
    (command, arguments)
}
