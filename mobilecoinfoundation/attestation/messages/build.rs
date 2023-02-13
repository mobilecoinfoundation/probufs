// Copyright (c) 2023 The MobileCoin Foundation

//! Generate the prost messages for the gRPC service.

use std::{
    env,
    io::{Error, ErrorKind},
    path::PathBuf,
    process::Command,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let protoc = prost_build::protoc_from_env();
    let mut cmd = Command::new(protoc);
    cmd.arg(format!("--prost_out={}", out_dir.display()));

    let protos = &["../v1/attest.proto"];
    let includes = &["../../../", "../"];

    for i in includes {
        cmd.arg("-I").arg(i);
    }

    for p in protos {
        cmd.arg(p);
    }

    let output = cmd.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::new(
            ErrorKind::Other,
            format!("Failed building gRPC messages: {stderr}"),
        ))?;
    }

    Ok(())
}
