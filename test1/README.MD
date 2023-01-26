<h2 align="center"><code> test1 </code></h2>
<h3 align="center"><i> Testing yansi  </i></h3>

----
1. [WTF](#wtf)
   1. [Recommended way](#recommended-way)
2. [Stucture](#stucture)
3. [To Do](#to-do)
4. [Releases information](#releases-information)
5. [Ideas](#ideas)

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

# Releases information 

[Branch w1 - test1 Dir](https://github.com/liqowomo/rust-syscoms/releases/tag/rsc0.0.2) 
- Note this doesnt just work OOTB when dowloading the exe 
- First you have to change its permissions and then it will execute 

# Ideas 

1. As you make these exes 
2. Note with ethers-rs for example you can build some sort of application binary 
   1. Then if this works in the fucking cli 
   2. You could make fucking Tauri app 
      1. Or you can also use dioxsys and make a web app 
         1. These tests has to be done on your shit acer bastard fucked u laptop , which has 4gb ram and 500gb HDD , fucking hate being poor , one day , you will have so many women , you will ask them to stay away 

