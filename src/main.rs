use std::process::ExitCode;
use std::sync::Arc;

use clap::Parser;
use psychoquine::application::registry::EngineRegistry;
use psychoquine::presentation::args::{CliArgs, Command};
use psychoquine::presentation::commands::{
    handle_completions, handle_doctor, handle_explain, handle_generate, handle_list, handle_verify,
};
use psychoquine::presentation::view::View;

fn main() -> ExitCode {
    let args = CliArgs::parse();
    let registry = Arc::new(EngineRegistry::new());
    let view = View::new(args.no_color, args.quiet);

    if !args.quiet && !args.json && args.output.is_some() {
        view.print_banner_stderr();
    }

    match args.command {
        Some(Command::Doctor) => handle_doctor(&args, &view),

        Some(Command::List(ref list_args)) | Some(Command::Langs(ref list_args)) => {
            handle_list(&args, list_args, registry, &view)
        }

        Some(Command::Explain(ref explain_args)) => {
            handle_explain(&args, explain_args, registry, &view)
        }

        Some(Command::Verify(ref verify_args)) => handle_verify(&args, verify_args, &view),

        Some(Command::Generate(ref gen_args)) => handle_generate(
            &args,
            gen_args.target.clone(),
            gen_args.payload.clone(),
            registry,
            &view,
        ),

        Some(Command::Completions(ref comp_args)) => handle_completions(comp_args),

        None => {
            // Implicit generate mode (e.g. `psychoquine "payload"`, `psychoquine rust`, or `psychoquine -l python`)
            handle_generate(&args, None, None, registry, &view)
        }
    }
}
