//! Lextime and Parsetime errors for technetium

use crate::Tok;
use codespan::Span;
use codespan_reporting::diagnostic::{Diagnostic, Label};

use lalrpop_util;

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
    PostPreOp(Span),
    Lex(LexError),
    Recursive(Box<ParseError>),
}

impl MiscParseError {
    pub fn post_pre_op(l: usize, r: usize) -> Self {
        MiscParseError::PostPreOp(Span::new(l as u32, r as u32))
    }

    pub fn lex(message: &str, loc: Option<usize>) -> Self {
        MiscParseError::Lex(LexError {
            message: message.to_owned(),
            loc,
        })
    }

    pub fn as_diagnostic<FileId>(&self, fileid: FileId) -> Diagnostic<FileId> {
        match self {
            MiscParseError::PostPreOp(s) => {
                Diagnostic::error()
                    .with_message("Parse error: invalid ++ or --")
                    .with_labels(vec![Label::primary(
                            fileid,
                            *s
                    ).with_message("Thing being incremented must be either a variable ('x++'), an attribute ('x.a++'), or an index ('x[0]++')")])
            },
            MiscParseError::Lex(l) => l.as_diagnostic(fileid),
            MiscParseError::Recursive(p) => parse_error_to_diagnostic(&p, fileid),
        }
    }

    pub fn offset_spans(&mut self, offset: usize) {
        match self {
            MiscParseError::PostPreOp(s) => {
                let l = s.start();
                let r = s.end();
                *s = Span::new(
                    u32::from(l) + (offset as u32),
                    u32::from(r) + (offset as u32),
                );
            }
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
            expected: _e,
        } => *l += offset,
        ParseError::UnrecognizedToken {
            token: t,
            expected: _e,
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
