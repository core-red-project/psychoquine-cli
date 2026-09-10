use crate::application::doctor_usecase::DoctorUseCase;
use crate::presentation::args::CliArgs;
use crate::presentation::view::View;
use std::process::ExitCode;

pub fn handle_doctor(args: &CliArgs, view: &View) -> ExitCode {
    let use_case = DoctorUseCase;
    let runtimes = use_case.execute();

    if args.json {
        view.print_json(&runtimes);
    } else {
        view.render_doctor_table(&runtimes);
    }

    let available_any = runtimes.iter().any(|r| r.available);
    if available_any {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
