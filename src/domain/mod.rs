pub mod error;
pub mod language;
pub mod payload;
pub mod quine;
pub mod report;

pub use error::DomainError;
pub use language::{Language, LanguageFamily, LanguageMetadata};
pub use payload::{Payload, PayloadMode};
pub use quine::{Quine, QuineStats};
pub use report::VerificationReport;
