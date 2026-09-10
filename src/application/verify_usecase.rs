use crate::domain::error::DomainError;
use crate::domain::language::Language;
use crate::domain::quine::Quine;
use crate::domain::report::VerificationReport;
use crate::infrastructure::sandbox::SandboxRunner;
use std::path::Path;

pub struct VerifyQuineUseCase;

impl VerifyQuineUseCase {
    pub fn infer_language_from_path(path: &Path) -> Option<Language> {
        let ext = path.extension()?.to_str()?;
        match ext {
            "py" => Some(Language::Python),
            "js" => Some(Language::Javascript),
            "c" => Some(Language::C),
            "cs" => Some(Language::CSharp),
            "rs" => Some(Language::Rust),
            "sh" => Some(Language::Bash),
            "pl" => Some(Language::Perl),
            "php" => Some(Language::Php),
            "rb" => Some(Language::Ruby),
            "go" => Some(Language::Go),
            "lua" => Some(Language::Lua),
            "ml" => Some(Language::Ocaml),
            "pas" => Some(Language::Pascal),
            "scm" => Some(Language::Scheme),
            "lisp" => Some(Language::CommonLisp),
            "bat" | "cmd" => Some(Language::DosBatch),
            "bf" => Some(Language::Brainfuck),
            "hq9" => Some(Language::Hq9Plus),
            _ => None,
        }
    }

    pub fn execute(
        &self,
        language: Language,
        code: &str,
    ) -> Result<VerificationReport, DomainError> {
        let (stdout, runtime) = SandboxRunner::execute(language, code)?;
        let success = code == stdout;
        let source_sha = Quine::calculate_sha256(code);
        let output_sha = Quine::calculate_sha256(&stdout);

        let diff_preview = if !success {
            Some(format!(
                "Byte length mismatch: source={}, output={}\nPreview output: {:?}",
                code.len(),
                stdout.len(),
                stdout.chars().take(120).collect::<String>()
            ))
        } else {
            None
        };

        Ok(VerificationReport {
            language,
            success,
            is_dry_run: false,
            runtime_binary: runtime.binary,
            source_bytes: code.len(),
            output_bytes: stdout.len(),
            sha256_source: source_sha,
            sha256_output: output_sha,
            diff_preview,
        })
    }
}
