use crate::utils::{create_temp_file, make_permanent, UtilsErr};
use flate2::write::{DeflateEncoder, GzEncoder, ZlibEncoder};
use flate2::Compression;
use std::{
    error::Error,
    fmt::{self, Display},
    fs::{self, File},
    io::{self, Write},
    path::Path,
};
use tempfile::{tempdir, TempDir};

#[derive(Debug)]
pub enum GzipErr {
    TempFileCreationFailed(UtilsErr),
    InvalidOutputFileName,
    FileOpenFailed(io::Error),
    CompressionFailed(io::Error),
    TempFileFinalizationFailed(UtilsErr),
}

impl fmt::Display for GzipErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GzipErr::TempFileCreationFailed(e) => {
                write!(f, "Temporary file creation failed: {}", e)
            }
            GzipErr::InvalidOutputFileName => write!(f, "Invalid output file name."),
            GzipErr::FileOpenFailed(e) => {
                write!(f, "Failed to open a file for gzip compression: {}", e)
            }
            GzipErr::CompressionFailed(e) => write!(f, "Gzip compression failed: {}", e),
            GzipErr::TempFileFinalizationFailed(e) => {
                write!(f, "Failed to finalize the temporary gzip file: {}", e)
            }
        }
    }
}

impl Error for GzipErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GzipErr::TempFileCreationFailed(e) => Some(e),
            GzipErr::FileOpenFailed(e) => Some(e),
            GzipErr::CompressionFailed(e) => Some(e),
            GzipErr::TempFileFinalizationFailed(e) => Some(e),
            _ => None,
        }
    }
}

pub fn create_gzip_archive<P: AsRef<Path>>(file_path: P, output_path: P) -> Result<(), GzipErr> {
    let (temp_dir, temp_file_path) =
        create_temp_file("gz").map_err(GzipErr::TempFileCreationFailed)?;
    let temp_file = File::create(&temp_file_path).map_err(GzipErr::FileOpenFailed)?;

    let mut encoder = GzEncoder::new(temp_file, Compression::best());
    let mut file = File::open(&file_path).map_err(GzipErr::FileOpenFailed)?;
    io::copy(&mut file, &mut encoder).map_err(GzipErr::CompressionFailed)?;
    encoder
        .finish()
        .map_err(|e| GzipErr::CompressionFailed(io::Error::new(io::ErrorKind::Other, e)))?;

    make_permanent(temp_dir, &temp_file_path, output_path.as_ref())
        .map_err(GzipErr::TempFileFinalizationFailed)?;
    Ok(())
}

pub fn encode_file_deflate<P: AsRef<Path>>(file_path: P, output_path: P) -> Result<(), GzipErr> {
    let output_path_ref = output_path.as_ref();
    let temp_file_extension = output_path_ref
        .extension()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or_default();
    let (temp_dir, temp_file_path) =
        create_temp_file(temp_file_extension).map_err(GzipErr::TempFileCreationFailed)?;
    let temp_file = File::create(&temp_file_path).map_err(GzipErr::FileOpenFailed)?;

    let mut encoder = ZlibEncoder::new(temp_file, Compression::best());
    let mut file = File::open(&file_path).map_err(GzipErr::FileOpenFailed)?;
    io::copy(&mut file, &mut encoder).map_err(GzipErr::CompressionFailed)?;
    encoder
        .finish()
        .map_err(|e| GzipErr::CompressionFailed(io::Error::new(io::ErrorKind::Other, e)))?;

    make_permanent(temp_dir, &temp_file_path, output_path.as_ref())
        .map_err(GzipErr::TempFileFinalizationFailed)?;
    Ok(())
}

pub fn encode_file_zlib<P: AsRef<Path>>(file_path: P, output_path: P) -> Result<(), GzipErr> {
    let output_path_ref = output_path.as_ref();
    let temp_file_extension = output_path_ref
        .extension()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or_default();
    let (temp_dir, temp_file_path) =
        create_temp_file(temp_file_extension).map_err(GzipErr::TempFileCreationFailed)?;
    let temp_file = File::create(&temp_file_path).map_err(GzipErr::FileOpenFailed)?;

    let mut encoder = ZlibEncoder::new(temp_file, Compression::best());
    let mut file = File::open(&file_path).map_err(GzipErr::FileOpenFailed)?;
    io::copy(&mut file, &mut encoder).map_err(GzipErr::CompressionFailed)?;
    encoder
        .finish()
        .map_err(|e| GzipErr::CompressionFailed(io::Error::new(io::ErrorKind::Other, e)))?;

    make_permanent(temp_dir, &temp_file_path, output_path.as_ref())
        .map_err(GzipErr::TempFileFinalizationFailed)?;
    Ok(())
}
