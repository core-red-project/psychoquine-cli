use crate::domain::language::Language;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "psychoquine",
    author = "Sxnnyside Project",
    version,
    about = "Universal resource-agnostic Quine generator and verifier",
    long_about = "A state-of-the-art metaprogramming CLI for generating legitimate, self-replicating programs across multiple languages with payload injection and live execution verification."
)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Quick payload text for implicit generate
    pub quick_payload: Option<String>,

    /// Target programming language
    #[arg(short, long, global = true)]
    pub lang: Option<Language>,

    /// Simulate generation without writing files or running external compilers
    #[arg(short = 'd', long, global = true)]
    pub dry_run: bool,

    /// Verify quine execution immediately by running in a child sandbox
    #[arg(short, long, global = true)]
    pub verify: bool,

    /// Explain the mathematical structure and Kleene recursion mechanics
    #[arg(long, global = true)]
    pub explain: bool,

    /// Show generation statistics (expansion ratio, bytes, lines)
    #[arg(short = 's', long, global = true)]
    pub stats: bool,

    /// Overwrite existing output file without confirmation
    #[arg(short = 'f', long, global = true)]
    pub force: bool,

    /// Embed the official Sxnnyside Project ASCII banner
    #[arg(short = 'b', long, global = true)]
    pub banner: bool,

    /// Output to a specific file instead of stdout
    #[arg(short, long, global = true)]
    pub output: Option<PathBuf>,

    /// Output results in structured JSON format
    #[arg(long, global = true)]
    pub json: bool,

    /// Read payload from standard input (stdin)
    #[arg(long, global = true)]
    pub stdin: bool,

    /// Read payload from a file path
    #[arg(long = "payload-file", global = true)]
    pub payload_file: Option<PathBuf>,

    /// Disable terminal ANSI colors (honors NO_COLOR)
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Suppress banners and decorative output
    #[arg(short, long, global = true)]
    pub quiet: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Generate a self-replicating program (quine)
    Generate(GenerateCommandArgs),

    /// Verify an existing quine file by executing it and asserting byte-for-byte identity
    Verify(VerifyCommandArgs),

    /// Explain the mathematical quine logic and Kleene theorem for a language
    Explain(ExplainCommandArgs),

    /// Audit system PATH for available runtimes and compilers
    Doctor,

    /// List all supported languages, extensions, and runtime statuses
    List(ListCommandArgs),

    /// Alias for list
    Langs(ListCommandArgs),

    /// Generate shell completion scripts (bash, zsh, fish, powershell, elvish)
    Completions(CompletionsCommandArgs),
}

#[derive(Args, Debug)]
pub struct GenerateCommandArgs {
    /// Target language or payload (e.g. `psychoquine generate python` or `psychoquine generate python "hello"`)
    pub target: Option<String>,

    /// Text or payload to embed into the quine (use '-' to read from stdin)
    pub payload: Option<String>,
}

#[derive(Args, Debug)]
pub struct VerifyCommandArgs {
    /// Path to the quine file to verify
    pub file: PathBuf,

    /// Language override (otherwise inferred from file extension)
    #[arg(short, long)]
    pub lang: Option<Language>,
}

#[derive(Args, Debug)]
pub struct ExplainCommandArgs {
    /// Target language to explain
    pub lang: Language,
}

#[derive(Args, Debug, Default)]
pub struct ListCommandArgs {
    /// Filter only languages with runtimes currently installed on this system
    #[arg(short, long)]
    pub available: bool,
}

#[derive(Args, Debug)]
pub struct CompletionsCommandArgs {
    /// Target shell to generate completions for
    pub shell: clap_complete::Shell,
}
