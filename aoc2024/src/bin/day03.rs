use logos::{Lexer, Logos};

fn part1(input: &str) -> impl std::fmt::Display {
    let mut total = 0;
    let mut lexer = Token::lexer(input);
    while let Some(token) = lexer.next() {
        if !matches!(token, Ok(Token::Mul)) {
            continue;
        }

        let Some((a, b)) = take_mul(&mut lexer) else {
            continue;
        };

        total += a * b;
    }

    total
}

fn part2(input: &str) -> impl std::fmt::Display {
    let mut total = 0;
    let mut enable = true;
    let mut lexer = Token::lexer(input);
    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::Mul) => {
                let Some((a, b)) = take_mul(&mut lexer) else {
                    continue;
                };

                if enable {
                    total += a * b;
                }
            }
            Ok(Token::Do) => {
                enable = true;
                continue;
            }
            Ok(Token::Dont) => {
                enable = false;
                continue;
            }
            _ => continue,
        }
    }

    total
}

#[derive(Debug, Logos)]
enum Token {
    #[token("mul")]
    Mul,

    #[token("(")]
    Open,

    #[token(")")]
    Close,

    #[token(",")]
    Comma,

    #[token("do()")]
    Do,

    #[token("don't()", priority = 10)]
    Dont,

    #[regex("[0-9]+", |lexer| lexer.slice().parse::<usize>().unwrap())]
    Number(usize),
}

fn take_mul(tokens: &mut Lexer<Token>) -> Option<(usize, usize)> {
    if !matches!(tokens.next(), Some(Ok(Token::Open))) {
        return None;
    }

    let Some(Ok(Token::Number(lval))) = tokens.next() else {
        return None;
    };

    if !matches!(tokens.next(), Some(Ok(Token::Comma))) {
        return None;
    }

    let Some(Ok(Token::Number(rval))) = tokens.next() else {
        return None;
    };

    if !matches!(tokens.next(), Some(Ok(Token::Close))) {
        return None;
    }

    Some((lval, rval))
}

aoc::setup! {
    day03;
    part1 == 161,
    "day03-part2.in": part2 == 48
}
