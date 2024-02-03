use crate::utils::{create_temp_file, make_permanent, UtilsErr};
use std::{
    error::Error,
    fs::File,
    io::{self, Write},
    path::Path,
};
use xz2::write::XzEncoder;

#[derive(Debug)]
pub enum Xz {
    TempFileCreationFailed(UtilsErr),
    CompressionFailed(io::Error),
    TempFileFinalizationFailed(UtilsErr),
}

impl Error for Xz {}

impl std::fmt::Display for Xz {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Xz::TempFileCreationFailed(e) => {
                write!(f, "Temporary file creation failed: {}", e)
            }
            Xz::CompressionFailed(e) => write!(f, "Bzip2 compression failed: {}", e),
            Xz::TempFileFinalizationFailed(e) => {
                write!(f, "Failed to finalize the temporary file: {}", e)
            }
        }
    }
}

pub fn create_file_xz<P: AsRef<Path>>(input_path: P, output_path: P) -> Result<(), Xz> {
    let (temp_dir, temp_file_path) = create_temp_file("xz").map_err(Xz::TempFileCreationFailed)?;
    let temp_file = File::create(&temp_file_path).map_err(Xz::CompressionFailed)?;
    let mut encoder = XzEncoder::new(temp_file, 9);

    let mut input_file = File::open(input_path).map_err(Xz::CompressionFailed)?;
    io::copy(&mut input_file, &mut encoder).map_err(Xz::CompressionFailed)?;
    encoder.finish().map_err(|_| {
        Xz::CompressionFailed(io::Error::new(
            io::ErrorKind::Other,
            "Failed to finish compression",
        ))
    })?;

    make_permanent(temp_dir, &temp_file_path, output_path.as_ref())
        .map_err(Xz::TempFileFinalizationFailed)?;
    Ok(())
}
