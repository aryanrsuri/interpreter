mod lexer;

fn main() {
    println!(">> Enter Statement");
    std::io::stdin().lines().for_each(|line| {
        if let Ok(line) = line {
            let mut tokeniser = lexer::Lexer::new(line);
            while let Some(token) = tokeniser.consume() {
                println!(">> {:?}", token);
                if let lexer::Token::Eof = token {
                    break;
                }
            }
        }
    })
}
