# ZippyLib

ZippyLib is a versatile Rust library designed for integrating file compression and decompression functionalities into Rust projects, supporting a wide array of formats: ZIP, TAR, TAR.GZ, TAR.XZ, TAR.BZ2, BZ2, XZ, GZ, Deflate, and Zlib. This guide provides straightforward API usage instructions to facilitate the inclusion of file compression and decompression capabilities.

## Installation

Add ZippyLib to your Rust project by including it in your `Cargo.toml`:

```toml
[dependencies]
zippylib = "^0.1"
```

or use:

```bash
cargo add zippylib
```

## Usage

### ZIP

```rust
use zippylib::create_zip_archive;
use std::path::PathBuf;

let files = vec![PathBuf::from("file1.txt"), PathBuf::from("file2.txt")];
let output_path = PathBuf::from("archive.zip");

create_zip_archive(&files, output_path).expect("ZIP archive creation failed");
```

### TAR

```rust
use zippylib::create_tar_archive;
use std::path::PathBuf;

let files = vec![PathBuf::from("file1.txt"), PathBuf::from("file2.txt")];
let output_path = PathBuf::from("archive.tar");

create_tar_archive(&files, &output_path).expect("TAR archive creation failed");
```

### TAR.GZ

```rust
use zippylib::create_tar_gz_archive;
use std::path::PathBuf;

let files = vec![PathBuf::from("file1.txt"), PathBuf::from("file2.txt")];
let output_path = PathBuf::from("archive.tar.gz");

create_tar_gz_archive(&files, &output_path).expect("TAR.GZ archive creation failed");
```

### TAR.XZ

```rust
use zippylib::create_tar_xz_archive;
use std::path::PathBuf;

let files = vec![PathBuf::from("file1.txt"), PathBuf::from("file2.txt")];
let output_path = PathBuf::from("archive.tar.xz");

create_tar_xz_archive(&files, &output_path).expect("TAR.XZ archive creation failed");
```

### TAR.BZ2

```rust
use zippylib::create_tar_bz2_archive;
use std::path::PathBuf;

let files = vec![PathBuf::from("file1.txt"), PathBuf::from("file2.txt")];
let output_path = PathBuf::from("archive.tar.bz2");

create_tar_bz2_archive(&files, &output_path).expect("TAR.BZ2 archive creation failed");
```

### BZ2

```rust
use zippylib::create_file_bzip2;
use std::path::PathBuf;

let input_path = PathBuf::from("file1.txt");
let output_path = PathBuf::from("file1.bz2");

create_file_bzip2(&input_path, &output_path).expect("BZ2 file creation failed");
```

### XZ

```rust
use zippylib::create_file_xz;
use std::path::PathBuf;

let input_path = PathBuf::from("file1.txt");
let output_path = PathBuf::from("file1.xz");

create_file_xz(&input_path, &output_path).expect("XZ file creation failed");
```

### GZ

```rust
use zippylib::create_gzip_archive;
use std::path::PathBuf;

let file_path = PathBuf::from("file1.txt");
let output_path = PathBuf::from("file1.gz");

create_gzip_archive(&file_path, &output_path).expect("GZ archive creation failed");
```

### Deflate

```rust
use zippylib::encode_file_deflate;
use std::path::PathBuf;

let file_to_compress = PathBuf::from("file1.txt");
let output_path = PathBuf::from("file1.deflate");

encode_file_deflate(&file_to_compress, &output_path).expect("Deflate encoding failed");
```

### Zlib

```rust
use zippylib::encode_file_zlib;
use std::path::PathBuf;

let file_to_compress = PathBuf::from("file1.txt");
let output_path = PathBuf::from("file1.zlib");

encode_file_zlib(&file_to_compress, &output_path).expect("Zlib encoding failed");
```

## Error Handling

All operations are designed to return a `Result<(), ErrorType>`, enabling robust error handling. Specific error types are defined to facilitate detailed error reporting and handling.

## Dependencies

ZippyLib makes use of several third-party crates to support its functionality:

- **bzip2 (0.4.4)**
- **flate2 (1.0.28)**
- **tar (0.4.40)**
- **tempfile (3.9.0)**
- **xz2 (0.1.7)**
- **zip (0.6.6)**

These dependencies are critical for providing the comprehensive compression and archiving capabilities of ZippyLib.

## License

ZippyLib is licensed under the MIT LICENSE. The full license text is available in the `LICENSE` file in the repository.