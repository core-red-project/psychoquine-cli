use std::fs;
use std::io::{self, Read};
use std::process::ExitCode;
use std::sync::Arc;

use colored::Colorize;
use serde::Serialize;

use crate::application::explain_usecase::ExplainQuineUseCase;
use crate::application::generate_usecase::{GenerateQuineRequest, GenerateQuineUseCase};
use crate::application::registry::EngineRegistry;
use crate::domain::error::DomainError;
use crate::domain::language::Language;
use crate::domain::payload::PayloadMode;
use crate::presentation::args::CliArgs;
use crate::presentation::view::View;

#[derive(Serialize)]
struct JsonOutput {
    success: bool,
    language: String,
    dry_run: bool,
    bytes: usize,
    lines: usize,
    expansion_ratio: f64,
    sha256: String,
    code: String,
    verification: Option<crate::domain::report::VerificationReport>,
}

fn read_stdin() -> Option<String> {
    let mut buffer = String::new();
    let mut handle = io::stdin().take((crate::domain::payload::MAX_PAYLOAD_BYTES + 1) as u64);
    if handle.read_to_string(&mut buffer).is_ok() {
        let trimmed = buffer.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }
    None
}

pub fn handle_generate(
    args: &CliArgs,
    target: Option<String>,
    arg_payload: Option<String>,
    registry: Arc<EngineRegistry>,
    view: &View,
) -> ExitCode {
    let use_case = GenerateQuineUseCase::new(registry.clone());

    // Resolve language and payload with human-friendly CLI ergonomics
    let mut resolved_lang = args.lang;
    let mut raw_payload = None;

    if let Some(ref t) = target {
        if let Ok(parsed) = t.parse::<Language>() {
            if resolved_lang.is_none() {
                resolved_lang = Some(parsed);
            }
            raw_payload = arg_payload;
        } else {
            raw_payload = Some(t.clone());
        }
    } else if let Some(ref qp) = args.quick_payload {
        if let Ok(parsed) = qp.parse::<Language>() {
            if resolved_lang.is_none() {
                resolved_lang = Some(parsed);
            }
        } else {
            raw_payload = Some(qp.clone());
        }
    }

    if let Some(ref pf) = args.payload_file {
        match fs::metadata(pf) {
            Ok(meta) => {
                if meta.len() > crate::domain::payload::MAX_PAYLOAD_BYTES as u64 {
                    eprintln!(
                        "{}: Payload file '{}' ({} bytes) exceeds maximum limit of {} bytes",
                        "Error".red().bold(),
                        pf.display(),
                        meta.len(),
                        crate::domain::payload::MAX_PAYLOAD_BYTES
                    );
                    return ExitCode::FAILURE;
                }
            }
            Err(e) => {
                eprintln!(
                    "{}: Cannot access payload file '{}': {}",
                    "Error".red().bold(),
                    pf.display(),
                    e
                );
                return ExitCode::FAILURE;
            }
        }

        match fs::read_to_string(pf) {
            Ok(content) => {
                let trimmed = content.trim();
                if !trimmed.is_empty() {
                    raw_payload = Some(trimmed.to_string());
                }
            }
            Err(e) => {
                eprintln!(
                    "{}: Failed to read payload file '{}': {}",
                    "Error".red().bold(),
                    pf.display(),
                    e
                );
                return ExitCode::FAILURE;
            }
        }
    } else if args.stdin
        || raw_payload.as_deref() == Some("-")
        || args.quick_payload.as_deref() == Some("-")
    {
        raw_payload = read_stdin();
    }

    let payload_mode = if args.banner {
        PayloadMode::Banner
    } else if let Some(p) = raw_payload {
        PayloadMode::Text(p)
    } else {
        PayloadMode::None
    };

    let request = GenerateQuineRequest {
        language: resolved_lang,
        payload_mode,
        dry_run: args.dry_run,
        verify: args.verify,
    };

    let response = match use_case.execute(request) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("{}: {}", "Generation Error".red().bold(), e);
            return ExitCode::FAILURE;
        }
    };

    if args.json {
        let json_data = JsonOutput {
            success: true,
            language: response.quine.language.to_string(),
            dry_run: response.is_dry_run,
            bytes: response.quine.stats.bytes,
            lines: response.quine.stats.lines,
            expansion_ratio: response.quine.stats.expansion_ratio,
            sha256: response.quine.sha256,
            code: response.quine.source_code,
            verification: response.verification,
        };
        view.print_json(&json_data);
        return ExitCode::SUCCESS;
    }

    // Handle explain flag if requested alongside generate
    if args.explain {
        let explain_use_case = ExplainQuineUseCase::new(registry);
        if let Ok(exp) = explain_use_case.execute(response.quine.language) {
            eprintln!("\n{}", "─── Mathematical Explanation ───".bold().cyan());
            eprintln!("Language:  {}", exp.metadata.display_name.bold());
            eprintln!("Mechanism: {}", exp.kleene_breakdown);
            for note in &exp.technical_notes {
                eprintln!("  • {}", note);
            }
            eprintln!();
        }
    }

    // Handle output destination
    if let Some(ref out_path) = args.output {
        if out_path.exists() && !args.force && !args.dry_run {
            eprintln!(
                "{}: {}",
                "Error".red().bold(),
                DomainError::FileAlreadyExists(out_path.display().to_string())
            );
            return ExitCode::FAILURE;
        }

        if !args.dry_run {
            if let Err(e) = fs::write(out_path, &response.quine.source_code) {
                eprintln!(
                    "{}: Failed to write to '{}': {}",
                    "Error".red().bold(),
                    out_path.display(),
                    e
                );
                return ExitCode::FAILURE;
            }
            if !args.quiet {
                eprintln!(
                    "{} Quine written to '{}'",
                    "✓".green().bold(),
                    out_path.display()
                );
                eprintln!(
                    "  Language:   {}",
                    response.quine.language.metadata().display_name.cyan()
                );
                eprintln!("  Size:       {} bytes", response.quine.stats.bytes);
                eprintln!("  SHA256:     {}", response.quine.sha256.dimmed());
            }
        } else if !args.quiet {
            eprintln!(
                "{} [DRY-RUN] Would write to '{}'",
                "•".yellow().bold(),
                out_path.display()
            );
            eprintln!("  Projected size: {} bytes", response.quine.stats.bytes);
        }
    } else {
        view.render_output_code(&response.quine.source_code);
    }

    // Display statistics if --stats is active
    if args.stats && !args.quiet {
        eprintln!("\n{}", "─── Quine Metrics ───".bold());
        eprintln!("Bytes:           {}", response.quine.stats.bytes);
        eprintln!("Lines:           {}", response.quine.stats.lines);
        eprintln!(
            "Expansion ratio: {:.2}x",
            response.quine.stats.expansion_ratio
        );
        eprintln!("SHA256:          {}", response.quine.sha256);
    }

    // Render verification report if active
    if let Some(ref report) = response.verification {
        view.render_verification_report(report);
        if !report.success && !report.is_dry_run {
            return ExitCode::from(2);
        }
    }

    ExitCode::SUCCESS
}
