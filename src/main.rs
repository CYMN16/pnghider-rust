use std::fs;
use std::str::FromStr;

use chunk::Chunk;
use chunk_type::ChunkType;
use clap::Parser;
use png::Png;

use crate::args::Args;
use crate::commands::Commands;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let cli = Args::parse();

    match &cli.command {
        Some(Commands::Encode {
            filepath,
            chunk_type,
            message,
            output_path,
        }) => {
            //println!("{:?}", filepath);
            //println!("{:?}", chunk_type);
            //println!("{:?}", message);
            //println!("{:?}", output_path);

            let png_file: &[u8] = &fs::read(filepath).unwrap();
            //println!("{:?}", png_file)
            let mut png = Png::try_from(png_file).unwrap();
            println!("{:?}", png);
            let chunk = Chunk::new(ChunkType::from_str(chunk_type).unwrap(), message.as_bytes().to_vec());
            png.append_chunk(chunk);
            
            match output_path {
                Some(path) => {fs::write(path, png.as_bytes()).unwrap();},
            
                None => {fs::write("./test/output.png", png.as_bytes()).unwrap();},
            }
        }
        Some(Commands::Decode {
            filepath,
            chunk_type,
        }) => {
            //println!("{:?}", filepath);
            //println!("{:?}", chunk_type);
            let png_file: &[u8] = &fs::read(filepath).unwrap();
            let png = Png::try_from(png_file).unwrap();


            //let msg_loc = ChunkType::from_str(chunk_type).unwrap();
            let hidden_message = png.chunk_by_type(chunk_type).unwrap();
            println!("{:?}", hidden_message.to_string())
        }
        Some(Commands::Remove {
            filepath,
            chunk_type,
        }) => {
            println!("{:?}", filepath);
            println!("{:?}", chunk_type); 

        }
        Some(Commands::Print { filepath }) => {
            println!("{:?}", filepath);
        }
        None => {}
    }
}
