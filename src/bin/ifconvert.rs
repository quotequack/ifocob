use std::env;
use std::fs;
use ifocob::resolvers::resolve_encode;
use ifocob::resolvers::resolve_name;

fn main() {
    let mut args = env::args().skip(1);
    let input  = args.next().expect("usage: mfp convert <input> <codec> <output.mfp>");
    let codec  = args.next().expect("codec required");
    let output = args.next().expect("output path required");

    let img = image::open(&input).expect("could not open image");

    let codec = resolve_name(codec);

    let encoded = resolve_encode(codec, &img).expect("failed to encode");

    fs::write(&output, encoded).expect("could not write output");

    println!("saved {}", output);
}