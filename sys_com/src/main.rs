/*
Testing out system commands execution in rust
*/
// Yansi crate adding paint style color
#[warn(unused_imports)]

use ::yansi::{Color, Paint, Style};

// this crate is for accessign system commands
use std::process::Command;

fn main() {
    colo_1();
    sys_co_1();
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

// System command test function
fn sys_co_1() {
    let talk_2 = Style::new(Color::Blue);
    print!("{}", talk_2.paint("Running Command ----
    "));

    // Sub functions 

    fn com() {
       print!("hey");
    }

}
