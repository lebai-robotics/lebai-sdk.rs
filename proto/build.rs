use std::path::PathBuf;
use std::{env, fs, io};

fn main() -> io::Result<()> {
    let descriptor_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("proto_descriptor.bin");
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("lebai-proto");
    let proto_files = vec![
        root.join("lebai.proto"),
        root.join("cmp.proto"),
        root.join("db.proto"),
        root.join("file.proto"),
        root.join("posture.proto"),
        root.join("motor.proto"),
        root.join("system.proto"),
        root.join("auto.proto"),
        root.join("storage.proto"),
        root.join("structure.proto"),
        root.join("dynamic.proto"),
        root.join("kinematic.proto"),
        root.join("motion.proto"),
        root.join("safety.proto"),
        root.join("task.proto"),
        root.join("plugin.proto"),
        root.join("trigger.proto"),
        root.join("signal.proto"),
        root.join("io.proto"),
        root.join("serial.proto"),
        root.join("modbus.proto"),
        root.join("led.proto"),
        root.join("flange.proto"),
        root.join("claw.proto"),
        root.join("backup.proto"),
        root.join("hardware.proto"),
        root.join("multi_devices.proto"),
    ];
    for proto_file in &proto_files {
        println!("cargo:rerun-if-changed={}", proto_file.display());
    }

    tonic_prost_build::configure()
        .build_client(false)
        .build_server(false)
        .file_descriptor_set_path(&descriptor_path)
        .compile_well_known_types(true)
        .extern_path(".google.protobuf", "::pbjson_types")
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile_protos(&proto_files, &[root])?;

    let descriptor_set = fs::read(descriptor_path)?;
    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_set)?
        .ignore_unknown_fields()
        .emit_fields()
        .build(&["."])?;
    pbjsonrpc_build::Builder::new()
        .register_descriptors(&descriptor_set)?
        .client()
        .build(&["."])?;
    Ok(())
}
