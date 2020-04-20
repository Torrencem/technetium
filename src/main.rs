#![allow(unused)]
#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub script);

pub mod ast; 
pub mod lexer;
use lexer::Lexer;

fn main() {
    let input = "[1,(a,\"pizza\"), object.method(), a.attr, fib(10, s, i)]";
    let lexer = Lexer::new(input);

    let ast = script::ExprParser::new().parse(lexer);

    dbg!(ast);
}
