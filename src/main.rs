use std::fmt::{Display, Formatter};
use std::io;
use std::io::{BufRead, Write};
use std::process::exit;
use ast_calc::ast::Ast;
use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Debug, Clone, Copy)]
enum AstView {
    Hierarchy,
    Tree,
}

impl Display for AstView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match &self {
            AstView::Hierarchy => "hierarchy",
            AstView::Tree => "tree",
        };
        write!(f, "{}", string)
    }
}

#[derive(Parser, Debug)]
struct Args {
    /// Toggle printing AST for each submitted expression before evaluating
    #[arg(short, long, default_value_t=false)]
    ast_mode: bool,

    /// Choose the printing mode when viewing ASTs
    #[arg(short='v', long, default_value_t=AstView::Hierarchy, requires="ast_mode")]
    ast_view: AstView
}

fn main() {
    let args = Args::parse();

    println!("Type exit or quit to stop the program!");

    let stdin = io::stdin();
    print!(">>> ");
    let _ = io::stdout().flush();
    for line in stdin.lock().lines() {
        if let Ok(l) = line {
            if l == "exit" || l == "quit" || l == "q" {
                exit(0)
            } else {
                let ast = Ast::string_to_ast(&l);
                if args.ast_mode {
                    println!("Here is the AST for your expression:");
                    match args.ast_view {
                        AstView::Hierarchy => ast.print_hierarchy(),
                        AstView::Tree => println!("{}", ast)
                    }
                }
                println!("The expression evaluates to: {}", ast.eval());
            }
        } else {
            println!("Cannot read line from stdin!");
        }
        print!(">>> ");
        let _ = io::stdout().flush();
    }
}

