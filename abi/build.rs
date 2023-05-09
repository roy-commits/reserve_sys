fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .compile(&["proto/reservation.proto"], &["proto"])
        .unwrap();
    println!("cargo:rerun-if-changed=proto/reservation.proto");
}
