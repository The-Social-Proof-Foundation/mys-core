// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;
use prost::Message;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let proto_dir = root_dir.join("proto");
    let proto_ext = OsStr::new("proto");
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let mut proto_files = vec![];
    for entry in walkdir::WalkDir::new(&proto_dir) {
        let entry = entry?;
        if entry.file_type().is_dir() {
            continue;
        }

        let path = entry.into_path();
        if path.extension() == Some(proto_ext) {
            proto_files.push(path)
        }
    }

    let mut fds = protox::Compiler::new(std::slice::from_ref(&proto_dir))?
        .include_source_info(true)
        .include_imports(true)
        .open_files(&proto_files)?
        .file_descriptor_set();

    // Sort files by name to have deterministic codegen output
    fds.file.sort_by(|a, b| a.name.cmp(&b.name));

    tonic_prost_build::configure()
        .build_client(true)
        .build_server(true)
        .bytes(".")
        .out_dir(&out_dir)
        .compile_fds(fds.clone())?;

    // Group the files by their package, in order to have a single fds file per package
    let mut packages: HashMap<_, prost_types::FileDescriptorSet> = HashMap::new();
    for mut file in fds.file {
        // Clear out the source code info as its not required for reflection
        file.source_code_info = None;
        packages
            .entry(file.package().to_owned())
            .or_default()
            .file
            .push(file);
    }

    for (package, fds) in packages {
        let file_name = format!("{package}.fds.bin");
        let file_descriptor_set_path = out_dir.join(&file_name);
        std::fs::write(file_descriptor_set_path, Message::encode_to_vec(&fds))?;
    }

    println!("cargo:rerun-if-changed=proto");
    Ok(())
}
