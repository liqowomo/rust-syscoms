/*
Testing out system commands execution in rust
*/
// Yansi crate adding paint style color
#[warn(unused_imports)]
use ::yansi::{Color, Paint, Style};

fn main() {
    colo_1();
}

fn colo_1() {
    // Define Colors as u want to use them
    let talk_1 = Style::new(Color::Green).bold().italic();
    println!(
        "{}",
        talk_1.paint(
            "
================================
Running system commands in Rust
================================"
        )
    );
}
