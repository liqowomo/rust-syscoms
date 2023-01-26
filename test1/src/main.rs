use std::borrow::Borrow;

/*
Testing out some system commands with this
*/
// Crate for color
#[warn(unused_imports)]
use yansi::{Color, Paint, Style};

fn main() {
    print!(
        "{}",
        Paint::red(
            "
Printing Out using YANSI as sub functions"
        )
        .bg(Color::Yellow)
        .italic()
        .bold()
    );
    colp();
    Colp_sytle()
}

//Color text function here
fn colp() {
    print!(
        "
        {}
        {}
        ",
        Paint::blue("The fuck you doing to do son ?"),
        Paint::red("bastard ! ")
    )
}

// Checkuing out the styles

fn Colp_sytle() {
    // Definins styles as variables here
    let Cst = Style::new(Color::Magenta).bold().blink();

    print!("{}", Cst.paint("Ass and pussy"));
}
