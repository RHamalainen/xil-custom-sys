use crate::EXIT_FAILURE;
use std::{path::PathBuf, process::exit};

pub fn validate_path_to_directory(path: &PathBuf, description: &str) {
    if path.exists() && path.is_dir() {
        return;
    }
    println!("cargo:warning=LOL");
    exit(EXIT_FAILURE);

    /*match path.try_exists() {
        Ok(true) => {
            println!("cargo:warning={} exists. Continuing build", description);
        }
        Ok(false) => {
            println!(
                "cargo:warning={} does not exist. Stopping build.",
                description
            );
            exit(EXIT_FAILURE);
        }
        Err(err) => {
            println!(
                "cargo:warning={} does not exist. Error: {err}. Stopping build.",
                description
            );
            exit(EXIT_FAILURE);
        }
    }
    if path.is_dir() {
        println!(
            "cargo:warning={} points to a valid directory. Continuing build.",
            description
        );
    } else {
        println!(
            "cargo:warning={} does no point to a valid directory. Stopping build.",
            description
        );
        exit(EXIT_FAILURE);
    }*/
}

pub fn validate_path_to_file(path: &PathBuf, description: &str) {
    if path.exists() && path.is_file() {
        return;
    }
    println!("cargo:warning=ASD");
    exit(EXIT_FAILURE);
    /*
    match path.try_exists() {
        Ok(true) => {
            println!("cargo:warning={} exists. Continuing build.", description);
        }
        Ok(false) => {
            println!(
                "cargo:warning={} does not exist. Stopping build.",
                description
            );
            exit(EXIT_FAILURE);
        }
        Err(err) => {
            println!(
                "cargo:warning={} does not exist. Error: {err}. Stopping build.",
                description
            );
            exit(EXIT_FAILURE);
        }
    }
    if path.is_file() {
        println!(
            "cargo:warning={} points to a valid file. Continuing build.",
            description
        );
    } else {
        println!(
            "cargo:warning={} does no point to a valid file. Stopping build.",
            description
        );
        exit(EXIT_FAILURE);
    }*/
}