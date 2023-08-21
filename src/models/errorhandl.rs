use std::process::exit; use color_print:: cprintln;


pub fn exiting_shell(code: i32) {
    let bye = emojis::get("👋").unwrap();
    cprintln!("<cyan!><bold> {} You Are Exiting Rustain Shell! I Will Miss You</>", bye);
    exit(code);
}