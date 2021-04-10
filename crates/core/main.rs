#[macro_use]
extern crate log;

pub mod logging;
use compile::*;
use lexer::error::parse_error_to_diagnostic;
use lexer::Lexer;
use runtime;
use runtime::bytecode;
use runtime::standard::STANDARD_CONTEXT_ID;
use std::borrow::Cow;
use std::collections::HashMap;
use std::process::exit;
use std::rc::Rc;
use std::env;
use std::path::PathBuf;

extern crate clap;
use clap::{App, AppSettings, Arg};
use std::io::{self, Read};

use codespan::Files;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

use log::Level;

use std::fmt::Display;

fn fail<D: Display, S: Display>(message: S, e: D) -> ! {
    eprintln!("{}: {}", message, e);
    exit(1)
}

fn recursive_make_script(name: &str) -> io::Result<Option<String>> {
    let mut dir = env::current_dir()?;

    loop {
        let mut build_script = dir.clone();
        build_script.push(name);
        if build_script.exists() {
            runtime::INVOKE_ABSOLUTE_PARENT_DIR.set(dir.clone().canonicalize().unwrap()).unwrap();
            return Ok(Some(std::fs::read_to_string(build_script)?));
        }
        if !dir.pop() {
            return Ok(None);
        }
    }
}

/// The main application entry point
fn main() {
    let matches = App::new("technetium")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::TrailingVarArg)
        .arg(
            Arg::with_name("INPUT")
                .help("Run file as a script (if not given, or '-', will read from stdin). If INPUT is not given, and additional arguments must be passed to the script, use '--' before additional arguments")
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Set logging verbosity level")
                .multiple(true),
        )
        .arg(
            Arg::with_name("recursive")
                .short("r")
                .long("recursive")
                .help("Search current directory and parents for a make file (default name: 'make.tc') and run it.")
        )
        .arg(
            Arg::with_name("MAKE_FILE_NAME")
                .short("bf")
                .long("make_file_name")
                .help("Name of make file to search for. Requires '-r'. Can also be overridden using the environment variable 'TC_MAKE_FILE_NAME'.")
                .requires("recursive")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("COMMAND")
                .short("c")
                .long("command")
                .help("Run COMMAND as a technetium script and exit")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("args")
                .help("Arguments to pass to script")
                .multiple(true)
        )
        .arg(
            Arg::with_name("more_args")
                .help("Arguments to pass to script")
                .multiple(true)
                .last(true)
                .hidden(true)
        )
        .get_matches();

    let log_level = match matches.occurrences_of("verbose") {
        0 => Level::Error,
        1 => Level::Warn,
        2 => Level::Info,
        3 => Level::Debug,
        _ => Level::Trace,
    };

    logging::init(log_level).expect("Error initializing logging");

    let mut extra_args = vec![];

    if matches.is_present("recursive") {
        // If recursive, first argument should be passed
        // as extra_args, not as INPUT
        if let Some(first_arg) = matches.value_of("INPUT") {
            extra_args.push(first_arg.to_string());
        }
    }

    extra_args.append(
        &mut matches
            .values_of("args")
            .map(|val| val.map(|val| val.to_owned()).collect())
            .unwrap_or(vec![]),
    );

    extra_args.append(
        &mut matches
            .values_of("more_args")
            .map(|val| val.map(|val| val.to_owned()).collect())
            .unwrap_or(vec![]),
    );

    runtime::PARSED_CLARGS.set(extra_args).unwrap();

    let mut files: Files<Cow<'_, str>> = Files::new();

    let mut input: String = {
        if let Some(cmd) = matches.value_of("COMMAND") {
            runtime::INVOKE_ABSOLUTE_PARENT_DIR.set(env::current_dir().unwrap().canonicalize().unwrap()).unwrap();
            cmd.to_owned()
        } else if matches.is_present("recursive") {
            let make_script_name = match env::var("TC_MAKE_FILE_NAME") {
                Ok(variable) => variable,
                _ => matches
                            .value_of("MAKE_FILE_NAME")
                            .unwrap_or("make.tc")
                            .to_owned(),
            };
            
            recursive_make_script(&make_script_name)
                .unwrap_or_else(|e| fail(format!("No build script found ('{}') in current or any parent directory", make_script_name), e))
                .unwrap_or_else(|| {
                    eprintln!("No build script found ('{}') in current or any parent directory", make_script_name);
                    exit(1)
                })
        } else {
            match matches.value_of("INPUT") {
                None | Some("-") => {
                    runtime::INVOKE_ABSOLUTE_PARENT_DIR.set(env::current_dir().unwrap().canonicalize().unwrap()).unwrap();
                    let mut buffer = String::new();
                    io::stdin()
                        .read_to_string(&mut buffer)
                        .expect("Error reading stdin");
                    buffer
                }
                Some(file_name) => {
                    let path = PathBuf::from(file_name);
                    runtime::INVOKE_ABSOLUTE_PARENT_DIR.set(path.canonicalize().unwrap().parent().unwrap().to_path_buf()).unwrap();
                    std::fs::read_to_string(file_name).unwrap_or_else(|e| fail(format!("Error reading file '{}'", file_name), e))
                },
            }
        }
    };

    // Input should always end in a newline. This would be useful to add to the grammar,
    // so that things like "return 1; }" would compile, but that is harder to get working.
    if !input.ends_with("\n") {
        input.push('\n');
    }

    let file_id = files.add(
        match matches.value_of("INPUT") {
            Some(file_name) => file_name,
            None => "<anonymous>",
        },
        Cow::from(&input),
    );

    let err_writer = StandardStream::stderr(ColorChoice::Always);
    let err_config = codespan_reporting::term::Config::default();

    let lexer = Lexer::new(input.as_ref());

    trace!("Beginning parsing stage");

    let mut recoveries = vec![];

    let ast = script::ProgramParser::new()
        .parse(&mut recoveries, lexer)
        .unwrap_or_else(|e| {
            let diagnostic = parse_error_to_diagnostic(&e, file_id);

            term::emit(&mut err_writer.lock(), &err_config, &files, &diagnostic)
                .expect("Error writing error message");

            eprintln!("The above error could not be recovered from, and parsing stopped.");

            exit(1)
        });

    if recoveries.len() != 0 {
        for recovery in recoveries.iter() {
            let diagnostic = parse_error_to_diagnostic(&recovery.error, file_id);

            term::emit(&mut err_writer.lock(), &err_config, &files, &diagnostic)
                .expect("Error writing error message");
        }

        eprintln!(
            "Exiting without running due to previous {} parsing error{}",
            recoveries.len(),
            if recoveries.len() == 1 { "" } else { "s" }
        );

        exit(1)
    }

    trace!("Completed parsing. AST: {:?}", ast);

    let mut manager = CompileManager::new(file_id);

    trace!("Compiling code into bytecode representation");

    let code = manager.compile_statement_list(&ast);

    let compile_context = manager.context_stack.pop().unwrap();

    let code = code.unwrap_or_else(|e| {
        let diagnostic = e.as_diagnostic(file_id);

        term::emit(&mut err_writer.lock(), &err_config, &files, &diagnostic)
            .expect("Error writing error message");
        exit(1)
    });

    trace!("Bytecode succesfully compiled. Creating Frame");

    let global_context = bytecode::GlobalContext {
        constant_descriptors: compile_context.constant_descriptors,
        debug_descriptors: compile_context.debug_symbol_descriptors,
    };

    trace!(
        "Constant Descriptors: {:#?}",
        global_context.constant_descriptors
    );
    trace!("Debug Descriptors: {:#?}", global_context.debug_descriptors);

    let mut frame = bytecode::Frame::new(
        &code,
        &mut manager.memory_manager,
        Rc::new(global_context),
        HashMap::new(),
        STANDARD_CONTEXT_ID + 1,
    );

    trace!("{}", frame);

    trace!("Running frame");

    let computation = frame.run();

    let computation = computation.unwrap_or_else(|e| {
        let primary_diagnostic = e.as_diagnostic();

        term::emit(
            &mut err_writer.lock(),
            &err_config,
            &files,
            &primary_diagnostic,
        )
        .expect("Error writing error message");

        println!("Stack trace:");
        let secondary_diagnostics = e.stack_trace(&files);
        for secondary in secondary_diagnostics.iter() {
            println!("{}", secondary);
        }

        exit(1)
    });

    trace!("Run completed successfully");

    debug!("Computation returned: {:?}", computation);
}
