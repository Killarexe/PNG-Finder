use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The file input.
    pub file_input: PathBuf,
    /// Set a custom folder name (by default is the name of the file).
    output_folder: Option<PathBuf>
}

impl Args {
    pub fn get_dir_output(&self) -> PathBuf {
        if let Some(output_folder) = &self.output_folder {
            output_folder.to_path_buf()
        } else {
            self.file_input.with_extension("")
        }
    }
}
