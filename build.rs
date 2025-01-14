extern crate bindgen;

use flate2::read::GzDecoder;
use reqwest::blocking::get;
use shellexpand;
use std::env;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::process::Command;
use tar::Archive;
use walkdir::WalkDir;

const LIBAOM_VERSION: &str = "3.9.1";
const LIBAOM_URL: &str = "https://storage.googleapis.com/aom-releases/libaom-3.9.1.tar.gz";
const AOM_DIR: &str = "~/.hyperflow/aom";
const AOM_BUILD_DIR: &str = "~/.hyperflow/aom_build";

const HEADERS: [&str; 5] = [
    "aom/aom.h",
    "aom/aomcx.h",
    "aom/aom_decoder.h",
    "aom/aom_encoder.h",
    "aom/aomdx.h",
];

fn create_combined_header_file() {
    let aom_dir = shellexpand::tilde(AOM_DIR).to_string();
    let combined_header_path = Path::new(&aom_dir).join("combined_header.h");

    let mut combined_content = String::new();

    for header in &HEADERS {
        combined_content.push_str(&format!("#include \"{}\"\n", header));
    }

    fs::write(&combined_header_path, combined_content)
        .expect("Failed to create combined header file");
    println!(
        "Successfully created combined header file at: {:?}",
        combined_header_path
    );
}

fn download_and_extract_libaom() {
    let tmp_dir = env::temp_dir().join("libaom");
    fs::create_dir_all(&tmp_dir).expect("Failed to create temporary directory");

    let response = get(LIBAOM_URL).expect("Failed to download libaom");
    let archive = GzDecoder::new(Cursor::new(
        response.bytes().expect("Failed to read response bytes"),
    ));

    let mut archive = Archive::new(archive);
    let extracted_dir = tmp_dir.join(format!("libaom-{}", LIBAOM_VERSION));

    fs::create_dir_all(&extracted_dir).expect("Failed to create extraction directory");

    for entry in archive.entries().expect("Failed to read archive entries") {
        let mut entry = entry.expect("Failed to get entry");
        let path = entry.path().expect("Failed to get entry path");
        let out_path = extracted_dir.join(
            path.strip_prefix(format!("libaom-{}/", LIBAOM_VERSION))
                .unwrap_or(&path),
        );

        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create directories");
        }

        entry.unpack(out_path).expect("Failed to unpack entry");
    }

    let aom_dir = shellexpand::tilde(AOM_DIR).to_string();

    if Path::new(&aom_dir).exists() {
        fs::remove_dir_all(&aom_dir).expect("Failed to remove existing aom directory");
    }

    fs::create_dir_all(&aom_dir).expect("Failed to create aom directory");

    for entry in WalkDir::new(&extracted_dir) {
        let entry = entry.expect("Failed to get directory entry");
        let path = entry.path();
        let dest_path = Path::new(&aom_dir).join(path.strip_prefix(&extracted_dir).unwrap());
        if path.is_dir() {
            fs::create_dir_all(&dest_path).expect("Failed to create destination directory");
        } else if path.is_file() {
            fs::copy(path, &dest_path).expect("Failed to copy file");
        }
    }

    println!("Successfully copied files to {}", aom_dir);
    fs::remove_dir_all(tmp_dir).expect("Failed to remove temporary directory");
}

fn build_libaom() {
    let aom_src_dir = shellexpand::tilde(AOM_DIR).to_string();
    let aom_build_dir = shellexpand::tilde(AOM_BUILD_DIR).to_string();

    if !Path::new(&aom_build_dir).exists() {
        fs::create_dir_all(&aom_build_dir).expect("Failed to create aom_build directory");
        println!("Created directory: {:?}", aom_build_dir);
    } else {
        println!("Directory already exists: {:?}", aom_build_dir);
    }

    if !Path::new(&format!("{}/libaom.so", aom_build_dir)).exists() {
        println!("Building libaom...");

        let cmake_status = Command::new("cmake")
            .arg(&aom_src_dir)
            .arg(format!("-DCMAKE_INSTALL_PREFIX={}", aom_src_dir))
            .arg("-DBUILD_SHARED_LIBS=ON")
            .current_dir(&aom_build_dir)
            .status()
            .expect("Failed to run cmake");

        if !cmake_status.success() {
            panic!("CMake failed");
        }

        let make_status = Command::new("make")
            .current_dir(&aom_build_dir)
            .status()
            .expect("Failed to run make");

        if !make_status.success() {
            panic!("Make failed");
        }

        println!("libaom built successfully!");
    } else {
        println!("libaom is already built");
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let aom_include_path = shellexpand::tilde(AOM_DIR).to_string();
    let aom_build_path = shellexpand::tilde(AOM_BUILD_DIR).to_string();
    let ffi_dir = PathBuf::from("src/ffi");

    if !ffi_dir.exists() {
        println!("Creating ffi directory at: {:?}", ffi_dir);
        fs::create_dir_all(&ffi_dir).expect("Failed to create ffi directory");
    } else {
        println!("ffi directory already exists at: {:?}", ffi_dir);
    }

    let mut headers_exist = true;

    for header in &HEADERS {
        let header_path = Path::new(&aom_include_path).join(header);
        if !header_path.exists() {
            eprintln!("Header file {} not found in {:?}.", header, header_path);
            headers_exist = false;
        } else {
            println!("Found header file: {:?}", header_path);
        }
    }

    if headers_exist {
        println!("Found all headers in {}", aom_include_path);
    } else {
        eprintln!(
            "Not all header files found in {}. Downloading libaom version {}...",
            aom_include_path, LIBAOM_VERSION
        );

        download_and_extract_libaom();
    }

    build_libaom();

    create_combined_header_file();

    let combined_header_path = Path::new(&aom_include_path).join("combined_header.h");

    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", aom_include_path))
        .header(combined_header_path.to_str().unwrap())
        .generate()
        .expect("Unable to generate bindings for libaom");

    let bindings_str = bindings.to_string();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let ffi_output_dir = out_path.join("ffi");

    if !out_path.exists() {
        println!("Creating output directory at: {:?}", out_path);
        fs::create_dir_all(&out_path).expect("Failed to create output directory");
    } else {
        println!("Output directory already exists at: {:?}", out_path);
    }

    if !ffi_output_dir.exists() {
        println!("Creating ffi output directory at: {:?}", ffi_output_dir);
        fs::create_dir_all(&ffi_output_dir).expect("Failed to create ffi output directory");
    }

    let bindings_file_path = ffi_output_dir.join(format!("{}.rs", "aom"));

    fs::write(&bindings_file_path, bindings_str).expect("Couldn't write bindings!");

    let src_bindings_path = ffi_dir.join(format!("{}.rs", "aom"));
    println!("Copying bindings to source path: {:?}", src_bindings_path);

    fs::copy(&bindings_file_path, &src_bindings_path)
        .expect("Failed to copy generated bindings to src directory");

    println!("cargo:rustc-link-search=native={}", aom_build_path);
    println!("cargo:rustc-link-lib=aom");
}
