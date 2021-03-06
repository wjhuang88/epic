use std::path::Path;
use npm_scripts::NpmScripts;
use bytes::{BytesMut, BufMut};

const FILE_HEAD: &str =
r"// Generated by build.rs. DO NOT modify.

use std::collections::HashMap;

pub struct GenPages<'s> {
    pub map: HashMap<&'s str, &'s str>
}
impl <'s> GenPages<'s> {
    pub fn new() -> Self {
        let mut map = HashMap::new();
";
const FILE_END: &str =
r"
        Self {map}
    }
}";

fn main() -> std::io::Result<()> {
    let npm = NpmScripts::new("pages");
    if !npm.is_available() {
        println!("npm not found");
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "npm not found"));
    }
    npm.run_script("build").map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e))?;
    let manifest = Path::new("pages/build/asset-manifest.json");
    let manifest_data = std::fs::read(manifest)?;
    let manifest_json: serde_json::Value = serde_json::from_slice(&manifest_data)?;
    let files = manifest_json["files"].as_object().unwrap();

    let gen = Path::new("src/gen___pages.rs");

    let mut content = BytesMut::from(FILE_HEAD);

    for (k, v) in files {
        let path = v.as_str().unwrap();
        let data = &std::fs::read(format!("pages/build{}", path))?;
        content.put_slice(b"\n        // data form ");
        content.put_slice(k.as_bytes());
        content.put_slice(b"\n        map.insert(\"");
        content.put_slice(path.as_bytes());
        content.put_slice(b"\", r###\"");
        content.put_slice(data);
        content.put_slice(b"\"###);\n");
    }

    {
        let path = "/manifest.json";
        let data = &std::fs::read(format!("pages/build{}", path))?;
        content.put_slice(b"\n        // data form ");
        content.put_slice(b"manifest.json");
        content.put_slice(b"\n        map.insert(\"");
        content.put_slice(path.as_bytes());
        content.put_slice(b"\", r###\"");
        content.put_slice(data);
        content.put_slice(b"\"###);\n");
    }

    content.put_slice(FILE_END.as_bytes());

    std::fs::write(gen, content)?;
    Ok(())
}