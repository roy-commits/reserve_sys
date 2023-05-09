use std::process::Command;

fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .compile(&["proto/reservation.proto"], &["proto"])
        .unwrap();
    // add cargo fmt command
    Command::new("cargo").arg(&["fmt"]).output().unwrap();
    println!("cargo:rerun-if-changed=proto/reservation.proto");
}
