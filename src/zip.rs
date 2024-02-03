use crate::utils::UtilsErr;
use crate::utils::{create_temp_file, make_permanent};
use std::{
    error::Error,
    fmt::{self, Display},
    fs::{self, File},
    io::{self, Write},
    path::Path,
};
use tempfile::TempDir;
use zip::result::ZipError;
use zip::write::{FileOptions, ZipWriter};

#[derive(Debug)]
pub enum ZippingErr {
    TempFileCreationFailed(UtilsErr),
    InvalidFileName,
    FileOpenFailed(io::Error),
    WriteFailed(io::Error),
    FinishFailed(zip::result::ZipError),
    ZipOperationFailed(ZipError),
}

impl fmt::Display for ZippingErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ZippingErr::TempFileCreationFailed(e) => {
                write!(f, "Temporary file creation failed: {}", e)
            }
            ZippingErr::InvalidFileName => write!(f, "Invalid file name provided for zipping."),
            ZippingErr::FileOpenFailed(e) => write!(f, "Failed to open a file for zipping: {}", e),
            ZippingErr::WriteFailed(e) => write!(f, "Failed to write to the zip archive: {}", e),
            ZippingErr::FinishFailed(e) => write!(f, "Failed to finalize the zip archive: {}", e),
            ZippingErr::ZipOperationFailed(e) => write!(f, "Zip operation failed: {}", e),
        }
    }
}

impl From<std::io::Error> for ZippingErr {
    fn from(error: std::io::Error) -> Self {
        ZippingErr::FileOpenFailed(error)
    }
}

impl From<ZipError> for ZippingErr {
    fn from(error: ZipError) -> Self {
        match error {
            ZipError::FileNotFound => ZippingErr::InvalidFileName,
            _ => ZippingErr::ZipOperationFailed(error),
        }
    }
}

pub fn create_zip_archive<P: AsRef<Path>>(files: &[P], output_path: P) -> Result<(), ZippingErr> {
    let (temp_dir, temp_file_path) =
        create_temp_file("zip").map_err(ZippingErr::TempFileCreationFailed)?;
    let temp_file = File::create(&temp_file_path).map_err(ZippingErr::FileOpenFailed)?;

    let mut zip = ZipWriter::new(temp_file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    for file_path in files {
        let file_name = file_path
            .as_ref()
            .file_name()
            .ok_or(ZippingErr::InvalidFileName)?;
        zip.start_file(file_name.to_string_lossy(), options)
            .map_err(|e| ZippingErr::from(e));

        let mut file = File::open(file_path).map_err(ZippingErr::FileOpenFailed)?;
        let mut buffer = Vec::new();
        io::copy(&mut file, &mut buffer).map_err(ZippingErr::WriteFailed)?;
        zip.write_all(&buffer).map_err(ZippingErr::WriteFailed)?;
    }

    zip.finish().map_err(ZippingErr::FinishFailed)?;
    make_permanent(temp_dir, &temp_file_path, output_path.as_ref())
        .map_err(ZippingErr::TempFileCreationFailed)?;
    Ok(())
}
