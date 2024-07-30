use logos::Logos;
use std::io::{self, BufRead, Write};
use std::{fs::File, io::Read};

mod fmt;
mod parser;
mod stmt;
mod token;

use fmt::Formatter;
use parser::Parser;
use token::Token;

fn main() -> std::io::Result<()> {
    // let mut file = File::open("edo_exchange.prg")?;
    // let mut file = File::open("ur_lic.prg")?;
    let mut file = File::open("test.prg")?;
    let mut source = Vec::new();
    file.read_to_end(&mut source)?;

    let source = String::from_utf8(source).unwrap();

    let lex = Token::lexer(&source);

    let mut parser = Parser::new(lex);
    let stmt = parser.parse();

    // let new_file = File::create("new.prg");
    // let mut writer = io::BufWriter::new(new_file.unwrap());

    let mut formatter = Formatter::new(3);

    for st in stmt {
        print!("{}", st.fmt(&mut formatter));
    }

    Ok(())
}
