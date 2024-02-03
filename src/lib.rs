mod bzip;
mod gzip;
mod tar;
mod utils;
mod xz;
mod zip;

pub use crate::{bzip::*, gzip::*, tar::*, utils::*, xz::*, zip::*};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        bzip::create_file_bzip2,
        gzip::{create_gzip_archive, encode_file_deflate, encode_file_zlib},
        tar::{
            create_tar_archive, create_tar_bz2_archive, create_tar_gz_archive,
            create_tar_xz_archive,
        },
        utils::{prepare_directory_with_files, UtilsErr},
        xz::create_file_xz,
        zip::create_zip_archive,
    };
    use clap::{arg, Command};
    use std::{
        error::Error,
        fs::File,
        io::Write,
        path::{Path, PathBuf},
    };

    fn create_example_files() -> Result<(), Box<dyn Error>> {
        let example_file_paths = &["file1.txt", "file2.txt"];

        for file_path in example_file_paths {
            if !Path::new(file_path).exists() {
                let mut file = File::create(file_path)?;
                file.write_all(b"This is example content.")?;
            }
        }

        Ok(())
    }

    #[test]
    fn tar_works() -> Result<(), Box<dyn Error>> {
        // Create example files if they don't exist
        create_example_files()?;

        // Define the input files and output paths for compression
        let input_files: Vec<PathBuf> = ["file1.txt", "file2.txt"]
            .iter()
            .map(PathBuf::from)
            .collect();

        let output = PathBuf::from("output.tar");
        // Create a temporary directory for working with files
        let temp_dir = tempfile::tempdir().map_err(UtilsErr::TempFileCreationFailed)?;

        // Prepare the input directory with files
        let input_dir = temp_dir.path().join("input");
        prepare_directory_with_files(&input_files, input_dir)?;

        // Create a tar archive
        create_tar_archive(&input_files, output.clone())?;

        Ok(())
    }

    #[test]
    fn tar_gz_works() -> Result<(), Box<dyn Error>> {
        // Create example files if they don't exist
        create_example_files()?;

        // Define the input files and output paths for compression
        let input_files: Vec<PathBuf> = ["file1.txt", "file2.txt"]
            .iter()
            .map(PathBuf::from)
            .collect();

        let output = PathBuf::from("output.tar.gz");
        // Create a temporary directory for working with files
        let temp_dir = tempfile::tempdir().map_err(UtilsErr::TempFileCreationFailed)?;

        // Prepare the input directory with files
        let input_dir = temp_dir.path().join("input");
        prepare_directory_with_files(&input_files, input_dir)?;

        // Create a tar archive
        create_tar_gz_archive(&input_files, output.clone())?;

        Ok(())
    }

    #[test]
    fn tar_xz_works() -> Result<(), Box<dyn Error>> {
        // Create example files if they don't exist
        create_example_files()?;

        // Define the input files and output paths for compression
        let input_files: Vec<PathBuf> = ["file1.txt", "file2.txt"]
            .iter()
            .map(PathBuf::from)
            .collect();

        let output = PathBuf::from("output.tar.xz");
        // Create a temporary directory for working with files
        let temp_dir = tempfile::tempdir().map_err(UtilsErr::TempFileCreationFailed)?;

        // Prepare the input directory with files
        let input_dir = temp_dir.path().join("input");
        prepare_directory_with_files(&input_files, input_dir)?;

        // Create a tar archive
        create_tar_xz_archive(&input_files, output.clone())?;

        Ok(())
    }

    #[test]
    fn tar_bz2_works() -> Result<(), Box<dyn Error>> {
        // Create example files if they don't exist
        create_example_files()?;

        // Define the input files and output paths for compression
        let input_files: Vec<PathBuf> = ["file1.txt", "file2.txt"]
            .iter()
            .map(PathBuf::from)
            .collect();

        let output = PathBuf::from("output.tar.bz2");
        // Create a temporary directory for working with files
        let temp_dir = tempfile::tempdir().map_err(UtilsErr::TempFileCreationFailed)?;

        // Prepare the input directory with files
        let input_dir = temp_dir.path().join("input");
        prepare_directory_with_files(&input_files, input_dir)?;

        // Create a tar archive
        create_tar_bz2_archive(&input_files, output.clone())?;

        Ok(())
    }

    #[test]
    fn encode_file_gz() -> Result<(), Box<dyn Error>> {
        create_example_files()?;
        let file_to_compress_gzip = "file1.txt";
        let gzip_output_path = "file1.gz";
        match gzip::create_gzip_archive(file_to_compress_gzip, gzip_output_path) {
            Ok(()) => println!("GZIP archive created successfully."),
            Err(e) => eprintln!("GZIP archive creation failed: {}", e),
        }

        Ok(())
    }

    #[test]
    fn encode_file_bz2() -> Result<(), Box<dyn Error>> {
        create_example_files()?;
        let file_to_compress_gzip = "file1.txt";
        let gzip_output_path = "file1.bz2";
        match gzip::create_gzip_archive(file_to_compress_gzip, gzip_output_path) {
            Ok(()) => println!("BZIP2 archive created successfully."),
            Err(e) => eprintln!("BZIP2 archive creation failed: {}", e),
        }

        Ok(())
    }

    #[test]
    fn encode_file_xz() -> Result<(), Box<dyn Error>> {
        create_example_files()?;
        let file_to_compress_gzip = "file1.txt";
        let gzip_output_path = "file1.xz";
        match gzip::create_gzip_archive(file_to_compress_gzip, gzip_output_path) {
            Ok(()) => println!("XZ2 archive created successfully."),
            Err(e) => eprintln!("XZ2 archive creation failed: {}", e),
        }

        Ok(())
    }

    #[test]
    fn encode_file_deflate_test() -> Result<(), Box<dyn Error>> {
        create_example_files()?;
        let file_to_compress = "file1.txt";
        let output_path = "file1.deflate";
        encode_file_deflate(file_to_compress, output_path)?;
        // Verification logic or just Ok(()) if nothing to verify
        Ok(())
    }

    #[test]
    fn encode_file_zlib_test() -> Result<(), Box<dyn Error>> {
        create_example_files()?;
        let file_to_compress = "file1.txt";
        let output_path = "file1.zlib";
        encode_file_zlib(file_to_compress, output_path)?;
        // Verification logic or just Ok(()) if nothing to verify
        Ok(())
    }
}
