//! Runtime errors for technetium

use crate::bytecode::DebugSymbol;
use codespan::FileId;
use codespan::Files;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use std::borrow::Cow;
use std::cell;
use std::fmt;
use std::sync;
use sys_info;

/// The result of a computation on the technetium runtime
pub type RuntimeResult<T> = std::result::Result<T, RuntimeError>;

/// An error from a computation on the technetium runtime
#[derive(Clone, Debug)]
pub struct RuntimeError {
    pub err: RuntimeErrorType,
    /// A short description helping diagnose what caused the error
    pub help: String,
    /// The segment trace of user code where the error occured
    pub symbols: Vec<DebugSymbol>,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{:?}: {}", self.err, self.help)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum RuntimeErrorType {
    TypeError,
    IntegerTooBigError,
    AttributeError,
    InternalError,
    IndexOutOfBounds,
    ChildProcessError,
    /// Caused by trying to read an uninitialized variable
    VariableUndefinedError,
    /// Wrapping an std::io::Error
    IOError,
    /// An error raised by the sys_info crate
    SysInfoError,
    /// An error raised by trying to lock() a poisoned mutex on an Object
    PoisonError,
    /// An error raised by trying to modify and read something at the same time
    BorrowError,
    BorrowMutError,
    MutateImmutableError,
    /// Caused by trying to read a key that doesn't exist in a dictionary
    KeyError,
}

impl From<sys_info::Error> for RuntimeError {
    fn from(error: sys_info::Error) -> Self {
        RuntimeError {
            err: RuntimeErrorType::SysInfoError,
            help: error.to_string(),
            symbols: vec![],
        }
    }
}

impl From<std::io::Error> for RuntimeError {
    fn from(error: std::io::Error) -> Self {
        RuntimeError {
            err: RuntimeErrorType::IOError,
            help: error.to_string(),
            symbols: vec![],
        }
    }
}

impl<T> From<sync::PoisonError<T>> for RuntimeError {
    fn from(error: sync::PoisonError<T>) -> Self {
        RuntimeError {
            err: RuntimeErrorType::PoisonError,
            help: error.to_string(),
            symbols: vec![],
        }
    }
}

impl From<cell::BorrowError> for RuntimeError {
    fn from(error: cell::BorrowError) -> Self {
        RuntimeError {
            err: RuntimeErrorType::BorrowError,
            help: error.to_string(),
            symbols: vec![],
        }
    }
}

impl From<cell::BorrowMutError> for RuntimeError {
    fn from(error: cell::BorrowMutError) -> Self {
        RuntimeError {
            err: RuntimeErrorType::BorrowMutError,
            help: error.to_string(),
            symbols: vec![],
        }
    }
}

impl From<mlrefcell::BorrowMutError> for RuntimeError {
    fn from(error: mlrefcell::BorrowMutError) -> Self {
        match error {
            mlrefcell::BorrowMutError::AlreadyBorrowed => RuntimeError {
                err: RuntimeErrorType::BorrowMutError,
                help: "tried to mutate and read from the same object".to_string(),
                symbols: vec![],
            },
            mlrefcell::BorrowMutError::Locked => RuntimeError {
                err: RuntimeErrorType::MutateImmutableError,
                help: "tried to mutate value that was forced to be immutable".to_string(),
                symbols: vec![],
            },
        }
    }
}

impl RuntimeError {
    pub fn type_error<S: ToString>(message: S) -> Self {
        RuntimeError {
            err: RuntimeErrorType::TypeError,
            help: message.to_string(),
            symbols: vec![],
        }
    }

    pub fn attribute_error<S: ToString>(message: S) -> Self {
        RuntimeError {
            err: RuntimeErrorType::AttributeError,
            help: message.to_string(),
            symbols: vec![],
        }
    }

    pub fn key_error<S: ToString>(message: S) -> Self {
        RuntimeError {
            err: RuntimeErrorType::KeyError,
            help: message.to_string(),
            symbols: vec![],
        }
    }

    pub fn internal_error<S: ToString>(message: S) -> Self {
        RuntimeError {
            err: RuntimeErrorType::InternalError,
            help: message.to_string(),
            symbols: vec![],
        }
    }

    pub fn variable_undefined_error<S: ToString>(message: S) -> Self {
        RuntimeError {
            err: RuntimeErrorType::VariableUndefinedError,
            help: message.to_string(),
            symbols: vec![],
        }
    }

    pub fn index_oob_error<S: ToString>(message: S) -> Self {
        RuntimeError {
            err: RuntimeErrorType::IndexOutOfBounds,
            help: message.to_string(),
            symbols: vec![],
        }
    }

    pub fn index_too_big_error<S: ToString>(message: S) -> Self {
        RuntimeError {
            err: RuntimeErrorType::IntegerTooBigError,
            help: message.to_string(),
            symbols: vec![],
        }
    }

    pub fn child_process_error<S: ToString>(message: S) -> Self {
        RuntimeError {
            err: RuntimeErrorType::ChildProcessError,
            help: message.to_string(),
            symbols: vec![],
        }
    }

    /// Attach a code location to an error, for reporting diagnostics to the user
    pub fn attach_span(mut self, debug: DebugSymbol) -> Self {
        self.symbols.push(debug);
        RuntimeError {
            err: self.err,
            help: self.help,
            symbols: self.symbols,
        }
    }

    /// Create a diagnostic message from an error, for reporting to the user
    pub fn as_diagnostic(&self) -> Diagnostic<FileId> {
        match self.symbols.get(0) {
            Some(&symbol) => Diagnostic::error()
                .with_message(format!("Runtime Error: {:?}", self.err))
                .with_labels(vec![
                    Label::primary(symbol.file_id, symbol.location).with_message(&self.help)
                ]),
            None => Diagnostic::error().with_message(&self.help),
        }
    }

    pub fn stack_trace(&self, files: &Files<Cow<'_, str>>) -> Vec<String> {
        let mut res = vec![];
        for symbol in self.symbols.iter() {
            let slice = files
                .source_slice(symbol.file_id, symbol.location)
                .unwrap_or("<Unknown>");
            let location = files
                .location(symbol.file_id, symbol.location.start())
                .map_or("??".to_string(), |location| {
                    format!(
                        "line {}, col {}",
                        location.line.number(),
                        location.column.number()
                    )
                });
            let fname = files.name(symbol.file_id).to_string_lossy();
            res.push(format!("{} at {}: \"{}\"", fname, location, slice));
        }
        res
    }
}
