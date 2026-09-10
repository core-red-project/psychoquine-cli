use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Unsupported language: '{0}'")]
    UnsupportedLanguage(String),

    #[error("No runtime available in PATH for language '{0}'")]
    RuntimeUnavailable(String),

    #[error("Empty payload provided where non-empty payload was expected")]
    EmptyPayload,

    #[error("Quine generation failed for language '{language}': {reason}")]
    GenerationFailed { language: String, reason: String },

    #[error("Verification failed for {language}: source ({source_bytes} bytes) != output ({output_bytes} bytes)\n{details}")]
    VerificationMismatch {
        language: String,
        source_bytes: usize,
        output_bytes: usize,
        details: String,
    },

    #[error("Compilation error in sandbox for {language}:\n{details}")]
    CompilationFailed { language: String, details: String },

    #[error("Execution error in sandbox for {language}:\n{details}")]
    ExecutionFailed { language: String, details: String },

    #[error("Target file '{0}' already exists. Use --force to overwrite.")]
    FileAlreadyExists(String),

    #[error("Execution timed out in sandbox for {language} after {timeout_secs}s")]
    ExecutionTimeout { language: String, timeout_secs: u64 },

    #[error(
        "Payload size ({actual_bytes} bytes) exceeds maximum allowable limit of {max_bytes} bytes"
    )]
    PayloadTooLarge {
        max_bytes: usize,
        actual_bytes: usize,
    },

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
