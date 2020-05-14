//! Compiletime errors for technetium

use codespan::Span;
use codespan_reporting::diagnostic::{Diagnostic, Label};

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
