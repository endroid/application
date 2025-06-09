use std::env;
use std::path::PathBuf;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let proto_dir = "proto";
    
    // Find all .proto files in the proto directory
    let mut proto_files = Vec::new();
    for entry in fs::read_dir(proto_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "proto") {
            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    proto_files.push(format!("{}/{}", proto_dir, file_name_str));
                }
            }
        }
    }
    
    if proto_files.is_empty() {
        return Err("No .proto files found in the proto directory".into());
    }
    
    println!("Found proto files: {:?}", proto_files);
    
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("descriptor.bin"))
        .compile_protos(
            &proto_files,
            &[proto_dir]
        )?;

    Ok(())
}
