use crate::utils::{create_temp_file, make_permanent, UtilsErr};
use bzip2::write::BzEncoder;
use std::{
    error::Error,
    fs::File,
    io::{self, Write},
    path::Path,
};

#[derive(Debug)]
pub enum BzipErr {
    TempFileCreationFailed(UtilsErr),
    CompressionFailed(io::Error),
    TempFileFinalizationFailed(UtilsErr),
}

impl Error for BzipErr {}

impl std::fmt::Display for BzipErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BzipErr::TempFileCreationFailed(e) => {
                write!(f, "Temporary file creation failed: {}", e)
            }
            BzipErr::CompressionFailed(e) => write!(f, "Bzip2 compression failed: {}", e),
            BzipErr::TempFileFinalizationFailed(e) => {
                write!(f, "Failed to finalize the temporary file: {}", e)
            }
        }
    }
}

pub fn create_file_bzip2<P: AsRef<Path>>(input_path: P, output_path: P) -> Result<(), BzipErr> {
    let (temp_dir, temp_file_path) =
        create_temp_file("bz2").map_err(BzipErr::TempFileCreationFailed)?;
    let temp_file = File::create(&temp_file_path).map_err(BzipErr::CompressionFailed)?;
    let mut encoder = BzEncoder::new(temp_file, bzip2::Compression::best());

    let mut input_file = File::open(input_path).map_err(BzipErr::CompressionFailed)?;
    io::copy(&mut input_file, &mut encoder).map_err(BzipErr::CompressionFailed)?;
    encoder.finish().map_err(|_| {
        BzipErr::CompressionFailed(io::Error::new(
            io::ErrorKind::Other,
            "Failed to finish compression",
        ))
    })?;

    make_permanent(temp_dir, &temp_file_path, output_path.as_ref())
        .map_err(BzipErr::TempFileFinalizationFailed)?;
    Ok(())
}
