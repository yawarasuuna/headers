use std::env;

use std::{thread, time};
use clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    let input = env::args().collect::<Vec<String>>()[1..].join(" ");

    let output = format!(
        "{}\n{}{}{}\n{}",
        "    /*//////////////////////////////////////////////////////////////",
        "    ",
        (0..(64 - input.len()) / 2).map(|_| " ").collect::<String>(),
        input,
        "    //////////////////////////////////////////////////////////////*/"
    );

    println!("{}", output); // Print the header to console.

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    ctx.set_contents(output).unwrap(); // Copy the header to clipboard.
    thread::sleep(time::Duration::from_millis(100)); 
}
