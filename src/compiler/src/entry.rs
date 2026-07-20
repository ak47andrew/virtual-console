use std::path::Path;
use log::{debug, error, info};
use vea_shared::cartridge::{get_bg_path, Cartridge};
use vea_shared::consts::TARGET_RESOLUTION;
use vea_shared::manifest::Manifest;
use crate::images::{encode_bg, encode_chr};
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
    report_addresses(&cartridge);

    // Step 2. Assemble entry
    let entry_path = Path::new(folder).join(cartridge.manifest.resources.entry.clone());
    let entry_bytecode = assemble_vea(&entry_path, &cartridge.manifest);
    if entry_bytecode.is_none() {
        error!("Error occurred while assembling entry script. Aborting");
        return false;
    }
    let entry_bytecode = entry_bytecode.unwrap();
    info!("Assembled entry script at {} totaling {} bytes", entry_path.display(), entry_bytecode.len());

    cartridge.manifest.resources.entry = "entry.veb".into();
    cartridge.entry_bytecode = entry_bytecode;
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

    // Step 4a. Background encoding
    for (idx, path) in &cartridge.manifest.resources.bg {
        debug!("{} -> {}", idx, path);
        let bg = encode_bg(Path::new(folder).join(path));
        if bg.is_none() {
            error!("Error occurred while encoding background. Aborting");
            return false;
        }
        cartridge.bg_data.insert(*idx, bg.unwrap());
    }
    for idx in &cartridge.manifest.resources.bg.keys().cloned().collect::<Vec<_>>() {
        cartridge.manifest.resources.bg.insert(*idx, get_bg_path(idx));
    }

    // Step 4b. Sprite encoding
    for (idx, path) in &cartridge.manifest.resources.img {
        debug!("{} -> {}", idx, path);
        let sprite = encode_chr(Path::new(folder).join(path));
        if sprite.is_none() {
            error!("Error occurred while encoding sprite. Aborting");
            return false;
        }
        cartridge.sprite_data.insert(*idx, sprite.unwrap());
    }
    for idx in &cartridge.manifest.resources.img.keys().cloned().collect::<Vec<_>>() {
        cartridge.manifest.resources.img.insert(*idx, get_bg_path(idx));
    }

    // Step <last>. Save cartridge
    let filename = folder.to_string() + ".vec";
    cartridge.save(filename.clone());  // Virtual Emulator Cartridge
    info!("Saved cartridge to {:?}", filename);

    true
}

fn report_addresses(cartridge: &Cartridge) {
    let manifest = &cartridge.manifest;
    let vram_size = TARGET_RESOLUTION.x * TARGET_RESOLUTION.y;
    let stack_start = vram_size + manifest.settings.ram_size;
    info!("=== SECTIONS ===");
    info!("VRAM: 0-{}", vram_size);
    info!("RAM: {}-{}", vram_size + 1, vram_size + manifest.settings.ram_size);
    info!("STACK: {}-{}", stack_start, stack_start + manifest.settings.stack_size - 1);
    info!("ROM: {}-...", stack_start + manifest.settings.stack_size);
    info!("=== SPECIAL ADDRESSES ===");
    info!("INPUT_HELD: ${}", stack_start - 2);
    info!("INPUT_PRESSED: ${}", stack_start - 1);
}