use logos::{Lexer, Logos};

aoc2024::aoc!();

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

#[cfg(test)]
mod day03 {

    #[test]
    fn part1() {
        const EXAMPLE: &str =
            r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

        assert_eq!(super::part1(EXAMPLE).to_string(), "161");
    }

    #[test]
    fn part2() {
        const EXAMPLE: &str =
            r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
        assert_eq!(super::part2(EXAMPLE).to_string(), "48");
    }
}
