use std::{
    fs::File,
    io::{Read, Write, stdin, stdout},
    path::Path,
};

use rlox::errors::LoxError;
use rlox::parser::Parser;
use rlox::{expression::parenthesize, scanner::Scanner};

fn run(source: String) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    // for tok in &tokens{
    //     println!("{:?}",tok);
    // }
    let mut parser = Parser::new(tokens);

    let exp = parser.expression()?;
    println!("{:?}",parenthesize(&exp)) ;
    // println!("{:?}",exp);

    Ok(())
}

fn run_file(path: &Path) -> Result<(), LoxError> {
    let mut source = String::new();
    File::open(path)?.read_to_string(&mut source)?;

    run(source)?;

    Ok(())
}

fn run_prompt() -> Result<(), LoxError> {
    loop {
        print!("> ");
        stdout().flush()?;

        let mut line = String::new();

        if stdin().read_line(&mut line)? == 0 {
            break;
        }

        if line.trim() == ".exit" {
            break;
        }

        if let Err(error) = run(line) {
            eprintln!("{error}");
        }
    }

    Ok(())
}

fn main() -> Result<(), LoxError> {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => run_prompt()?,

        2 => run_file(Path::new(&args[1]))?,

        _ => {
            eprintln!("Usage: lox [script]");
        }
    }

    Ok(())
}

// fn main() {
//     // 123
//     let literal_123 = Box::new(Expr::Literal {
//         value: Literal::Number(123.0),
//     });

//     // (- 123)
//     let minus = Token::new(
//         MINUS,
//         "-".to_string(),
//         Literal::Nil,
//         0,
//     );

//     let unary = Box::new(Expr::Unary {
//         operator: minus,
//         right: literal_123,
//     });

//     // 45.67
//     let literal_45_67 = Box::new(Expr::Literal {
//         value: Literal::Number(45.67),
//     });

//     // (group 45.67)
//     let grouping = Box::new(Expr::Grouping {
//         expression: literal_45_67,
//     });

//     // (* (- 123) (group 45.67))
//     let multiply = Token::new(
//         rlox::token_type::TokenType::STAR,
//         "*".to_string(),
//         rlox::token::Literal::Nil,
//         0,
//     );

//     let expr = Expr::Binary {
//         left: unary,
//         operator: multiply,
//         right: grouping,
//     };

//     println!("{}", RPN(&expr));
// }
