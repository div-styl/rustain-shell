#[warn(dead_code)]
mod models;
use models::dashboard::dashboard;
fn main() {
    let mut flag: bool = true;
    dashboard(&mut flag);
}
