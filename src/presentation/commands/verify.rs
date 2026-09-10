use colored::Colorize;
use std::fs;
use std::process::ExitCode;

use crate::application::verify_usecase::VerifyQuineUseCase;
use crate::presentation::args::{CliArgs, VerifyCommandArgs};
use crate::presentation::view::View;

pub fn handle_verify(args: &CliArgs, cmd_args: &VerifyCommandArgs, view: &View) -> ExitCode {
    let lang = match cmd_args
        .lang
        .or_else(|| VerifyQuineUseCase::infer_language_from_path(&cmd_args.file))
    {
        Some(l) => l,
        None => {
            eprintln!(
                "{}: Cannot infer language from file extension. Specify with --lang <LANG>",
                "Error".red().bold()
            );
            return ExitCode::FAILURE;
        }
    };

    let code = match fs::read_to_string(&cmd_args.file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "{}: Failed to read '{}': {}",
                "Error".red().bold(),
                cmd_args.file.display(),
                e
            );
            return ExitCode::FAILURE;
        }
    };

    if !args.quiet {
        eprintln!(
            "Verifying quine '{}' using {} runtime...",
            cmd_args.file.display(),
            lang.metadata().display_name.cyan().bold()
        );
    }

    let use_case = VerifyQuineUseCase;
    match use_case.execute(lang, &code) {
        Ok(report) => {
            if args.json {
                view.print_json(&report);
            } else {
                view.render_verification_report(&report);
            }

            if report.success {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(2)
            }
        }
        Err(e) => {
            eprintln!("{}: {}", "Verification Error".red().bold(), e);
            ExitCode::FAILURE
        }
    }
}
