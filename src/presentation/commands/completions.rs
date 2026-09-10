use std::io;
use std::process::ExitCode;

use clap::CommandFactory;
use clap_complete::generate;

use crate::presentation::args::{CliArgs, CompletionsCommandArgs};

pub fn handle_completions(args: &CompletionsCommandArgs) -> ExitCode {
    let mut cmd = CliArgs::command();
    let bin_name = cmd.get_name().to_string();
    generate(args.shell, &mut cmd, bin_name, &mut io::stdout());
    ExitCode::SUCCESS
}
