use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use logos::Logos;
use crate::lex::Token;
use crate::parse::{Expr, expr_prec};

pub struct Ast {
    expr: Expr
}

impl Ast {
    fn new(expr: Expr) -> Self {
        Self {
            expr
        }
    }

    pub fn eval(&self) -> f64 {
        self.expr.eval()
    }

    pub fn string_to_ast(s: &str) -> Self {
        let lex = Token::lexer(s);
        let expr = expr_prec(&mut lex.peekable(), 0);
        Ast::new(expr)
    }
}

#[derive(Debug, Clone, Copy)]
enum Align {
    Left,
    Right
}

#[derive(Debug, Clone)]
struct PositionedExpr<'a> {
    expr: &'a Expr,
    pos: usize,
    align: Align
}

impl<'a> PositionedExpr<'a> {
    fn new(expr: &'a Expr, pos: usize, align: Align) -> Self {
        Self {
            expr,
            pos,
            align
        }
    }
}

// Helper method for creating a padded cell with a string at the center
fn pad_center(s: String, total_width: usize, align: Align) -> String {
    if s.len() >= total_width {
        s
    } else {
        let diff = total_width - s.len();
        if diff % 2 == 0 {
            let padding = " ".repeat(diff/2);
            format!("{}{}{}", padding, s, padding)
        } else {
            let short_padding = " ".repeat(diff/2);
            let long_padding = " ".repeat((diff+1)/2);
            match align {
                Align::Left => format!("{}{}{}", short_padding, s, long_padding),
                Align::Right => format!("{}{}{}", long_padding, s, short_padding)
            }
        }
    }
}

impl Ast {
    pub fn print_hierarchy(&self) {
        self.expr.print_hierarchy("", false);
    }
}

impl Display for Ast {
    /// Try to print the AST for the expression like so:
    /// Split Ops into cells of a fixed length
    /// Generate edges below them in cells of same length
    /// Setup next line for printing using the Exprs inside the given Expr
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut width = self.expr.get_width();

        // Ensure width is odd so root can start at middle
        width = if width % 2 == 0 {
            width + 1
        } else {
            width
        };

        let mut cell_length = self.expr.get_max_len();
        // Ensure odd length so can have | in middle
        cell_length = if cell_length % 2 == 0 {
            cell_length + 1
        } else {
            cell_length
        };
        let cell_minus_2 = cell_length - 2;

        let mut current_row: VecDeque<PositionedExpr> = VecDeque::from([PositionedExpr::new(&self.expr, (width+1)/2, Align::Left)]);
        let mut next_row: VecDeque<PositionedExpr> = VecDeque::new();
        loop {
            let mut edges_vec = vec![" ".repeat(cell_length); width];
            let mut nodes_vec = vec![" ".repeat(cell_length); width];
            while !current_row.is_empty() {
                let next = current_row.pop_front().unwrap();
                match &next.expr {
                    Expr::BinaryOp(op, e1, e2) => {
                        nodes_vec[next.pos] = pad_center(op.to_string(), cell_length, next.align);
                        edges_vec[next.pos] = format!("/{:cell_minus_2$}\\", " ");
                        let left_diff = match **e1 {
                            Expr::BinaryOp(_, _, _) => 2usize,
                            _ => 1usize,
                        };
                        let right_diff = match **e2 {
                            Expr::BinaryOp(_, _, _) => 2usize,
                            _ => 1usize,
                        };
                        next_row.push_back(PositionedExpr::new(e1, next.pos-left_diff, Align::Left));
                        next_row.push_back(PositionedExpr::new(e2, next.pos+right_diff, Align::Right));
                    },
                    Expr::UnaryOp(op, e) => {
                        nodes_vec[next.pos] = pad_center(op.to_string(), cell_length, next.align);
                        edges_vec[next.pos] = pad_center("|".to_owned(), cell_length, next.align);
                        next_row.push_back(PositionedExpr::new(e, next.pos, Align::Left));
                    },
                    Expr::Number(n) => {
                        nodes_vec[next.pos] = pad_center(n.to_string(), cell_length, next.align);
                    },
                    Expr::Eof => {}
                }
            }

            // Print the collected nodes and edges for this row
            let row_nodes = nodes_vec.join("");
            let row_edges = edges_vec.join("");
            if !row_nodes.is_empty() {
                writeln!(f, "{}", row_nodes)?;
                writeln!(f, "{}", row_edges)?;
            }

            if !next_row.is_empty() {
                // Next row becomes current row and clear next row for new parsing
                current_row = next_row.clone();
                next_row.clear();
            } else {
                // Loop completes when next row is empty
                break;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc1() {
        let ast = Ast::string_to_ast("sin(4) + exp(3 - 1)^3");
        assert_eq!(ast.eval(), 402.67199099742726)
    }

    #[test]
    fn test_calc2() {
        let ast = Ast::string_to_ast("-2 + 4 * -(5^3 + 7 * 3!)");
        assert_eq!(ast.eval(), -670f64)
    }

    #[test]
    fn test_calc3() {
        let ast = Ast::string_to_ast("sin(3.14159) + cos(3.14159) + exp(0)^2 - ln(1)/2");
        assert_eq!(ast.eval(), 0.0000026535933140836576)
    }

    #[test]
    fn test_calc4() {
        let ast = Ast::string_to_ast("tan(-4--4) / ln(4)");
        assert_eq!(ast.eval(), 0f64);
    }

    #[test]
    fn test_calc5() {
        let ast = Ast::string_to_ast("ln(exp(-4/5))");
        assert_eq!(ast.eval(), -0.8);
    }
}