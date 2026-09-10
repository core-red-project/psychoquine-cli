use crate::domain::payload::SXNNYSIDE_BANNER;
use crate::domain::report::VerificationReport;
use crate::infrastructure::detector::RuntimeInfo;
use crate::infrastructure::terminal::Terminal;
use colored::Colorize;
use serde::Serialize;
use std::io::{self, Write};

pub struct View {
    no_color: bool,
    quiet: bool,
}

impl View {
    pub fn new(no_color: bool, quiet: bool) -> Self {
        let disable_color = no_color || Terminal::is_no_color_active();
        Self {
            no_color: disable_color,
            quiet,
        }
    }

    pub fn print_banner_stderr(&self) {
        if !self.quiet && Terminal::is_stderr_tty() {
            if self.no_color {
                eprintln!("{}", SXNNYSIDE_BANNER);
            } else {
                eprintln!("{}", SXNNYSIDE_BANNER.red().bold());
            }
        }
    }

    pub fn print_json<T: Serialize>(&self, data: &T) {
        if let Ok(json_str) = serde_json::to_string_pretty(data) {
            println!("{}", json_str);
        }
    }

    pub fn render_doctor_table(&self, runtimes: &[RuntimeInfo]) {
        if self.quiet {
            return;
        }

        let title = "Sxnnyside Project - System Runtime Audit";
        if self.no_color {
            eprintln!("\n{}", title);
            eprintln!("Inspecting PATH for execution runtimes & compilers:\n");
        } else {
            eprintln!("\n{}", title.bold().underline());
            eprintln!("Inspecting PATH for execution runtimes & compilers:\n");
        }

        let mut available_count = 0;
        for rt in runtimes {
            let meta = rt.language.metadata();
            if rt.available {
                available_count += 1;
                let ver_text = rt.version.as_deref().unwrap_or("detected");
                if self.no_color {
                    eprintln!(
                        "  [OK] {:<18} [{}] ({})",
                        meta.display_name, rt.binary, ver_text
                    );
                } else {
                    eprintln!(
                        "  {} {:<18} [{}] ({})",
                        "✓".green().bold(),
                        meta.display_name.bold(),
                        rt.binary.cyan(),
                        ver_text.dimmed()
                    );
                }
            } else if self.no_color {
                eprintln!(
                    "  [--] {:<18} [{}] (not found)",
                    meta.display_name, rt.binary
                );
            } else {
                eprintln!(
                    "  {} {:<18} [{}] (not found in PATH)",
                    "✗".red().bold(),
                    meta.display_name.dimmed(),
                    rt.binary.dimmed()
                );
            }
        }

        eprintln!(
            "\nStatus: {}/{} runtimes available.",
            available_count,
            runtimes.len()
        );
    }

    pub fn render_verification_report(&self, report: &VerificationReport) {
        if self.quiet {
            return;
        }

        if report.is_dry_run {
            if self.no_color {
                eprintln!(
                    "* [DRY-RUN] Quine execution simulated (target runner: {})",
                    report.runtime_binary
                );
                eprintln!(
                    "  Projected Size: {} bytes (SHA256: {})",
                    report.source_bytes, report.sha256_source
                );
            } else {
                eprintln!(
                    "{} {} (target runner: {})",
                    "•".yellow().bold(),
                    "[DRY-RUN] Quine execution simulated".yellow().bold(),
                    report.runtime_binary.cyan()
                );
                eprintln!(
                    "  Projected Size: {} bytes (SHA256: {})",
                    report.source_bytes,
                    report.sha256_source.dimmed()
                );
            }
            return;
        }

        if report.success {
            if self.no_color {
                eprintln!(
                    "[OK] Quine self-verified! (100% byte-for-byte identical via {})",
                    report.runtime_binary
                );
                eprintln!("  SHA256: {}", report.sha256_source);
            } else {
                eprintln!(
                    "{} {} ({})",
                    "✓".green().bold(),
                    "Quine self-verified! (100% byte-for-byte identical)"
                        .green()
                        .bold(),
                    report.runtime_binary.cyan()
                );
                eprintln!("  SHA256: {}", report.sha256_source.dimmed());
            }
        } else if self.no_color {
            eprintln!("[FAIL] Quine verification failed!");
            if let Some(ref diff) = report.diff_preview {
                eprintln!("{}", diff);
            }
        } else {
            eprintln!(
                "{} {}",
                "✗".red().bold(),
                "Quine verification failed!".red().bold()
            );
            if let Some(ref diff) = report.diff_preview {
                eprintln!("{}", diff.yellow());
            }
        }
    }

    pub fn render_output_code(&self, code: &str) {
        print!("{}", code);
        io::stdout().flush().ok();
    }
}
