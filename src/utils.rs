// utils.rs
use std::{
    error::Error,
    fmt,
    fs::{self, DirBuilder, File},
    io,
    path::{Path, PathBuf},
};
use tempfile::{tempdir, tempfile, TempDir};

#[derive(Debug)]
pub enum UtilsErr {
    TempFileCreationFailed(io::Error),
    InvalidOutputFileName,
    FileRenameFailed(io::Error),
    TempDirCloseFailed(io::Error),
    DirectoryCreationFailed(io::Error),
    FileCopyFailed(io::Error),
}

impl fmt::Display for UtilsErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UtilsErr::TempFileCreationFailed(_) => write!(f, "Failed to create a temporary file."),
            UtilsErr::InvalidOutputFileName => write!(f, "Invalid output file name."),
            UtilsErr::FileRenameFailed(_) => {
                write!(f, "Failed to rename the temporary file to the output path.")
            }
            UtilsErr::TempDirCloseFailed(_) => {
                write!(f, "Failed to close and remove the temporary directory.")
            }
            UtilsErr::DirectoryCreationFailed(_) => {
                write!(f, "Failed to create the target directory.")
            }
            UtilsErr::FileCopyFailed(_) => {
                write!(f, "Failed to copy file to the target directory.")
            }
        }
    }
}

impl Error for UtilsErr {}

pub fn create_temp_file(extension: &str) -> Result<(TempDir, PathBuf), UtilsErr> {
    let temp_dir = tempdir().map_err(UtilsErr::TempFileCreationFailed)?;
    let temp_file_path = temp_dir.path().join(format!("temp_file.{}", extension));
    Ok((temp_dir, temp_file_path))
}

pub fn make_permanent(
    temp_dir: TempDir,
    temp_file_path: &Path,
    output_path: &Path,
) -> Result<(), UtilsErr> {
    fs::rename(temp_file_path, output_path).map_err(UtilsErr::FileRenameFailed)?;
    temp_dir.close().map_err(UtilsErr::TempDirCloseFailed)?;
    Ok(())
}

pub fn prepare_directory_with_files<P: AsRef<Path>>(
    files: &[P],
    target_directory: P,
) -> Result<(), UtilsErr> {
    DirBuilder::new()
        .recursive(true)
        .create(&target_directory)
        .map_err(UtilsErr::DirectoryCreationFailed)?;

    for file_path in files {
        if let Some(filename) = file_path.as_ref().file_name() {
            let target_path = target_directory.as_ref().join(filename);
            fs::copy(&file_path, &target_path).map_err(UtilsErr::FileCopyFailed)?;
        }
    }

    Ok(())
}
