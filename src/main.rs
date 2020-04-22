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
use std::process::exit;

extern crate clap;
use clap::{Arg, App, SubCommand};
use std::io::{self, Read};

use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::term;

fn main() {
    let matches = App::new("marsh")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("INPUT")
             .help("Run file as a script (if not given, will read from stdin)")
             .index(1))
        .arg(Arg::with_name("verbose")
             .short("v")
             .long("verbose")
             .help("Emit bytecode and debug information"))
        .get_matches();

    let mut files = SimpleFiles::new();

    let input: String = match matches.value_of("INPUT") {
        Some(file_name) => {
            std::fs::read_to_string(file_name).expect("Error reading file")
        },
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).expect("Error reading stdin");
            buffer
        },
    };

    let file_id = files.add(match matches.value_of("INPUT") {
        Some(file_name) => {
            file_name.clone()
        },
        None => {
            "<anonymous>"
        },
    },&input);

    let verbose = matches.is_present("verbose");

    let lexer = Lexer::new(input.as_ref());

    let ast = script::ProgramParser::new().parse(lexer).expect("temp1");

    if verbose {
        dbg!(&ast);
    }

    let mut compile_context = CompileContext::new();

    let code = ast.compile(&mut compile_context);
    
    let code = code.unwrap_or_else(|e| {
        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();

        let diagnostic = e.as_diagnostic(file_id);

        term::emit(&mut writer.lock(), &config, &files, &diagnostic).expect("Error writing error message");
        exit(1)
    });

    let global_context = bytecode::GlobalContext { constant_descriptors: compile_context.constant_descriptors, debug_descriptors: compile_context.debug_span_descriptors };

    let mut frame = bytecode::Frame::new(&code, Arc::new(global_context));

    let computation = frame.run();
    
    let computation = computation.unwrap_or_else(|e| {
        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();

        let diagnostic = e.as_diagnostic(file_id);

        term::emit(&mut writer.lock(), &config, &files, &diagnostic).expect("Error writing error message");
        exit(1)
    });

    dbg!(computation);
}

