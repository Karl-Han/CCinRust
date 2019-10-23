mod lib;

use lib::Lexer;
use std::fs;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = fs::File::open("../hello.c")?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    let mut lex = Lexer::new(&content);

    match &lex.lex() {
        Ok(_) => {
            println!("Successfully lex");
        }

        Err(e) => {
            if lex.get_status() {
                println!("Finish");
            } else {
                println!("Error while lexing.\n{}", e);
            }
        }
    }

    for token in lex.into_iter() {
        println!("{}", token.to_string());
    }
    println!("Hello, world!");
    Ok(())
}
