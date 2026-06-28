use std::fs::{remove_file, File};
use std::path::{Path, PathBuf};
use std::process::{exit, Command, Stdio};
use notify_rust::Notification;
use raylib::prelude::Texture2D;
use raylib::{RaylibHandle, RaylibThread};
use vea_shared::manifest::Manifest;

pub struct Panel {
    pub name: String,
    pub version: String,
    pub path: PathBuf,
    pub logo: Option<Texture2D>
}

impl Panel {
    pub fn new(name: String, version: String, path: PathBuf, logo: Option<Texture2D>) -> Panel {
        Panel { name, version, path, logo }
    }

    pub fn from_path(path: PathBuf, raylib_handle: &mut RaylibHandle, raylib_thread: &mut RaylibThread) -> Option<Panel> {
        let manifest = Manifest::from_file(&path.join("manifest.toml"))?;

        let name = manifest.clone().metadata.unwrap_or_default().name.unwrap_or("Unnamed game".to_string());
        let version = manifest.metadata.unwrap_or_default().version.unwrap_or("v1.0.0".to_string());

        let logo = match raylib_handle.load_texture(&raylib_thread, path.join("logo.png").to_str().unwrap()) {
            Ok(logo) => Some(logo),
            Err(_) => {None}
        };

        Some(Panel::new(name, version, path, logo))
    }
}

pub fn get_dist_path() -> PathBuf {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
        .join("dist")
}

pub fn build(path: PathBuf) {
    let runner = get_dist_path().join(
        if cfg!(target_os = "windows") {
            "vea_compiler.exe"
        } else {
            "vea_compiler"
        }
    );

    let log_file = File::create("build.log").unwrap();
    let stderr = log_file.try_clone().unwrap();

    let status = Command::new(runner)
        .arg(path.clone())
        .stdout(Stdio::from(log_file))
        .stderr(Stdio::from(stderr))
        .status()
        .unwrap();

    if status.success() {
        let _ = Notification::new()
            .summary("Build Complete successfully!")
            .body(format!("Successfully built game at path {}!", path.display()).as_str())
            .show();

        let _ = remove_file("build.log");
    } else {
        let _ = Notification::new()
            .summary("Build failed...")
            .body(format!("Game at path {} failed to build. Check `build.log` for more information", path.display()).as_str())
            .show();
    }
}

pub fn run(path: PathBuf) {
    let runner = get_dist_path().join(
        if cfg!(target_os = "windows") {
            "vea_runtime.exe"
        } else {
            "vea_runtime"
        }
    );

    Command::new(runner)
        .arg(path)
        .spawn()
        .unwrap();

    exit(0);
}
