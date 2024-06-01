use std::path::PathBuf;

pub mod cli;

pub static PYTHON_THINGS: [&str; 5] = [
    ".mypy_cache",
    ".ruff_cache",
    ".pytest_cache",
    "__pycache__",
    ".venv",
];

#[derive(Debug)]
#[allow(dead_code)]
pub struct SearchResult {
    pub base_path: PathBuf,
    pub is_node_modules: bool,
    pub is_rust: bool,
    pub is_python_related: bool,
    pub is_dotfile: bool,
    pub is_git: bool,
}
