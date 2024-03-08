use std::process::Command;

fn main() {

    let _output = Command::new("bash")
        .arg("generate_proto.sh")
        .spawn()
        .unwrap();

    embuild::espidf::sysenv::output();
}
