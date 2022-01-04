use std::path::PathBuf;
use structopt::StructOpt;

use crate::chunk::Chunk;
use crate::png::Png;
use std::str::from_utf8;

/// EncodeArgs options
#[derive(StructOpt, Debug)]
#[structopt(name="PiNG_PoNG")]
pub enum PngArgs{
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(Remove),
    Print(Print),
} 

#[derive(StructOpt, Debug)]
/// Add hidden message to PNG file
pub struct EncodeArgs {
    /// The origin PNG that you want encoded with
    /// a message
    #[structopt(short, long)]
    pub file_path: PathBuf,

    /// 4 letter, valid PNG chunk type code
    #[structopt(short, long)]
    pub chunk_type: String,

    /// The message you want encoded into the
    /// PNG file
    #[structopt(short, long)]
    pub message: String,

    /// Optional output file if you dont want the origin to be overwritten
    #[structopt(short, long)]
    pub output_file: Option<PathBuf>,
}

impl EncodeArgs {
    // process any call to Encode a message
    pub fn process_req(&self) -> Png{
        match Chunk::new_no_state(
            self.chunk_type.clone(), 
            self.message.as_bytes().to_vec()) {
                Ok(chunk) => {
                    match Png::from_file(self.file_path.clone()) {
                        Ok(mut png) => {
                            png.append_chunk(chunk);
                            return png
                        }
                        Err(_) => {
                            panic!("Failed to read png file, is the file formatted as a png?");
                        } 
                    }
                }
                Err(_) => {
                    panic!("Invalid chunk type format. Check the png docs for the proper chunk type formatting");
                }
        }
    } 
}

#[derive(StructOpt, Debug)]
/// Decode message from specific chunk
pub struct DecodeArgs {
    /// The PNG with the hidden message
    #[structopt(short, long)]
    pub file_path: PathBuf,
   
    /// 4 letter valid PNG chunk type code
    /// that contains the hidden message
    #[structopt(short, long)]
    pub chunk_type: String,
}

impl DecodeArgs {
    pub fn process_req(&self) -> String {
        match Png::from_file(self.file_path.clone()) {
            Ok(png) => {
                let chunk = png.chunk_by_type(&self.chunk_type[..]).unwrap();
                return chunk.data_as_string().unwrap();
            }
            Err(_) => {
                panic!("Failed to load png from file");
            }
        }
    }
}

#[derive(StructOpt, Debug)]
/// Remove encoded chunk from PNG file
pub struct Remove {
    /// The file containing the encoded message
    #[structopt(short, long)]
    pub file_path: PathBuf,

    /// The chunk type containing the cnoded message
    #[structopt(short, long)]
    pub chunk_type: String,
}

impl Remove {
    pub fn process_req(&self) -> bool {
        let mut png: Png = Png::from_file(self.file_path.clone()).unwrap();
        png.remove_chunk(&self.chunk_type[..]).unwrap();
        return true
    }
}


#[derive(StructOpt, Debug)]
/// Print the encoded message in the PNG file
pub struct Print {
    /// The PNG file containing the encoded message
    #[structopt(short, long)]
    pub file_path: PathBuf,
}


impl Print {
    pub fn process_req(&self) {
        let png: Png = Png::from_file(self.file_path.clone()).unwrap();
        for chunk in png.chunks().iter() {
            match from_utf8(chunk.data()) {
                Ok(fstr) => {
                    println!("{}", fstr);
                }
                Err(_) => {}
            }
        }
    }
}
