use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use logos::Lexer;
use crate::lex::Token;

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum BinOp {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            BinOp::Plus => "+",
            BinOp::Minus => "-",
            BinOp::Multiply => "*",
            BinOp::Divide => "/",
            BinOp::Power => "^"
        };

        write!(f, "{}", s)
    }
}

impl From<Token> for BinOp {
    fn from(value: Token) -> Self {
        match value {
            Token::Plus => Self::Plus,
            Token::Minus => Self::Minus,
            Token::Multiply => Self::Multiply,
            Token::Divide => Self::Divide,
            Token::Power => Self::Power,
            e => panic!("Cannot convert {:?} to binary operator", e),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum UnOp {
    Negative,
    Sin,
    Cos,
    Tan,
    Exp,
    Log,
    Factorial
}

impl Display for UnOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            UnOp::Negative => "-",
            UnOp::Sin => "sin",
            UnOp::Cos => "cos",
            UnOp::Tan => "tan",
            UnOp::Exp => "exp",
            UnOp::Log => "log",
            UnOp::Factorial => "!"
        };

        write!(f, "{}", s)
    }
}

impl From<Token> for UnOp {
    fn from(value: Token) -> Self {
        match value {
            Token::Minus => Self::Negative,
            Token::Sin => Self::Sin,
            Token::Cos => Self::Cos,
            Token::Tan => Self::Tan,
            Token::Exp => Self::Exp,
            Token::Log => Self::Log,
            Token::Factorial => Self::Factorial,
            e => panic!("Cannot convert {:?} to unary operator", e)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Expr {
    BinaryOp(BinOp, Box<Expr>, Box<Expr>),
    UnaryOp(UnOp, Box<Expr>),
    Number(f64),
    Eof
}

impl Expr {
    // Get number of cells needed to display the corresponding AST
    pub(crate) fn get_width(&self) -> usize {
        match self {
            Expr::BinaryOp(_, e1, e2) => e1.get_width() + e2.get_width() + 3,
            Expr::UnaryOp(_, e) => e.get_width(),
            Expr::Number(_) => 1usize,
            Expr::Eof => 0usize
        }
    }

    // Get size of cell for AST printing based on max size of number in it
    pub(crate) fn get_max_len(&self) -> usize {
        match self {
            Expr::BinaryOp(_, e1, e2) => std::cmp::max(e1.get_max_len(), e2.get_max_len()),
            Expr::UnaryOp(_, e) => e.get_max_len(),
            Expr::Number(n) => std::cmp::max(n.to_string().len(), 3usize),
            Expr::Eof => 0usize
        }
    }

    /// Print the hierarchical representation of Expr
    /// Adapted from: https://stackoverflow.com/a/51730733/22391278
    pub(crate) fn print_hierarchy(&self, prefix: &str, is_left: bool) {
        let second_part = if is_left {
            "├── "
        } else {
            "└── "
        };
        let new_prefix = prefix.to_owned() + if is_left {
            "│   "
        } else {
            "    "
        };
        match self {
            Expr::BinaryOp(op, e1, e2) => {
                println!("{}{}{}", prefix, second_part, op);
                e1.print_hierarchy(&new_prefix, true);
                e2.print_hierarchy(&new_prefix, false);
            },
            Expr::UnaryOp(op, e) => {
                println!("{}{}{}", prefix, second_part, op);
                e.print_hierarchy(&new_prefix, false);
            },
            Expr::Number(n) => {
                println!("{}{}{}", prefix, second_part, n);
            },
            Expr::Eof => {}
        }
    }

    pub(crate) fn eval(&self) -> f64 {
        match self {
            Expr::BinaryOp(op, e1, e2) => {
                match op {
                    BinOp::Plus => {e1.eval() + e2.eval()},
                    BinOp::Minus => {e1.eval() - e2.eval()},
                    BinOp::Multiply => {e1.eval() * e2.eval()},
                    BinOp::Divide => {e1.eval() / e2.eval()},
                    BinOp::Power => {e1.eval().powf(e2.eval())}
                }
            },
            Expr::UnaryOp(op, e) => {
                match op {
                    UnOp::Negative => {-e.eval()},
                    UnOp::Sin => {e.eval().sin()},
                    UnOp::Cos => {e.eval().cos()},
                    UnOp::Tan => {e.eval().tan()},
                    UnOp::Exp => {e.eval().exp()},
                    UnOp::Log => {e.eval().ln()}
                    UnOp::Factorial => {
                        let val = e.eval();
                        if val.fract() == 0.0 {
                            let int_val = val as u64;
                            (1..=int_val).product::<u64>() as f64
                        } else {
                            panic!("Cannot evaluate factorial on decimal.")
                        }
                    }
                }
            },
            Expr::Number(n) => *n,
            Expr::Eof => panic!("Should not eval Eof expr!")
        }
    }
}

fn infix_prec(op: &Token) -> Option<(u8, u8)> {
    let prec = match op {
        Token::Plus => (1, 2),
        Token::Minus => (1, 2),
        Token::Multiply => (3, 4),
        Token::Divide => (3, 4),
        Token::Power => (5, 6),
        _ => return None
    };

    Some(prec)
}

fn prefix_prec(op: &Token) -> ((), u8) {
    match op {
        Token::Minus => ((), 8),
        Token::Sin => ((), 8),
        Token::Cos => ((), 8),
        Token::Tan => ((), 8),
        Token::Exp => ((), 8),
        Token::Log => ((), 8),
        _ => panic!("Bad op for prefix: {:?}", op)
    }
}

fn postfix_prec(op: &Token) -> Option<(u8, ())> {
    let prec = match op {
        Token::Factorial => (9, ()),
        _ => return None,
    };
    Some(prec)
}

/// Based off of this blog post: https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
pub(crate) fn expr_prec(lexer: &mut Peekable<Lexer<Token>>, min_prec: u8) -> Expr {
    // Check if lexer reached end of input
    let lhs_read = if let Some(t) = lexer.next() {
        t.expect("Could not read token")
    } else {
        return Expr::Eof
    };

    // Otherwise check the next token type
    let mut lhs = match lhs_read {
        Token::Number(n) => Expr::Number(n),
        Token::LParens => {
            let lhs = expr_prec(lexer, 0);
            assert_eq!(lexer.next(), Some(Ok(Token::RParens)));
            lhs
        },
        t => {
            let ((), r_prec) = prefix_prec(&t);
            let rhs = expr_prec(lexer, r_prec);
            Expr::UnaryOp(UnOp::from(t), Box::new(rhs))
        }
    };

    loop {
        let op = match lexer.peek() {
            Some(Ok(t)) => t,
            Some(Err(e)) => panic!("Could not parse token with error: {:?}", e),
            None => break,
        };

        if let Some((l_bp, ())) = postfix_prec(op) {
            if l_bp < min_prec {
                break;
            }

            let op = lexer.next().unwrap().unwrap();
            lhs = Expr::UnaryOp(UnOp::from(op), Box::new(lhs));
            continue;
        }

        if let Some((l_prec, r_prec)) = infix_prec(op) {
            if l_prec < min_prec {
                break;
            }

            let op = lexer.next().unwrap().unwrap();
            let rhs = expr_prec(lexer, r_prec);

            lhs = Expr::BinaryOp(BinOp::from(op), Box::new(lhs), Box::new(rhs));
            continue;
        }

        break;
    }

    lhs
}

#[cfg(test)]
mod tests {
    use logos::Logos;
    use super::*;

    #[test]
    fn parse_expr1() {
        let lex = Token::lexer("sin(3--1)");
        let test_e = expr_prec(&mut lex.peekable(), 0);

        let neg = Box::new(Expr::UnaryOp(UnOp::Negative, Box::new(Expr::Number(1f64))));
        let diff = Box::new(Expr::BinaryOp(BinOp::Minus, Box::new(Expr::Number(3f64)), neg));
        let expect_e = Expr::UnaryOp(UnOp::Sin, diff);
        assert_eq!(test_e, expect_e);
    }

    #[test]
    fn parse_expr2() {
        let lex = Token::lexer("1+2/3-4/5");
        let test_e = expr_prec(&mut lex.peekable(), 0);

        let frac1 = Box::new(Expr::BinaryOp(BinOp::Divide, Box::new(Expr::Number(2f64)), Box::new(Expr::Number(3f64))));
        let frac2 = Box::new(Expr::BinaryOp(BinOp::Divide, Box::new(Expr::Number(4f64)), Box::new(Expr::Number(5f64))));
        let sum = Box::new(Expr::BinaryOp(BinOp::Plus, Box::new(Expr::Number(1f64)), frac1));
        let expect_e = Expr::BinaryOp(BinOp::Minus, sum, frac2);

        assert_eq!(test_e, expect_e);
    }
}