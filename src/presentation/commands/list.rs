use colored::Colorize;
use serde::Serialize;
use std::process::ExitCode;
use std::sync::Arc;

use crate::application::registry::EngineRegistry;
use crate::domain::language::LanguageMetadata;
use crate::infrastructure::detector::SystemDetector;
use crate::presentation::args::{CliArgs, ListCommandArgs};
use crate::presentation::view::View;

#[derive(Serialize)]
struct LanguageStatusItem {
    language: String,
    name: String,
    family: String,
    extension: String,
    binary: String,
    available_locally: bool,
}

pub fn handle_list(
    args: &CliArgs,
    cmd_args: &ListCommandArgs,
    registry: Arc<EngineRegistry>,
    view: &View,
) -> ExitCode {
    let languages = registry.supported_languages();
    let mut items = Vec::new();

    for lang in languages {
        let meta: LanguageMetadata = lang.metadata();
        let rt = SystemDetector::detect_for_language(lang);

        if cmd_args.available && !rt.available {
            continue;
        }

        items.push(LanguageStatusItem {
            language: lang.slug().to_string(),
            name: meta.display_name.to_string(),
            family: meta.family.to_string(),
            extension: meta.extension.to_string(),
            binary: meta.default_binary.to_string(),
            available_locally: rt.available,
        });
    }

    if args.json {
        view.print_json(&items);
        return ExitCode::SUCCESS;
    }

    eprintln!(
        "\n{}",
        "─── Supported Quine Engines (Wikipedia Catalog) ───"
            .bold()
            .underline()
    );
    eprintln!(
        "{:<12} {:<24} {:<14} {:<8} {:<10} {}",
        "SLUG".bold(),
        "LANGUAGE".bold(),
        "FAMILY".bold(),
        "EXT".bold(),
        "BINARY".bold(),
        "LOCAL RUNTIME".bold()
    );
    eprintln!("{}", "─".repeat(80).dimmed());

    for item in &items {
        let status = if item.available_locally {
            "✓ Available".green().bold()
        } else {
            "✗ Not found".dimmed()
        };

        eprintln!(
            "{:<12} {:<24} {:<14} .{:<7} {:<10} {}",
            item.language.cyan(),
            item.name.bold(),
            item.family,
            item.extension,
            item.binary,
            status
        );
    }

    eprintln!("\nTotal: {} language engines available.\n", items.len());
    ExitCode::SUCCESS
}
