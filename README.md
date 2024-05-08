# ast-calc

This is yet another calculator/expression parser written in Rust. The repo is primarily meant to be used for educational purposes. In particular, I make no claims that this implementation is efficient or even completely correct, so do not use for anything serious. I was curious to learn more about parsing in general and thought this would be a fun exercise. The distinguishing features of this repo are:
* It uses the [logos](https://crates.io/crates/logos) crate to handle lexing of user inputs (in the `lex.rs` module). Wanting to learn how to use this crate was the motivation for this repo.
* It reimplements [this](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html) Pratt parser algorithm almost verbatim, with some tweaks made to better fit my implementation choices with logos (in the `parse.rs` module).
* It goes further than the above tutorial in computing an [AST](https://en.wikipedia.org/wiki/Abstract_syntax_tree) structure by hand before evaluating the expression (in the `ast.rs` module). This tree can be visualized by the user for each of their inputs by toggling a CLI flag. The default hierarchical view is modeled after [this](https://stackoverflow.com/a/51730733/22391278) C++ solution for printing binary trees. The tree view is experimental and will break for very complex expressions due to missing handling spacing based on the global structure. Use the former if accuracy of the AST is paramount, though the tree view is a little prettier when it works.

## Install

If you want to try out the program, you can install it by cloning and using `cargo`, e.g.
```
git clone https://github.com/rickymagner/ast-calc
cd ast-calc
cargo install --path .
```

## Usage

You can start the calculator session by running:
* `ast-calc` for the normal calculator mode;
* `ast-calc -a` for the `ast-mode` which will print an AST of your expression before evaluating it. 
* `ast-calc -a -v tree` for the *experimental* `tree` mode visualizer, as opposed to the default `hierarchy` mode.

An example of running in `ast-mode` using `ast-calc -a`:
```
Type exit or quit to stop program!
>>> sin(4*8-2) + 3/4
Here is the AST for your expression:
└── +
    ├── sin
    │   └── -
    │       ├── *
    │       │   ├── 4
    │       │   └── 8
    │       └── 2
    └── /
        ├── 3
        └── 4
The expression evaluates to: -0.23803162409286183
```

An example of running in `ast-mode` using `ast-calc -a -v tree`:
```
Type exit or quit to stop program!
>>> sin(4*8-2) + 3/4
Here is the AST for your expression:
                            +
                           / \
                        sin       /
                         |       / \
                         -     3     4
                        / \
                   *        2
                  / \
                4     8


The expression evaluates to: -0.23803162409286183
```
It's possible some complicated expressions might get a little in this view mode, but generally it should get the right shape. 

## Valid Commands

The following symbols are supported and should work the way you'd expect:
* `+`, `-` (subtract and negate), `*` (multiplication), `/`, `^` (exponents)
* `!` for factorial
* `sin`, `cos`, `tan`, `exp`, `ln`
* `(` and `)` for prioritizing subexpressions
* Any `f64` number

Note the parser is not currently hooked up for careful error handling, so submitting invalid syntax like `sin)4//3`, etc. would lead to a panic and crash the program. Same goes for doing mathematically illegal things like factorials of anything but nonnegative integers, taking the logarithm of a negative number, etc.

## Developer's Notes

If you find any bugs I'd be happy to reply to an issue, or review a PR if you'd like to add some functionality.