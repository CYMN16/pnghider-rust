use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum Commands {
    /// Encodes the hidden message into a png
    Encode {
        /// input filepath to the png
        filepath: String,
        /// 4 byte long chunk type
        chunk_type: String,
        /// the message to be hidden
        message: String,
        /// output path for the operation
        #[arg(short, long)]
        output_path: Option<PathBuf>,
    },

    /// Decodes the hidden message with the given chunk_type
    Decode {
        /// input filepath to the png
        filepath: String,
        /// 4 byte long chunk type to search for
        chunk_type: String,
    },

    /// Removes the chunk given its chunk_type
    Remove {
        /// input filepath to the png
        filepath: String,
        /// 4 byte long chunk type to search and remove
        chunk_type: String,
    },

    /// Prints the image
    Print {
        /// input filepath to the png
        filepath: String,
    },
}
