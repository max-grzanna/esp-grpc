use std::io::Result;
//use std::path::PathBuf;
//use std::process::Command;

fn main() -> Result<()> {
    prost_build::compile_protos(&["proto/message.proto"], &["proto/"])?;

    //     let _output = Command::new("bash")
    //         .arg("generate_proto.sh")
    //         .spawn()
    //         .unwrap();
    //
    //
    //     //let include_path =  "/proto/generated/proto/calculator.pb.h";
    //     let proto_bindings = bindgen::Builder::default()
    //         .header("proto_wrapper.h")
    //         .clang_args(&[
    //             "-I/proto/generated/proto/pb.h",
    //         ])
    //         .generate()
    //         .expect("Unable to generate bindings");
    //
    //     let proto_out_path = PathBuf::from("proto/generated/");
    //     proto_bindings
    //         .write_to_file(proto_out_path.join("proto_bindings.rs"))
    //         .expect("Couldn't write proto bindings!");
    //
    //     let header_path = "wrapper.h";
    //
    //     let bindings = bindgen::Builder::default()
    //         .header(header_path)
    //         .clang_args(&[
    //             "-I/components/nanopb/pb_encode.h",
    //             "-I/components/nanopb/pb_decode.h",
    //             "-I/components/nanopb/pb_common.h",
    //         ])
    //         .allowlist_function("pb_encode")
    //         .generate()
    //         .expect("Unable to generate bindings");
    //
    //     let out_path = PathBuf::from("bindings/");
    //
    //     bindings
    //         .write_to_file(out_path.join("bindings.rs"))
    //         .expect("Couldn't write bindings!");
    //
    embuild::espidf::sysenv::output();
    Ok(())
}
