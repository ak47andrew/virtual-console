use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Write};
use serde::de::DeserializeOwned;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};
use crate::manifest::Manifest;

#[derive(Debug, Clone)]
pub struct Cartridge {
    pub manifest: Manifest,
    pub entry_bytecode: Vec<u8>,
    pub palette: Vec<u8>,
    pub bg_data: BTreeMap<u32, Vec<u8>>,
    pub sprite_data: BTreeMap<u32, Vec<u8>>,
}

fn load_file(archive: &mut ZipArchive<File>, filename: &str) -> Vec<u8> {
    let mut file = archive.by_name(filename).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    contents
}

fn parse_from_zip<T: DeserializeOwned>(archive: &mut ZipArchive<File>, filename: &str) -> T {
    let data = load_file(archive, filename);
    toml::from_slice(data.as_slice()).unwrap()
}

pub fn get_bg_path(ind: &u32) -> String {
    format!("bg/{}.bg.bin", ind)
}

pub fn get_spr_path(ind: &u32) -> String {
    format!("sprite/{}.sprite.bin", ind)
}

impl Cartridge {
    pub fn new(manifest: Manifest) -> Cartridge {
        Cartridge { manifest, entry_bytecode: vec![], palette: vec![], bg_data: BTreeMap::new(), sprite_data: BTreeMap::new() }
    }
    
    /// The caller is responsible for validation of the filename, or we'll get a panic
    /// It's also assumed that file is a valid cartridge or... well we're fucked
    pub fn load(filename: String) -> Cartridge {
        let file = File::open(filename).unwrap();
        let mut archive = ZipArchive::new(file).unwrap();

        let manifest = parse_from_zip::<Manifest>(&mut archive, "manifest.toml");
        let entry_bytecode = load_file(&mut archive, manifest.resources.entry.as_str());
        let palette = load_file(&mut archive, manifest.resources.palette.as_str());

        let mut bg_data: BTreeMap<u32, Vec<u8>> = BTreeMap::new();
        for (ind, _) in &manifest.resources.bg {
            bg_data.insert(*ind, load_file(&mut archive, get_bg_path(&ind).as_str()));
        }

        let mut sprite_data: BTreeMap<u32, Vec<u8>> = BTreeMap::new();
        for (ind, _) in &manifest.resources.bg {
            sprite_data.insert(*ind, load_file(&mut archive, get_spr_path(&ind).as_str()));
        }

        Self { manifest, entry_bytecode, palette, bg_data, sprite_data }
    }

    pub fn save(&self, filename: String) {
        let file = File::create(filename).unwrap();
        let mut zip = ZipWriter::new(file);

        let options = SimpleFileOptions::default()
            .compression_method(CompressionMethod::DEFLATE);

        zip.start_file("manifest.toml", options).unwrap();
        zip.write_all(self.manifest.to_string().as_bytes()).unwrap();

        // It's expected that compiler is gonna change the entry name to entry.veb and store it there
        // after compilation, but it's gonna work either way. because of `self.manifest.entry` here.
        // This convention is purely for geeks unarchiving the fuck out of the cartridge
        zip.start_file(&self.manifest.resources.entry, options).unwrap();
        zip.write_all(self.entry_bytecode.as_slice()).unwrap();

        // Same here, but with palette.rfe, where rfe stands for "random fucking extension".
        // I could use like .aco, tho I'm not sure if it's the same thing
        zip.start_file(&self.manifest.resources.palette.as_str(), options).unwrap();
        zip.write_all(self.palette.as_slice()).unwrap();

        // Packing backgrounds
        for (ind, data) in &self.bg_data {
            zip.start_file(get_bg_path(ind), options).unwrap();
            zip.write_all(data.as_slice()).unwrap();
        }

        // Packing sprites
        for (ind, data) in &self.sprite_data {
            zip.start_file(get_spr_path(ind), options).unwrap();
            zip.write_all(data.as_slice()).unwrap();
        }

        zip.finish().unwrap();
    }
}
