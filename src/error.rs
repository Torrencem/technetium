//! Lextime, Parsetime, Compiletime, and Runtime errors for technetium
//!

use crate::bytecode::Op;
use crate::lexer::Tok;
use codespan::{FileId, Span};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use std::fmt;
use std::sync;
use sys_info;

use lalrpop_util;

/// The result of a computation on the technetium runtime
pub type RuntimeResult<T> = std::result::Result<T, RuntimeError>;

/// An error from a computation on the technetium runtime
#[derive(Clone, Debug)]
pub struct RuntimeError {
    pub err: RuntimeErrorType,
    /// A short description helping diagnose what caused the error
    pub help: String,
    /// The segment trace of user code where the error occured
    pub span: Vec<Span>,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{:?}: {}", self.err, self.help)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum RuntimeErrorType {
    TypeError,
    AttributeError,
    InternalError,
    IndexOutOfBounds,
    ChildProcessError,
    /// Wrapping an std::io::Error
    IOError,
    /// An error raised by the sys_info crate
    SysInfoError,
    /// An error raised by trying to lock() a poisoned mutex on an Object
    PoisonError,
}

impl From<sys_info::Error> for RuntimeError {
    fn from(error: sys_info::Error) -> Self {
        RuntimeError {
            err: RuntimeErrorType::SysInfoError,
            help: error.to_string(),
            span: vec![],
        }
    }
}

impl From<std::io::Error> for RuntimeError {
    fn from(error: std::io::Error) -> Self {
        RuntimeError {
            err: RuntimeErrorType::IOError,
            help: error.to_string(),
            span: vec![],
        }
    }
}

impl<T> From<sync::PoisonError<T>> for RuntimeError {
    fn from(error: sync::PoisonError<T>) -> Self {
        RuntimeError {
            err: RuntimeErrorType::PoisonError,
            help: error.to_string(),
            span: vec![],
        }
    }
}

impl RuntimeError {
    pub fn type_error<S: ToString>(message: S) -> Self {
        RuntimeError {
            err: RuntimeErrorType::TypeError,
            help: message.to_string(),
            span: vec![],
        }
    }

    pub fn attribute_error<S: ToString>(message: S) -> Self {
        RuntimeError {
            err: RuntimeErrorType::AttributeError,
            help: message.to_string(),
            span: vec![],
        }
    }

    pub fn internal_error<S: ToString>(message: S) -> Self {
        RuntimeError {
            err: RuntimeErrorType::InternalError,
            help: message.to_string(),
            span: vec![],
        }
    }

    pub fn index_oob_error<S: ToString>(message: S) -> Self {
        RuntimeError {
            err: RuntimeErrorType::IndexOutOfBounds,
            help: message.to_string(),
            span: vec![],
        }
    }

    pub fn child_process_error<S: ToString>(message: S) -> Self {
        RuntimeError {
            err: RuntimeErrorType::ChildProcessError,
            help: message.to_string(),
            span: vec![],
        }
    }

    /// Attach a code location to an error, for reporting diagnostics to the user
    pub fn attach_span(mut self, span: Span) -> Self {
        self.span.push(span);
        RuntimeError {
            err: self.err,
            help: self.help,
            span: self.span,
        }
    }

    /// Create a diagnostic message from an error, for reporting to the user
    pub fn as_diagnostic<FileId>(&self, fileid: FileId) -> Diagnostic<FileId> {
        // TODO: use other spans in the stack
        match self.span.get(0) {
            Some(&span) => Diagnostic::error()
                .with_message(format!("Runtime Error: {:?}", self.err))
                .with_labels(vec![Label::primary(fileid, span).with_message(&self.help)]),
            None => Diagnostic::error().with_message(&self.help),
        }
    }
}

/// An error that has occured when translating code from the AST to bytecode
#[derive(Debug, Clone)]
pub struct CompileError {
    pub kind: CompileErrorType,
    pub help: String,
}

#[derive(Debug, Clone)]
pub enum CompileErrorType {
    UndefinedVariable(Span),
}

impl CompileError {
    pub fn new<S: ToString>(kind: CompileErrorType, help: S) -> Self {
        CompileError {
            kind,
            help: help.to_string(),
        }
    }

    /// Create a diagnostic message from an error, for reporting to the user
    pub fn as_diagnostic<FileId>(&self, fileid: FileId) -> Diagnostic<FileId> {
        match self.kind {
            CompileErrorType::UndefinedVariable(span) => Diagnostic::error()
                .with_message(self.help.clone())
                .with_labels(vec![
                    Label::primary(fileid, span).with_message("Undefined variable")
                ]),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LexError {
    pub message: String,
    pub loc: Option<usize>,
}

impl LexError {
    pub fn new(message: &str, loc: Option<usize>) -> Self {
        LexError {
            message: message.to_owned(),
            loc,
        }
    }

    pub fn offset_spans(&mut self, offset: usize) {
        if let Some(loc) = &mut self.loc {
            *loc += offset;
        }
    }

    pub fn as_diagnostic<FileId>(&self, fileid: FileId) -> Diagnostic<FileId> {
        match self.loc {
            Some(loc) => {
                let loc = loc as u32;
                Diagnostic::error()
                    .with_message("Lex Error")
                    .with_labels(vec![
                        Label::primary(fileid, Span::new(loc, loc + 1)).with_message(&self.message)
                    ])
            }
            None => Diagnostic::error().with_message(&self.message),
        }
    }
}

#[derive(Clone, Debug)]
pub enum MiscParseError {
    Lex(LexError),
    Recursive(Box<ParseError>),
}

impl MiscParseError {
    pub fn lex(message: &str, loc: Option<usize>) -> Self {
        MiscParseError::Lex(LexError {
            message: message.to_owned(),
            loc,
        })
    }

    pub fn as_diagnostic<FileId>(&self, fileid: FileId) -> Diagnostic<FileId> {
        match self {
            MiscParseError::Lex(l) => l.as_diagnostic(fileid),
            MiscParseError::Recursive(p) => parse_error_to_diagnostic(&p, fileid),
        }
    }

    pub fn offset_spans(&mut self, offset: usize) {
        match self {
            MiscParseError::Lex(l) => l.offset_spans(offset),
            MiscParseError::Recursive(p) => offset_parse_error_spans(p, offset),
        }
    }
}

pub fn parse_error_to_diagnostic<FileId>(p: &ParseError, fileid: FileId) -> Diagnostic<FileId> {
    match p {
        ParseError::InvalidToken { location: l } => Diagnostic::error()
            .with_message("Parse error: Invalid Token")
            .with_labels(vec![Label::primary(
                fileid,
                Span::new(*l as u32, *l as u32 + 1),
            )
            .with_message("Invalid or unknown token")]),
        ParseError::UnrecognizedEOF {
            location: l,
            expected: e,
        } => Diagnostic::error()
            .with_message("Parse error: Unrecognized End of Input")
            .with_labels(vec![Label::primary(
                fileid,
                Span::new(*l as u32, *l as u32 + 1),
            )
            .with_message("Invalid EOI")])
            .with_notes(vec![format!("Expected one of {:?} after this point", e)]),
        ParseError::UnrecognizedToken {
            token: t,
            expected: e,
        } => Diagnostic::error()
            .with_message("Parse error: Unrecognized or unexpected token")
            .with_labels(vec![Label::primary(
                fileid,
                Span::new(t.0 as u32, t.2 as u32),
            )
            .with_message(format!("Did not expect {:?} here", t.1))])
            .with_notes(vec![format!("Expected one of {:?}", e)]),
        ParseError::ExtraToken { token: t } => Diagnostic::error()
            .with_message("Parse error: extra token")
            .with_labels(vec![Label::primary(
                fileid,
                Span::new(t.0 as u32, t.2 as u32),
            )
            .with_message(format!("Did not expect {:?} here", t.1))]),
        ParseError::User { error: e } => e.as_diagnostic(fileid),
    }
}

pub type ParseError = lalrpop_util::ParseError<usize, Tok, MiscParseError>;

impl From<MiscParseError> for ParseError {
    fn from(x: MiscParseError) -> Self {
        ParseError::User { error: x }
    }
}

impl From<ParseError> for MiscParseError {
    fn from(x: ParseError) -> Self {
        MiscParseError::Recursive(Box::new(x))
    }
}

pub fn offset_parse_error_spans(p: &mut ParseError, offset: usize) {
    match p {
        ParseError::InvalidToken { location: l } => *l += offset,
        ParseError::UnrecognizedEOF {
            location: l,
            expected: e,
        } => *l += offset,
        ParseError::UnrecognizedToken {
            token: t,
            expected: e,
        } => *t = (t.0 + offset, t.1.clone(), t.2 + offset),
        ParseError::ExtraToken { token: t } => *t = (t.0 + offset, t.1.clone(), t.2 + offset),
        ParseError::User { error: e } => e.offset_spans(offset),
    }
}

pub fn offset_parse_result_error_spans<T>(p: &mut Result<T, ParseError>, offset: usize) {
    if let Err(e) = p {
        offset_parse_error_spans(e, offset)
    }
}
