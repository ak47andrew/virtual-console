use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=opcodes/");
    let docs_dir = Path::new("opcodes");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("opcodes.rs");

    let mut arms = String::new();

    for entry in fs::read_dir(docs_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let opcode = path.file_stem().unwrap().to_str().unwrap().to_lowercase();
        let abs_path = fs::canonicalize(&path).unwrap();
        arms.push_str(&format!(
            "\t\t\"{opcode}\" => Some(include_str!(\"{}\")),\n",
            abs_path.display()
        ));
    }

    let code = format!(
        "fn get_opcode_docs(opcode: &str) -> Option<&'static str> {{\n    match opcode {{\n{arms}        _ => None,\n    }}\n}}"
    );

    fs::write(out_path, code).unwrap();
}