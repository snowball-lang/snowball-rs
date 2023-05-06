mod frontend;
use clap::Parser;
use frontend::lexer::Token;
use logos::Logos;

#[derive(Parser)]
struct Args {

}

fn main() {
    let lexer = Token::lexer("use Core::System fn main() Int{System::print(2.1) }");
    // TODO: show errors
    println!("{:?}", lexer.map(|e| e.unwrap()).collect::<Vec<Token>>());
}
