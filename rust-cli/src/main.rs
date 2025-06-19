use colored::*;
use figlet_rs::FIGfont;

fn main() {
    // 1. Create the ASCII Art Header
    let font = FIGfont::standard().unwrap(); // Or use another font like "slant"
    let figure = font.convert("My App");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
    println!("{}", "A super fast CLI tool".cyan());


    // 2. Print the "Hello World" message with rich text
    println!("{}", "Hello, World!".yellow());

    // 3. Add a hint for future expansion
    println!(
        "\n{}",
        "Next steps: Try adding a new command!".green().underline()
    );
}