
use std::fmt;
use codespan::{Span, FileId};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use crate::bytecode::Op;
use crate::lexer::Tok;

use lalrpop_util;

pub type RuntimeResult<T> = std::result::Result<T, RuntimeError>;

#[derive(Clone, Debug)]
pub struct RuntimeError {
    err: RuntimeErrorType,
    help: String,
    span: Option<Span>,
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
    IOError,
}

impl From<std::io::Error> for RuntimeError {
    fn from(error: std::io::Error) -> Self {
        RuntimeError {
            err: RuntimeErrorType::IOError,
            help: error.to_string(),
            span: None,
        }
    }
}

impl RuntimeError {
    pub fn type_error(message: String) -> Self {
        RuntimeError {
            err: RuntimeErrorType::TypeError,
            help: message,
            span: None,
        }
    }
    
    pub fn attribute_error(message: String) -> Self {
        RuntimeError {
            err: RuntimeErrorType::AttributeError,
            help: message,
            span: None,
        }
    }

    pub fn internal_error(message: String) -> Self {
        RuntimeError {
            err: RuntimeErrorType::InternalError,
            help: message,
            span: None,
        }
    }

    pub fn index_oob_error(message: String) -> Self {
        RuntimeError {
            err: RuntimeErrorType::IndexOutOfBounds,
            help: message,
            span: None,
        }
    }
    
    pub fn child_process_error(message: String) -> Self {
        RuntimeError {
            err: RuntimeErrorType::ChildProcessError,
            help: message,
            span: None,
        }
    }

    pub fn attach_span(self, span: Span) -> Self {
        RuntimeError {
            err: self.err,
            help: self.help,
            span: Some(span),
        }
    }

    pub fn weak_attach_span(self, span: Span) -> Self {
        match self.span {
            Some(val) => {
                RuntimeError {
                    err: self.err,
                    help: self.help,
                    span: Some(val),
                }
            },
            None => {
                RuntimeError {
                    err: self.err,
                    help: self.help,
                    span: Some(span),
                }
            }
        }
    }
    
    pub fn as_diagnostic<FileId>(&self, fileid: FileId) -> Diagnostic<FileId> {
        match self.span {
            Some(span) => Diagnostic::error()
                .with_message(format!("Runtime Error: {:?}", self.err))
                .with_labels(vec![
                    Label::primary(fileid, span).with_message(&self.help),
                ]),
            None => Diagnostic::error()
                .with_message(&self.help),
        }
    }
}

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
    pub fn new(kind: CompileErrorType, help: &str) -> Self {
        CompileError { kind, help: help.to_string() }
    }

    pub fn as_diagnostic<FileId>(&self, fileid: FileId) -> Diagnostic<FileId> {
        match self.kind {
            CompileErrorType::UndefinedVariable(span) => Diagnostic::error()
                .with_message(self.help.clone())
                .with_labels(vec![
                    Label::primary(fileid, span).with_message("Undefined variable"),
                ]),
        }
    }
}

pub type CompileResult = std::result::Result<Vec<Op>, CompileError>;

#[derive(Clone, Debug)]
pub struct LexError {
    pub message: String,
    pub loc: Option<usize>,
}

impl LexError {
    pub fn new(message: &str, loc: Option<usize>) -> Self {
        LexError {
            message: message.to_owned(),
            loc
        }
    }

    pub fn as_diagnostic<FileId>(&self, fileid: FileId) -> Diagnostic<FileId> {
        match self.loc {
            Some(loc) => {
                let loc = loc as u32;
                Diagnostic::error()
                    .with_message("Lex Error".to_string())
                    .with_labels(vec![
                        Label::primary(fileid, Span::new(loc, loc+1)).with_message(&self.message),
                    ])
            },
            None => Diagnostic::error()
                .with_message(&self.message),
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
            loc
        })
    }

    pub fn as_diagnostic<FileId>(&self, fileid: FileId) -> Diagnostic<FileId> {
        match self {
            MiscParseError::Lex(l) => l.as_diagnostic(fileid),
            MiscParseError::Recursive(p) => parse_error_to_diagnostic(&p, fileid),
        }
    }
}

pub fn parse_error_to_diagnostic<FileId>(p: &ParseError, fileid: FileId) -> Diagnostic<FileId> {
    match p {
        ParseError::InvalidToken { location: l } => {
                Diagnostic::error()
                    .with_message("Parse error: Invalid Token".to_string())
                    .with_labels(vec![
                        Label::primary(fileid, Span::new(*l as u32, *l as u32 + 1)).with_message("Invalid or unknown token"),
                    ])
        },
        ParseError::UnrecognizedEOF { location: l, expected: e } => {
                Diagnostic::error()
                    .with_message("Parse error: Unrecognized End of File".to_string())
                    .with_labels(vec![
                        Label::primary(fileid, Span::new(*l as u32, *l as u32 - 1)).with_message("Invalid EOF"),
                    ])
                    .with_notes(vec![
                        format!("Expected one of {:?}", e)
                    ])
        },
        ParseError::UnrecognizedToken { token: t, expected: e } => {
            Diagnostic::error()
                .with_message("Parse error: Unrecognized or unexpected token".to_string())
                .with_labels(vec![
                    Label::primary(fileid, Span::new(t.0 as u32, t.2 as u32)).with_message(format!("Did not expect {:?} here", t.1)),
                ])
                .with_notes(vec![
                    format!("Expected one of {:?}", e)
                ])
        },
        ParseError::ExtraToken { token: t } => {
            Diagnostic::error()
                .with_message("Parse error: extra token".to_string())
                .with_labels(vec![
                    Label::primary(fileid, Span::new(t.0 as u32, t.2 as u32)).with_message(format!("Did not expect {:?} here", t.1)),
                ])
        },
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

