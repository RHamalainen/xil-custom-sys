//! This script creates foreign function interface to C board support package.

mod validators;

use bindgen::Bindings;
use glob::glob;
use std::{
    collections::HashSet,
    env,
    path::{Path, PathBuf},
    process::exit,
};
use validators::{validate_path_to_directory, validate_path_to_file};

const EXIT_FAILURE: i32 = 1;
const LIBRARY_NAME: &str = "xil_sf";
const PATH_WRAPPER: &str = "./wrapper.h";
const PATH_XIL_HEADERS: &str = "./include/";
const PATH_XIL_BSP: &str = "./libsrc/";

/*
/// Guess the Xilinx SDK install path like ".../Xilinx/SDK/version"
fn guess_xil_sdk_path() -> path::PathBuf {
    // If XILINX_SDK environment variable exists, use that
    let xil_sdk_env = env::var(XILINX_SDK_ENV_VAR_NAME);
    if let Ok(xil_sdk_env) = xil_sdk_env {
        return path::Path::new(&xil_sdk_env).to_path_buf();
    }

    // Like ".../Xilinx"
    let xil_dir =
        // If XILINX environment variable exists, use that
        if let Ok(xil_env) = env::var(XILINX_ENV_VAR_NAME) {
            xil_env
        }
        // Otherwise try to guess a path based on platform
        else if  cfg!(windows) {
            DEFAULT_XILINX_WIN_PATH.to_owned()
        } else if cfg!(not(windows)) {
            DEFAULT_XILINX_LIN_PATH.to_owned()
        } else {
            eprintln!("cannot detect Xilinx SDK location for this OS, please make sure Xilinx SDK is installed and set the XILINX_SDK environment variable to the directory path where Xilinx SDK is installed, like so: XILINX_SDK=.../Xilinx/SDK/version");
            process::exit(1)
        };
    let xil_dir = path::Path::new(&xil_dir);

    let no_tools_at_all = !xil_dir.exists();
    // Add to comprise ".../Xilinx/SDK"
    let sdk_parent_dir = xil_dir.join(SDK_DIR_NAME).to_owned();

    if !sdk_parent_dir.exists() {
        eprintln!("cannot detect Xilinx SDK at {:?}, please make sure Xilinx SDK is installed and set the XILINX_SDK environment variable to the directory path where Xilinx SDK is installed, like so: XILINX_SDK=.../Xilinx/SDK/version", sdk_parent_dir);
        if no_tools_at_all {
            eprintln!("cannot detect any Xilinx tools at {:?}", xil_dir);
        }
        process::exit(1)
    }

    // Then try to guess a version
    let sdk_dir = sdk_parent_dir.join(DEFAULT_XILINX_SDK_VERSION);
    if sdk_dir.exists() {
        sdk_dir
    } else {
        guess_xil_sdk_ver_path(&sdk_parent_dir)
    }
}

fn guess_xil_sdk_ver_path(xil_sdk_parent_dir: &path::Path) -> path::PathBuf {
    let mut entries = fs::read_dir(xil_sdk_parent_dir)
        .expect(&format!("cannot read contents of {:?}", xil_sdk_parent_dir))
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name())
        // Filter out anything that doesn't start with a number
        .filter(|name| {
            name.to_str().map_or(false, |name| {
                name.chars().nth(0).map_or(false, |c| c.is_digit(10))
            })
        })
        .collect::<Vec<_>>();

    // Sorts by filename, smallest number first
    entries.sort();

    // Take the last element, which is the latest version
    match entries.last() {
        Some(name) => xil_sdk_parent_dir.join(name),
        None => {
            eprintln!(
                "Xilinx SDK directory {:?} contains no installed version",
                xil_sdk_parent_dir
            );
            process::exit(1)
        }
    }
}

/// Returns the Xilinx SDK path like ".../Xilinx/SDK/<ver>"
fn locate_xil_sdk_path() -> path::PathBuf {
    let xil_dir = guess_xil_sdk_path();
    let xil_dir = path::Path::new(&xil_dir);

    if !xil_dir.exists() {
        let export_cmd = "export XILINX_SDK=/path/to/Xilinx/SDK/version";
        eprintln!(
            "Xilinx SDK does not exist at path {:?}. Please make sure Xilinx SDK is installed, and set the correct path using `{}`",
            xil_dir, export_cmd
        );
        process::exit(1);
    }

    if !xil_dir.is_dir() {
        eprintln!("{:?} is not a directory", xil_dir);
        process::exit(1)
    }

    xil_dir.to_path_buf()
}
*/

/*
/// Get paths to compile
fn get_src_paths(path_libsrc: &Path) -> Vec<PathBuf> {
    // How on earth you make a globally accessible Path in rust? Is it even
    // possible? I'll make a function that returns a constant pathbuf then

    path_libsrc
        .read_dir()
        .expect("Unable to read libsrc directory")
        .filter_map(|entry| {
            let entry = entry.expect("Unable to read file from directory");
            let path = entry.path();

            // Ignore files at root-level
            if path.is_file() {
                return None;
            }

            // All paths include this intermediary src/
            let path = path.join("src/");

            Some(path)
        })
        .collect_vec()
    //.collect::<Vec<path::PathBuf>>();
}

fn src_files(path: &Path) -> Vec<PathBuf> {
    let ignore_files = vec![];

    let c_ext = Some(ffi::OsStr::new("c"));
    let asm_ext = Some(ffi::OsStr::new("S"));

    /*if path.is_file() {
        // Single files can be compiled too, though idk why someone wants that
        let ext = path.extension();
        match ext {
            e if e == c_ext => {
                return vec![path.clone()];
            }
            _ => panic!("Invalid file extension on source file."),
        }
    } else if path.is_dir() {*/
    path.read_dir()
        .expect(&format!(
            "Unable to read directory: {}",
            path.to_str().unwrap()
        ))
        .filter_map(|entry| {
            let entry = entry.expect("Unable to read a file from directory");

            let path = entry.path();

            // Ignore directories
            if path.is_dir() {
                return None;
            }

            // Ignore Files
            if ignore_files.contains(&path.file_name()) {
                return None;
            }

            // We only care about .c and .S
            let ext = path.extension();
            if ext == c_ext || ext == asm_ext {
                Some(path.clone())
            } else {
                None
            }
        })
        .collect_vec()
    //.collect::<Vec<PathBuf>>()
    /*} else {
        panic!("Uh oh")
    }*/
}
*/

fn generate_bindings(
    path_libc: &Path,
    path_stdint: &Path,
    path_xil_headers: &Path,
    path_wrapper: &Path,
) -> Bindings {
    let include_libc = format!(
        "-I{}",
        path_libc
            .to_str()
            .expect("Path to libc must be valid UTF-8.")
    );
    let include_stdint = format!(
        "-I{}",
        path_stdint
            .to_str()
            .expect("Path to stdint must be valid UTF-8.")
    );
    let include_xil = format!(
        "-I{}",
        path_xil_headers
            .to_str()
            .expect("Path to BSP headers must be valid UTF-8.")
    );
    let header = path_wrapper
        .to_str()
        .expect("Path to wrapper must be valid UTF-8.");

    bindgen::Builder::default()
        // Set-up cross-compilation.
        .clang_arg("-target")
        .clang_arg("armv7a-none-eabi")
        // Include Xilinx cross-compiler libc headers.
        .clang_arg(&include_libc)
        // Include Xilinx cross-compiler stdint headers.
        .clang_arg(&include_stdint)
        // Include Xilinx headers.
        .clang_arg(&include_xil)
        // The input header we would like to generate bindings for.
        .header(header)
        // Use core instead of std to retain no_std compatibility.
        .use_core()
        .ctypes_prefix("cty")
        // Do not generate tests, because I can't be bothered to set up #[test] in the build
        // environment of the cross-compiler
        .layout_tests(false)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Failed to generate bindings.")
}

fn compile(
    path_archiver: &Path,
    path_compiler: &Path,
    path_xil_headers: &Path,
    xil_bsp_header_search_paths: Vec<PathBuf>,
    xil_bsp_source_paths: Vec<PathBuf>,
) {
    //let mut c_files = Vec::new();
    let mut builder = cc::Build::new();

    /*for path in xil_bsp_header_search_paths {
        builder.include(path);
    }*/

    /*for path in xil_bsp_source_paths {
        builder.file(path);
    }*/

    /*for path in get_src_paths(path_bsp_sources).iter() {
        let c = src_files(path);
        c_files.extend(c);
        builder.include(path);
    }*/

    //println!("cargo:warning=Included {} C-files.", c_files.len());

    // Compile BSP's C-files.
    builder
        .compiler(path_compiler)
        .archiver(path_archiver)
        // Add files which are compiled.
        .files(xil_bsp_source_paths)
        // Add BSP headers to header search path.
        .include(path_xil_headers)
        .includes(xil_bsp_header_search_paths)
        //.files(paths_xil_bsp_sources)
        // Disable warning flags.
        .warnings(false)
        // Disable position independent code.
        .pic(false)
        // Set name of the target processor.
        .flag("-mcpu=cortex-a9")
        // Set name of the floating-point hardware.
        .flag("-mfpu=vfpv3")
        // Execute floating point operations using library calls.
        .flag("-mfloat-abi=soft")
        // Disable generation of standard runtime.
        .flag("-nostartfiles")
        .opt_level_str("2")
        // Compile or assemble the source files, but do not link.
        // Generates object file for each source file.
        .flag("-c")
        .compile(LIBRARY_NAME);
}

fn error_missing_environment_variable(name: &str) -> String {
    format!("Missing environment variable: \"{}\".", name)
}

fn get_path(environment_variable: &str) -> PathBuf {
    let path = match env::var(environment_variable) {
        Ok(path) => path,
        Err(err) => {
            let message = error_missing_environment_variable(environment_variable);
            println!("cargo:warning={} Error: {}.", message, err);
            exit(EXIT_FAILURE);
        }
    };
    Path::new(&path).to_path_buf()
}

fn main() {
    println!("cargo:rerun-if-changed=lib.rs");
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed={}", PATH_XIL_HEADERS);
    println!("cargo:rerun-if-changed={}", PATH_XIL_BSP);
    println!("cargo:rerun-if-env-changed=OUT_DIR");
    println!("cargo:rerun-if-env-changed=PATH_GNU_BINARIES");
    println!("cargo:rerun-if-env-changed=PATH_LIBC");
    println!("cargo:rerun-if-env-changed=PATH_STDINT");

    let path_output = get_path("OUT_DIR");
    let path_binaries = get_path("PATH_GNU_BINARIES");
    let path_libc = get_path("PATH_LIBC");
    let path_stdint = get_path("PATH_STDINT");

    let path_wrapper = Path::new(PATH_WRAPPER);
    let path_xil_headers = Path::new(PATH_XIL_HEADERS);
    let path_xil_bsp = Path::new(PATH_XIL_BSP);

    validate_path_to_directory(&path_output, "Path to output");
    validate_path_to_directory(&path_binaries, "Path to GNU binaries");
    validate_path_to_directory(&path_libc, "Path to libc");
    validate_path_to_directory(&path_stdint, "Path to stdint");

    validate_path_to_file(path_wrapper, "Path to wrapper");
    validate_path_to_directory(&path_xil_headers, "Path to XIL headers");
    validate_path_to_directory(&path_xil_bsp, "Path to XIL BSP");

    let path_compiler = path_binaries.join("arm-none-eabi-gcc.exe");
    let path_compiler = path_compiler.as_path();
    if path_to_file_is_valid(path_compiler, "Path to compiler") {
        println!("cargo:warning=Path to compiler is valid. Continuing build.");
    } else {
        println!("cargo:warning=Path to compiler is not valid. Stopping build.");
        exit(EXIT_FAILURE);
    }

    let path_archiver = path_binaries.join("arm-none-eabi-ar.exe");
    let path_archiver = path_archiver.as_path();
    if path_to_file_is_valid(path_archiver, "Path to archiver") {
        println!("cargo:warning=Path to archiver is valid. Continuing build.");
    } else {
        println!("cargo:warning=Path to archiver is not valid. Stopping build.");
        exit(EXIT_FAILURE);
    }

    let pattern = format!("{}**/*.h", PATH_XIL_BSP);
    let paths_xil_bsp_headers_glob = match glob(&pattern) {
        Ok(paths) => paths,
        Err(err) => {
            println!(
                "cargo:warning=Failed to find XIL BSP headers using pattern: {}. Error: {}.",
                pattern, err
            );
            exit(EXIT_FAILURE);
        }
    };
    let mut paths_xil_bsp_headers: Vec<PathBuf> = Vec::new();
    for path in paths_xil_bsp_headers_glob {
        let path = match path {
            Ok(path) => path,
            Err(err) => {
                println!(
                    "cargo:warning=Failed to add XIL BSP header path. Error: {}.",
                    err
                );
                exit(EXIT_FAILURE);
            }
        };
        paths_xil_bsp_headers.push(path);
    }
    let mut xil_bsp_header_search_paths = HashSet::new();
    for path in paths_xil_bsp_headers {
        let parent = path
            .parent()
            .expect("Path to a header must have a parent.")
            .to_path_buf();
        //println!("cargo:warning={}", parent.display());
        if xil_bsp_header_search_paths.insert(parent.clone()) {
            println!("cargo:warning=Added {}.", parent.display());
        } else {
            //println!("cargo:warning=Not added.");
        }
    }
    let xil_bsp_header_search_paths = Vec::from_iter(xil_bsp_header_search_paths);

    let pattern = format!("{}**/*.[cSs]", PATH_XIL_BSP);
    let paths_xil_bsp_sources = match glob(&pattern) {
        Ok(paths) => paths,
        Err(err) => {
            println!(
                "cargo:warning=Failed to find XIL BSP source files using pattern: {}. Error: {}.",
                pattern, err
            );
            exit(EXIT_FAILURE);
        }
    };
    let mut xil_bsp_source_paths = Vec::new();
    for path in paths_xil_bsp_sources {
        let path = match path {
            Ok(path) => path,
            Err(err) => {
                println!(
                    "cargo:warning=Failed to add XIL BSP source path. Error: {}.",
                    err
                );
                exit(EXIT_FAILURE);
            }
        };
        xil_bsp_source_paths.push(path.clone());
    }

    let bindings = generate_bindings(path_libc, path_stdint, path_xil_headers, path_wrapper);
    println!("cargo:warning=Bindings generated!");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(path_output.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    compile(
        path_archiver,
        path_compiler,
        path_xil_headers,
        xil_bsp_header_search_paths,
        xil_bsp_source_paths,
    );
    println!("cargo:warning=Compiled libxil.a successfully!");
}
