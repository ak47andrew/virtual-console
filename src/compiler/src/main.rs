use std::io::Write;
use std::{env, fs};
use std::path::Path;
use std::process::ExitCode;
use env_logger::fmt::style::{AnsiColor, Color, Style};
use vea_shared::manifest::Manifest;
use crate::entry::entry_compiler;

mod entry;
pub mod errors;
pub mod vea;
pub mod palette;
pub mod images;

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

    let compile = args.len() == 2 && fs::exists(args[1].clone()).unwrap_or_else(|_| false);
    let new_ = args.len() == 3 && args[1] == "new";
    if !(compile || new_) {
        eprintln!("Usage: {} <source folder|new>", args[0]);
        eprintln!("Use `{} new` to create new template", args[0]);
        return ExitCode::FAILURE;
    }
    if new_ {
        let path = Path::new(args[2].as_str());

        fs::create_dir_all(&args[2]).unwrap();
        fs::create_dir_all(path.join("bg")).unwrap();
        fs::create_dir_all(path.join("img")).unwrap();

        let default_manifest = Manifest::default(args[2].clone());
        let manifest = path.join("manifest.toml");
        fs::write(manifest, default_manifest.to_string()).unwrap();

        let source = path.join("source.vea");
        fs::write(source, "; TODO: write something").unwrap();

        let palette = path.join("palette.pal");
        fs::write(palette, "JASC-PAL\n0100\n1\n0 0 0 0").unwrap();

        ExitCode::SUCCESS
    } else {
        if entry_compiler(args[1].as_str()) { ExitCode::SUCCESS } else { ExitCode::FAILURE }
    }
}
