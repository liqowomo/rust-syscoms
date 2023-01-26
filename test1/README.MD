<h2 align="center"><code> test1 </code></h2>
<h3 align="center"><i> Testing yansi  </i></h3>

----

----

# WTF 

1. This program tests put the color options of `yansi` as decribed in its [ manual`](https://docs.rs/yansi/latest/yansi/)
2. Implementing only what is the front page 
3. There is some other stuff that is not understood by me yet 

## Recommended way

```rs
use yansi::{Paint, Color, Style};

// A bold, itatlic style with red foreground.
let alert = Style::new(Color::Red).bold().italic();

// Using `style.paint()`; this is preferred.
println!("Alert! {}", alert.paint("This is serious business!"));
println!("Hi! {}", alert.underline().paint("Super serious!"));

// Using `paint.with_style()`.
println!("Alert! {}", Paint::new("Yet another.").with_style(alert));
```
- I like this styel and have impl it in the code 


# Stucture 

1. Main function calls sub functions that have actual color and sytling

# To Do 

- [ ] Add banner with color 
- [ ] Setup GITHUB Actions Pipe line for releases