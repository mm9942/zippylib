use crate::utils::{create_temp_file, make_permanent, UtilsErr};
use bzip2::write::BzEncoder;
use flate2::write::GzEncoder;
use std::{
    error::Error,
    fs::{self, File},
    io::{self, Write},
    path::Path,
};
use tar::Builder;
use xz2::write::XzEncoder;

#[derive(Debug)]
pub enum TarErr {
    TempFileCreationFailed(UtilsErr),
    ArchiveCreationFailed(io::Error),
    TempFileFinalizationFailed(UtilsErr),
}

impl Error for TarErr {}

impl std::fmt::Display for TarErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TarErr::TempFileCreationFailed(e) => write!(f, "Temporary file creation failed: {}", e),
            TarErr::ArchiveCreationFailed(e) => write!(f, "Failed to create tar archive: {}", e),
            TarErr::TempFileFinalizationFailed(e) => {
                write!(f, "Failed to finalize the temporary file: {}", e)
            }
        }
    }
}

pub fn create_tar_archive<P: AsRef<Path>>(files: &[P], output_path: P) -> Result<(), TarErr> {
    let (temp_dir, temp_file_path) =
        create_temp_file("tar").map_err(TarErr::TempFileCreationFailed)?;
    let file = File::create(&temp_file_path).map_err(TarErr::ArchiveCreationFailed)?;
    let mut archive = Builder::new(file);

    for file_path in files {
        archive
            .append_path(file_path)
            .map_err(|e| TarErr::ArchiveCreationFailed(e))?;
    }

    archive
        .finish()
        .map_err(|e| TarErr::ArchiveCreationFailed(e))?;
    make_permanent(temp_dir, &temp_file_path, output_path.as_ref())
        .map_err(TarErr::TempFileFinalizationFailed)?;
    Ok(())
}

pub fn create_tar_gz_archive<P: AsRef<Path>>(files: &[P], output_path: P) -> Result<(), TarErr> {
    let (temp_dir, temp_file_path) =
        create_temp_file("tar.gz").map_err(TarErr::TempFileCreationFailed)?;
    let file = File::create(&temp_file_path).map_err(TarErr::ArchiveCreationFailed)?;
    let tar_gz_encoder = GzEncoder::new(file, flate2::Compression::best());
    let mut archive = Builder::new(tar_gz_encoder);

    for file_path in files {
        archive
            .append_path(file_path)
            .map_err(|e| TarErr::ArchiveCreationFailed(e))?;
    }

    archive
        .finish()
        .map_err(|e| TarErr::ArchiveCreationFailed(e))?;
    make_permanent(temp_dir, &temp_file_path, output_path.as_ref())
        .map_err(TarErr::TempFileFinalizationFailed)?;
    Ok(())
}

pub fn create_tar_bz2_archive<P: AsRef<Path>>(files: &[P], output_path: P) -> Result<(), TarErr> {
    let (temp_dir, temp_file_path) =
        create_temp_file("tar.bz2").map_err(TarErr::TempFileCreationFailed)?;
    let file = File::create(&temp_file_path).map_err(TarErr::ArchiveCreationFailed)?;
    let mut tar_bz2_encoder = BzEncoder::new(file, bzip2::Compression::best());
    let mut archive = Builder::new(tar_bz2_encoder);

    for file_path in files {
        archive
            .append_path(file_path)
            .map_err(|e| TarErr::ArchiveCreationFailed(e))?;
    }

    archive
        .finish()
        .map_err(|e| TarErr::ArchiveCreationFailed(e))?;
    make_permanent(temp_dir, &temp_file_path, output_path.as_ref())
        .map_err(TarErr::TempFileFinalizationFailed)?;
    Ok(())
}

pub fn create_tar_xz_archive<P: AsRef<Path>>(files: &[P], output_path: P) -> Result<(), TarErr> {
    let (temp_dir, temp_file_path) =
        create_temp_file("tar.xz").map_err(TarErr::TempFileCreationFailed)?;
    let file = File::create(&temp_file_path).map_err(TarErr::ArchiveCreationFailed)?;
    let mut tar_xz_encoder = XzEncoder::new(file, 9);
    let mut archive = Builder::new(tar_xz_encoder);

    for file_path in files {
        archive
            .append_path(file_path)
            .map_err(|e| TarErr::ArchiveCreationFailed(e))?;
    }

    archive
        .finish()
        .map_err(|e| TarErr::ArchiveCreationFailed(e))?;
    make_permanent(temp_dir, &temp_file_path, output_path.as_ref())
        .map_err(TarErr::TempFileFinalizationFailed)?;
    Ok(())
}
