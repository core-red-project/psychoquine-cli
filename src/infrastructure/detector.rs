use crate::domain::language::Language;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeInfo {
    pub language: Language,
    pub binary: String,
    pub version: Option<String>,
    pub available: bool,
}

pub struct SystemDetector;

impl SystemDetector {
    pub fn probe_binary(binary: &str, version_arg: &str) -> (bool, Option<String>) {
        let res = Command::new(binary).arg(version_arg).output();
        match res {
            Ok(out) if out.status.success() => {
                let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
                let full = if !stdout.is_empty() { stdout } else { stderr };
                let first_line = full.lines().next().unwrap_or("").to_string();
                (true, Some(first_line))
            }
            _ => (false, None),
        }
    }

    pub fn detect_for_language(lang: Language) -> RuntimeInfo {
        let meta = lang.metadata();
        let binary_candidates: &[&str] = match lang {
            Language::Python => &["python3", "python"],
            Language::Javascript => &["node", "bun", "deno"],
            Language::C => &["clang", "gcc", "cc"],
            Language::CSharp => &["dotnet"],
            Language::Rust => &["rustc"],
            Language::Bash => &["bash"],
            Language::Perl => &["perl"],
            Language::Php => &["php"],
            Language::Ruby => &["ruby"],
            Language::Go => &["go"],
            Language::Lua => &["lua", "luajit"],
            Language::Ocaml => &["ocaml", "ocamlopt"],
            Language::Pascal => &["fpc"],
            Language::Scheme => &["scheme", "guile", "racket"],
            Language::CommonLisp => &["sbcl", "clisp"],
            Language::DosBatch => &["cmd"],
            Language::Brainfuck => &["brainfuck", "bf"],
            Language::Hq9Plus => &["hq9plus"],
        };

        for bin in binary_candidates {
            let (found, ver) = Self::probe_binary(bin, "--version");
            if found {
                return RuntimeInfo {
                    language: lang,
                    binary: bin.to_string(),
                    version: ver,
                    available: true,
                };
            }
        }

        RuntimeInfo {
            language: lang,
            binary: meta.default_binary.to_string(),
            version: None,
            available: false,
        }
    }

    pub fn detect_all() -> Vec<RuntimeInfo> {
        Language::ALL
            .iter()
            .map(|&l| Self::detect_for_language(l))
            .collect()
    }
}
