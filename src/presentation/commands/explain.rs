use colored::Colorize;
use std::process::ExitCode;
use std::sync::Arc;

use crate::application::explain_usecase::ExplainQuineUseCase;
use crate::application::registry::EngineRegistry;
use crate::presentation::args::{CliArgs, ExplainCommandArgs};
use crate::presentation::view::View;

pub fn handle_explain(
    args: &CliArgs,
    cmd_args: &ExplainCommandArgs,
    registry: Arc<EngineRegistry>,
    view: &View,
) -> ExitCode {
    let use_case = ExplainQuineUseCase::new(registry);
    match use_case.execute(cmd_args.lang) {
        Ok(explanation) => {
            if args.json {
                view.print_json(&explanation);
            } else {
                eprintln!(
                    "\n{}",
                    format!(
                        "─── Quine Architecture: {} ───",
                        explanation.metadata.display_name
                    )
                    .bold()
                    .underline()
                );
                eprintln!("Family:         {}", explanation.metadata.family);
                eprintln!("Extension:      .{}", explanation.metadata.extension);
                eprintln!(
                    "Default Runner: {}",
                    explanation.metadata.default_binary.cyan()
                );
                eprintln!("\nDescription:\n  {}", explanation.metadata.description);
                eprintln!(
                    "\nKleene Recursion Mechanics:\n  {}",
                    explanation.kleene_breakdown.italic()
                );
                eprintln!("\nTechnical Invariants:");
                for note in &explanation.technical_notes {
                    eprintln!("  • {}", note);
                }
                eprintln!(
                    "\nCanonical Source Code:\n{}",
                    explanation.canonical_sample.dimmed()
                );
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{}: {}", "Error".red().bold(), e);
            ExitCode::FAILURE
        }
    }
}
