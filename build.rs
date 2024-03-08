use std::path::PathBuf;
use std::process::Command;

fn main() {
    let _output = Command::new("bash")
        .arg("generate_proto.sh")
        .spawn()
        .unwrap();

    let header_path = "wrapper.h";

    let bindings = bindgen::Builder::default()
        .header(header_path)
        .clang_args(&[
            "-I/components/nanopb/pb_encode.h",
            "-I/components/nanopb/pb_decode.h",
            "-I/components/nanopb/pb_common.h",
        ])
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("bindings/");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    embuild::espidf::sysenv::output();
}
