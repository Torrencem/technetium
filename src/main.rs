#![allow(unused)]
#![allow(non_upper_case_globals)]
#![feature(fn_traits)]
#[macro_use] extern crate lalrpop_util;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;

lalrpop_mod!(pub script);

pub mod ast; 
pub mod lexer;
pub mod core;
pub mod bytecode;
pub mod builtins;
pub mod compile;
pub mod standard;
pub mod logging;
use compile::*;
use lexer::Lexer;
use standard::STANDARD_CONTEXT_ID;
use std::sync::Arc;
use std::process::exit;
use std::collections::HashMap;

extern crate clap;
use clap::{Arg, App, SubCommand};
use std::io::{self, Read};

use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::term;

use log::Level;

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
             .help("Set logging verbosity level")
             .multiple(true))
        .get_matches();

    let log_level = match matches.occurrences_of("verbose") {
        0 => Level::Error,
        1 => Level::Warn,
        2 => Level::Info,
        3 => Level::Debug,
        _ => Level::Trace,
    };

    logging::init(log_level);

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

    trace!("Beginning parsing stage");

    let ast = script::ProgramParser::new().parse(lexer).expect("temp1");

    trace!("Completed parsing. AST: {:?}", ast);

    let mut manager = CompileManager::new();

    trace!("Compiling code into bytecode representation");

    let code = manager.compile_statement_list(&ast);

    let compile_context = manager.context_stack.pop().unwrap();

    let code = code.unwrap_or_else(|e| {
        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();

        let diagnostic = e.as_diagnostic(file_id);

        term::emit(&mut writer.lock(), &config, &files, &diagnostic).expect("Error writing error message");
        exit(1)
    });

    trace!("Bytecode succesfully compiled. Creating Frame");

    let global_context = bytecode::GlobalContext { constant_descriptors: compile_context.constant_descriptors, debug_descriptors: compile_context.debug_span_descriptors };

    let mut locals = HashMap::new();

    let mut frame = bytecode::Frame::new(&code, &mut locals, Arc::new(global_context), HashMap::new(), STANDARD_CONTEXT_ID + 1);

    trace!("Running frame");

    let computation = frame.run();
    
    let computation = computation.unwrap_or_else(|e| {
        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();

        let diagnostic = e.as_diagnostic(file_id);

        term::emit(&mut writer.lock(), &config, &files, &diagnostic).expect("Error writing error message");
        exit(1)
    });

    trace!("Run completed successfully");

    debug!("Computation returned: {:?}", computation);
}

