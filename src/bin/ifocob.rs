use std::env;
use std::fs;
use std::process::Command;
use ifocob::resolvers::resolve_decode;
use ifocob::resolvers::resolve_magic;
use tempfile::tempdir;

fn main() {
    let path = env::args().nth(1).expect("usage: ifocob <image>");
    let data = fs::read(&path).expect("could not read file");

    let codec = resolve_magic(&data).expect("Failed to resolve magic");
    let img = resolve_decode(codec, &data)
        .expect("Failed to decode image");
    

    let tmp = tempdir().expect("failed to create temp dir");
    let tmp_path = tmp.path().join("image.png");
    img.save(&tmp_path).expect("could not save temp png");

    let viewer = match std::env::consts::OS {
        "linux" | "freebsd" | "openbsd" | "netbsd" => "xdg-open",
        "macos" => "open",
        "windows" => "start",
        _ => panic!("unsupported OS"),
    };

    Command::new(viewer)
        .arg(&tmp_path)
        .spawn().expect("failed to open")
        .wait().expect("failed to wait");
}