use std::path::Path;
use log::{debug, error, info};
use vea_shared::cartridge::Cartridge;
use vea_shared::manifest::Manifest;
use crate::palette::parse_palette;
use crate::vea::assemble_vea;

pub fn entry_compiler(folder: &str) -> bool {
    info!("Compiling project folder \"{}\"...", folder);

    // Step 1. Parse manifest
    let manifest_filepath = Path::new(folder).join("manifest.toml");
    let manifest = Manifest::from_file(&manifest_filepath);
    if manifest.is_none() {
        error!("Failed to parse manifest file: {}. Aborting", manifest_filepath.display());
        return false;
    }
    info!("Parsed manifest at {}", manifest_filepath.display());
    let mut cartridge = Cartridge::new(manifest.unwrap());
    debug!("Prepared cartridge with manifest data");

    // Step 2. Assemble entry
    let entry_path = Path::new(folder).join(cartridge.manifest.resources.entry.clone());
    let entry_bytecode = assemble_vea(&entry_path, &cartridge.manifest);
    if entry_bytecode.is_none() {
        error!("Error occurred while assembling entry script. Aborting");
        return false;
    }
    info!("Assembled entry script at {} totaling {} bytes", entry_path.display(), entry_bytecode.as_ref().unwrap().len());

    cartridge.manifest.resources.entry = "entry.veb".into();
    cartridge.entry_bytecode = entry_bytecode.unwrap();
    debug!("Inserted entry script into cartridge");

    // Step 3. Convert palette
    let palette_path = Path::new(folder).join(cartridge.manifest.resources.palette.clone());
    let palette = parse_palette(&palette_path);
    if palette.is_none() {
        error!("Error occurred while converting palette. Aborting");
        return false;
    }
    info!("Converted palette at {}", palette_path.display());

    cartridge.manifest.resources.palette = "palette.rfn".into();
    cartridge.palette = palette.unwrap();

    // Step <last>. Save cartridge
    let filename = folder.to_string() + ".vec";
    cartridge.save(filename.clone());  // Virtual Emulator Cartridge
    info!("Saved cartridge to {:?}", filename);

    true
}
