#![allow(unused)]
#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub script);

pub mod ast; 
pub mod lexer;
pub mod core;
use lexer::Lexer;

fn main() {
    let input =
r#"
x = 100 + 150
if x > 10 {
    print("x is big!")
} elif x > 100 {
    print("x is SUPER HUGE!!!")
}
mylist = [("a", b, 3, "abc" + "def", 10 / 5), [[1]]]
mylist.tell_a_story(arg1, arg2)
"#;
    let lexer = Lexer::new(input);

    let ast = script::ProgramParser::new().parse(lexer);

    dbg!(ast);

}

// fn main_old() {
//     let input = "[1,(a,\"pizza\"), object.method(), a.attr, fib(10, s, i)]";
//     let lexer = Lexer::new(input);
//
//     let ast = script::ExprParser::new().parse(lexer);
//
//     dbg!(ast);
// }
