use std::io::Write;
use std::{env, fs};
use std::process::ExitCode;
use env_logger::fmt::style::{AnsiColor, Color, Style};
use log::Level;
use crate::entry::entry_compiler;

mod entry;
pub mod errors;
pub mod vea;
pub mod palette;

fn main() -> ExitCode {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info")
    )
        .format(|buf, record| {
            let level_style = buf.default_level_style(record.level());
            let secondary_style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::BrightBlack)));
            let ts = buf.timestamp();
            const RESET_STYLE: &str = "\x1B[0m";

            writeln!(buf, "[{secondary_style}{}{RESET_STYLE}][{level_style}{}{RESET_STYLE}]: {}", ts, record.level().as_str(), record.args())
        })
        .init();

    let mut args = env::args().collect::<Vec<String>>();
    if args.len() != 2 || !fs::exists(args[1].clone()).unwrap_or_else(|_| false) {
        eprintln!("Usage: {} <source folder>", args[0]);
        return ExitCode::FAILURE;
    }
    if entry_compiler(args[1].as_str()) { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}
