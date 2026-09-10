use serde::Serialize;
use std::sync::Arc;

use crate::application::registry::EngineRegistry;
use crate::domain::error::DomainError;
use crate::domain::language::{Language, LanguageMetadata};

#[derive(Debug, Clone, Serialize)]
pub struct ExplainQuineResponse {
    pub metadata: LanguageMetadata,
    pub canonical_sample: String,
    pub kleene_breakdown: String,
    pub technical_notes: Vec<&'static str>,
}

pub struct ExplainQuineUseCase {
    registry: Arc<EngineRegistry>,
}

impl ExplainQuineUseCase {
    pub fn new(registry: Arc<EngineRegistry>) -> Self {
        Self { registry }
    }

    pub fn execute(&self, language: Language) -> Result<ExplainQuineResponse, DomainError> {
        let engine = self.registry.get(language)?;
        let metadata = language.metadata();
        let canonical_sample = engine.generate(None)?;

        let technical_notes = match language {
            Language::Python => vec![
                "Uses format string {!r} which invokes repr()",
                "Python repr() ensures single quotes are balanced and escaped",
                "Output matches source code with exact zero-byte tolerance",
            ],
            Language::Javascript => vec![
                "Uses console.log format string with %j",
                "%j serializes the argument using JSON.stringify()",
                "Automatic newline matching matches the file line termination",
            ],
            Language::C => vec![
                "Uses printf with ASCII decimal codes: 10 (newline) and 34 (double quote)",
                "Avoids literal quote escape nesting completely",
                "Strict ANSI C standard compliance",
            ],
            Language::Rust => vec![
                "Uses print! macro with {0:?} positional debug specifier",
                "The Rust compiler's debug formatter provides safe string tokenization",
                "Produces a self-contained, compilable main() function",
            ],
            Language::Bash => vec![
                "Uses printf with octal escape \\47 for single quote",
                "Bypasses shell word-splitting and quote-stripping",
                "Runs on POSIX shells, Bash 3, Bash 4, Bash 5, Dash, and Zsh",
            ],
            Language::Go => vec![
                "Uses fmt.Printf with %q string quote verb",
                "Go's %q guarantees valid Go string literal syntax",
            ],
            Language::Ruby => vec![
                "Uses eval coupled with Kernel#p inspection",
                "Compact 1-line self-evaluator",
            ],
            _ => vec![
                "Solves Kleene's Second Recursion Theorem directly",
                "Separate Data part from Code generator part",
            ],
        };

        Ok(ExplainQuineResponse {
            metadata: metadata.clone(),
            canonical_sample,
            kleene_breakdown: metadata.kleene_principle.to_string(),
            technical_notes,
        })
    }
}
