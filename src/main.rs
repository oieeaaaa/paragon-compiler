use std::{
    env,
    fs,
    io,
};

#[derive(Debug)]
enum TokenType {
    Keyword,
    Identifier,
    Literal,
    Operator,
    Separator,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    value: String,
}

fn main() -> io::Result<()> {
    // let mut path: &str = "something";
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a path");
        std::process::exit(512); // FF: exit code will be 0 - because 1 byte
    }

    let contents = fs::read_to_string(&args[1])?;

    let mut tokens: Vec<Token> = Vec::new();
    let mut count = 0;
    let mut i = 0;

    while i < contents.len() {
        let current = contents.chars().nth(i).unwrap();

        if current.is_ascii_whitespace() {
            i += 1;
            continue;
        }

        // IDENTIFIERS
        if current.is_alphabetic() {
            let mut value = String::new();

            while contents.chars().nth(i).unwrap().is_alphabetic() {
                value.push(contents.chars().nth(i).unwrap());

                i += 1;
            }

            tokens.push(Token {
                token_type: TokenType::Identifier,
                value,
            });

            count += 1;
        }

        // SEPARATORS
        if current == '(' || current == ')' || current == ';' {
            tokens.push(Token {
                token_type: TokenType::Separator,
                value: current.to_string(),
            });

            i += 1;
            count += 1;
        }

        // LITERAL
        if current == '"' {
            let mut value = String::new();

            i += 1;

            value.push('"');

            while contents.chars().nth(i).unwrap() != '"' {
                value.push(contents.chars().nth(i).unwrap());
                i += 1;
            }

            value.push('"');

            tokens.push(Token {
                token_type: TokenType::Literal,
                value,
            });

            i += 1;
            count += 1;
        }
    }

    let code = convert_to_rust(&tokens);

    fs::write("output.rs", code)?;

    Ok(())
}

fn convert_to_rust(tokens: &Vec<Token>) -> String {
    let mut code = String::from("fn main() {\n");

    for token in tokens {
        match token.token_type {
            TokenType::Identifier => {
                if token.value == "ipasingang" {
                    code.push_str("println!");
                }
            }
            TokenType::Separator => code.push_str(&token.value),
            TokenType::Literal => code.push_str(&token.value),
            _ => {
                // handle other token types
            }
        }
    }

    code.push_str("\n}");

    return code;
}
