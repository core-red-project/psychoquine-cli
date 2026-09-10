pub mod completions;
pub mod doctor;
pub mod explain;
pub mod generate;
pub mod list;
pub mod verify;

pub use completions::handle_completions;
pub use doctor::handle_doctor;
pub use explain::handle_explain;
pub use generate::handle_generate;
pub use list::handle_list;
pub use verify::handle_verify;
