#![allow(unused)]
#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub script);

pub mod ast; 
pub mod lexer;
use lexer::Lexer;

fn main() {
    let input =
r#"
if check("yourself") {
    print("Goodbye!")
} else {
    check(the_weather)
    if its_sunny_outside {
        eat(lots_of_food1)
    }
    case my("name", 2) of {
        3 => no,
        4 => "yes",
    }
}
"#;
    let lexer = Lexer::new(input);

    let ast = script::ProgramParser::new().parse(lexer);

    dbg!(ast);
}

fn main_old() {
    let input = "[1,(a,\"pizza\"), object.method(), a.attr, fib(10, s, i)]";
    let lexer = Lexer::new(input);

    let ast = script::ExprParser::new().parse(lexer);

    dbg!(ast);
}
