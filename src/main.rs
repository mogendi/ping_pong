#![feature(drain_filter)]

mod args;
mod chunk;
mod chunk_type;
mod png;

use args::PngArgs;
use std::fs;
use structopt::StructOpt;

fn main() {
    let args: PngArgs = PngArgs::from_args();
    match args{
        PngArgs::Encode(enc) => {
            let png = enc.process_req();
            if let Some(output_file) = enc.output_file {
                fs::write(output_file, png.as_bytes()).unwrap();
            } else {
                fs::write(enc.file_path, png.as_bytes()).unwrap();
            }
        }
        PngArgs::Decode(dec) => {
            println!("{}", dec.process_req());
        }
        PngArgs::Remove(rem) => {
            rem.process_req();
        }
        PngArgs::Print(prnt) => {
            prnt.process_req();
        }
    }
}
