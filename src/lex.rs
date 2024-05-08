use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
pub enum Token {
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Multiply,

    #[token("/")]
    Divide,

    #[token("^")]
    Power,

    #[token("!")]
    Factorial,

    #[token("sin")]
    Sin,

    #[token("cos")]
    Cos,

    #[token("tan")]
    Tan,

    #[token("exp")]
    Exp,

    #[token("ln")]
    Log,

    #[token("(")]
    LParens,

    #[token(")")]
    RParens,

    // Regex from the Logos tutorial book
    // https://logos.maciej.codes/examples/json.html
    #[regex(r"(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Number(f64),
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    #[test]
    fn parse_line() {
        let mut lex = Token::lexer("5^ln(3/4* cos(9- -1))");

        assert_eq!(lex.next(), Some(Ok(Number(5f64))));
        assert_eq!(lex.next(), Some(Ok(Power)));
        assert_eq!(lex.next(), Some(Ok(Log)));
        assert_eq!(lex.next(), Some(Ok(LParens)));
        assert_eq!(lex.next(), Some(Ok(Number(3f64))));
        assert_eq!(lex.next(), Some(Ok(Divide)));
        assert_eq!(lex.next(), Some(Ok(Number(4f64))));
        assert_eq!(lex.next(), Some(Ok(Multiply)));
        assert_eq!(lex.next(), Some(Ok(Cos)));
        assert_eq!(lex.next(), Some(Ok(LParens)));
        assert_eq!(lex.next(), Some(Ok(Number(9f64))));
        assert_eq!(lex.next(), Some(Ok(Minus)));
        assert_eq!(lex.next(), Some(Ok(Minus)));
        assert_eq!(lex.next(), Some(Ok(Number(1f64))));
        assert_eq!(lex.next(), Some(Ok(RParens)));
        assert_eq!(lex.next(), Some(Ok(RParens)));
    }
}