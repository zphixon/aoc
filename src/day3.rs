use logos::Logos;

pub fn run(example: bool) {
    let data1 = if example {
        include_str!("../data/example/day3.1.txt")
    } else {
        include_str!("../data/day3.1.txt")
    };

    tracing::info!("day 3 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 3 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(data1),
    );


    let data2 = if example {
        include_str!("../data/example/day3.2.txt")
    } else {
        include_str!("../data/day3.1.txt")
    };

    tracing::info!("day 3 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 3 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(data2),
    );
}

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(skip "")]
enum Token {
    #[token("mul")]
    Mul,
    #[token("do")]
    Do,
    #[token("don't")]
    Dont,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token(",")]
    Comma,
    #[regex("[0-9]+")]
    Number,
    #[regex(r".|\s", priority = 0)]
    Other,
}

fn do_the_thing(data: &str, toggle_stuff: bool) -> u64 {
    let mut lex = Token::lexer(data);

    #[derive(Clone, Copy, Debug)]
    enum Want {
        Keyword,

        LeftParenDo,
        RightParenDo,

        LeftParenDont,
        RightParenDont,

        LeftParenMul,
        FirstNumber,
        Comma,
        SecondNumber,
        RightParenMul,
    }

    let mut want = Want::Keyword;

    let mut do_sum = true;
    let mut sum = 0;
    let mut first = None;
    let mut second = None;

    while let Some(token) = lex.next() {
        let Ok(token) = token else {
            tracing::error!("invalid token {:?}", lex.slice());
            return 0;
        };

        if toggle_stuff {
            if token == Token::Other {
                first = None;
                second = None;
                want = Want::Keyword;
                continue;
            }
        } else {
            if matches!(token, Token::Other | Token::Do | Token::Dont) {
                first = None;
                second = None;
                want = Want::Keyword;
                continue;
            }
        }

        tracing::trace!("want={:?} token={:?} {:?}", want, token, lex.slice());

        match (want, token) {
            (Want::Keyword, Token::Do) => {
                want = Want::LeftParenDo;
            }
            (Want::LeftParenDo, Token::LeftParen) => {
                want = Want::RightParenDo;
            }
            (Want::RightParenDo, Token::RightParen) => {
                tracing::debug!("turn on sum");
                do_sum = true;
                want = Want::Keyword;
            }

            (Want::Keyword, Token::Dont) => {
                want = Want::LeftParenDont;
            }
            (Want::LeftParenDont, Token::LeftParen) => {
                want = Want::RightParenDont;
            }
            (Want::RightParenDont, Token::RightParen) => {
                tracing::debug!("turn off sum");
                do_sum = false;
                want = Want::Keyword;
            }

            (Want::Keyword, Token::Mul) => {
                want = Want::LeftParenMul;
            }
            (Want::LeftParenMul, Token::LeftParen) => {
                want = Want::FirstNumber;
            }

            (Want::FirstNumber, Token::Number) => {
                let Ok(first_num) = lex.slice().parse::<u64>() else {
                    first = None;
                    second = None;
                    want = Want::Keyword;
                    tracing::trace!("first number not a number: {:?}", lex.slice());
                    continue;
                };
                first = Some(first_num);
                want = Want::Comma;
            }

            (Want::Comma, Token::Comma) => {
                want = Want::SecondNumber;
            }

            (Want::SecondNumber, Token::Number) => {
                let Ok(second_num) = lex.slice().parse::<u64>() else {
                    first = None;
                    second = None;
                    want = Want::Keyword;
                    tracing::trace!("second number not a number: {:?}", lex.slice());
                    continue;
                };
                second = Some(second_num);

                want = Want::RightParenMul;
            }

            (Want::RightParenMul, Token::RightParen) => {
                let first = first.expect("first number exists");
                let second = second.expect("second number exists");

                if do_sum {
                    tracing::debug!("sum += {} * {}", first, second);
                    sum += first * second;
                } else {
                    tracing::debug!("not doing sum");
                }

                want = Want::Keyword;
            }

            _ => {
                first = None;
                second = None;
                want = Want::Keyword;
                tracing::trace!("unexpected token {:?}, wanted {:?}", token, want);
            }
        }
    }

    sum
}

fn part1(data: &str) -> u64 {
    do_the_thing(data, false)
}

fn part2(data: &str) -> u64 {
    do_the_thing(data, true)
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day3.1.txt")), 182780583);
}

#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day3.1.txt")), 90772405);
}
