use cargo_nobackup::{SearchResult, PYTHON_THINGS};
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use std::path::{Path, PathBuf};

use cargo_nobackup::cli::CliOpts;
use clap::Parser;
use log::{debug, error, info};
use rayon::prelude::*;

fn scan_path(base_path: &Path, be_thorough: bool) -> Vec<SearchResult> {
    let dir_contents: Vec<PathBuf> = match base_path.read_dir() {
        Ok(contents) => contents
            .filter_map(|entry| entry.map(|entry| entry.path()).ok())
            .collect(),
        Err(err) => {
            error!(
                "Failed to read contents of '{}' error={:?}",
                base_path.display(),
                err
            );
            Vec::new()
        }
    };

    // if it has a Cargo.toml file then we should be able to infer it's a rust project
    let is_rust = dir_contents.par_iter().any(|contents| {
        if let Some(filename) = contents.file_name() {
            if filename == "Cargo.toml" {
                return true;
            }
        }
        false
    });

    // if it is node_modules or .node_modules then we should be able to infer it's a node project
    let mut is_node_modules = false;
    let mut is_python_related = false;
    let mut is_dotfile = false;
    let mut is_git = false;
    if let Some(name) = base_path.file_name() {
        if name.to_string_lossy().starts_with('.') {
            is_dotfile = true
        }

        if name == "node_modules" || name == ".node_modules" {
            is_node_modules = true;
        } else if name == ".git" {
            is_git = true;
        } else if PYTHON_THINGS.contains(&name.to_str().unwrap()) {
            is_python_related = true;
        }
    }

    // if we aren't being thorough, we can skip scanning the contents of node_modules and rust projects
    let mut res: Vec<SearchResult> =
        if be_thorough || !(is_node_modules || is_rust || is_python_related || is_dotfile) {
            // we don't care about files themselves.
            dir_contents
                .par_iter()
                .flat_map(|path| {
                    if path.is_dir() {
                        scan_path(path, be_thorough)
                    } else {
                        Vec::new()
                    }
                })
                .collect()
        } else {
            Vec::new()
        };

    res.push(SearchResult {
        base_path: base_path.to_path_buf(),
        is_node_modules,
        is_rust,
        is_python_related,
        is_dotfile,
        is_git,
    });
    res
}

fn main() {
    let opts = CliOpts::parse();
    if let Err(err) = opts.setup_logging() {
        eprintln!("Logging setup failed, can't continue! error: {err:?}");
        std::process::exit(1);
    }

    info!("Starting at {:?}", &opts.base_path());

    if !opts.base_path().exists() {
        error!("Path does not exist");
        std::process::exit(1);
    }

    let results: Vec<_> = [opts.base_path()]
        .par_iter()
        .flat_map(|base_path| {
            debug!("Processing {:?}", base_path);
            scan_path(base_path, opts.be_thorough)
        })
        .collect();
    info!("Found {} results", results.len());
}
