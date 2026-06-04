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
}

fn load_file(archive: &mut ZipArchive<File>, filename: &str) -> Vec<u8> {
    let mut manifest_file = archive.by_name(filename).unwrap();
    let mut contents = Vec::new();
    manifest_file.read_to_end(&mut contents).unwrap();
    contents
}

fn parse_from_zip<T: DeserializeOwned>(archive: &mut ZipArchive<File>, filename: &str) -> T {
    let data = load_file(archive, filename);
    toml::from_slice(data.as_slice()).unwrap()
}

impl Cartridge {
    pub fn new(manifest: Manifest) -> Cartridge {
        Cartridge { manifest, entry_bytecode: vec![] }
    }
    
    /// The caller is responsible for validation of the filename, or we'll get a panic
    /// It's also assumed that file is a valid cartridge or... well we're fucked
    pub fn load(filename: String) -> Cartridge {
        let file = File::open(filename).unwrap();
        let mut archive = ZipArchive::new(file).unwrap();

        let manifest = parse_from_zip::<Manifest>(&mut archive, "manifest.toml");
        let entry_bytecode = load_file(&mut archive, manifest.entry.as_str());

        Self { manifest, entry_bytecode }
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
        zip.start_file(&self.manifest.entry, options).unwrap();
        zip.write_all(self.entry_bytecode.as_slice()).unwrap();

        zip.finish().unwrap();
    }
}
