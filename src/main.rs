#![allow(unused)]
#![feature(fn_traits)]
#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub script);

pub mod ast; 
pub mod lexer;
pub mod core;
pub mod bytecode;
pub mod builtins;
pub mod compile;
use compile::*;
use lexer::Lexer;
use std::sync::Arc;

fn main() {
    let input =
r#"
func incr(x) {
    return x + 1
}
func hello(f, b) {
    return f(b)
}
x = 10
if x + 100 < 15 {
    return [1, 2, 3, 4, "no", 100].length()
} else {
    return hello(incr, 20)
}
"#;
//     let input =
// r#"
// sum = 0
// x = 1
// sum = sum + x
// x = x + 1
// return x
// "#;
    let input =
r#"
func fib(n) {
    a = 1
    b = 1
    while n > 0 {
        tmp = b
        b = a + b
        a = tmp
        n = n - 1
    }
    return b
}
return fib(40)
"#;
    let input =
r#"
result = 0
for i in [1, 2, 3, 2] {
    result = result + i
}

return result
"#;
    let lexer = Lexer::new(input);

    let ast = script::ProgramParser::new().parse(lexer).expect("temp1");

    dbg!(&ast);

    let mut compile_context = CompileContext::new();

    let code = ast.compile(&mut compile_context).expect("temp2");

    let global_context = bytecode::GlobalContext { constant_descriptors: compile_context.constant_descriptors };
    
    let mut frame = bytecode::Frame::new(&code, Arc::new(global_context));

    let computation = frame.run();

    dbg!(computation);
}

// fn main_old() {
//     let input = "[1,(a,\"pizza\"), object.method(), a.attr, fib(10, s, i)]";
//     let lexer = Lexer::new(input);
//
//     let ast = script::ExprParser::new().parse(lexer);
//
//     dbg!(ast);
// }
