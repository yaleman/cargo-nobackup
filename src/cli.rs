#![deny(warnings)]

use std::path::PathBuf;

use clap::*;
use flexi_logger::{DeferredNow, Logger};
use log::Record;

pub const DEFAULT_BASE_PATH: &str = "./";

pub fn log_format(
    w: &mut dyn std::io::Write,
    _now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    // let level = record.level();
    write!(
        w,
        "{}",
        // now.format(TS_DASHES_BLANK_COLONS_DOT_BLANK),
        // record.level(),
        &record.args()
    )
}

#[derive(Clone, Debug, Parser)]
pub struct CliOpts {
    base_path: Option<PathBuf>,
    #[arg(short, long, default_value = "false")]
    debug: bool,
    #[arg(short, long)]
    /// If you enable this, it'll keep scanning past finding the first reason to flag a directory
    pub be_thorough: bool,
}

impl CliOpts {
    /// Returns the base path (or the [DEFAULT_BASE_PATH] if not provided).
    pub fn base_path(&self) -> PathBuf {
        let path = self
            .base_path
            .clone()
            .unwrap_or(PathBuf::from(DEFAULT_BASE_PATH));
        let path = path.as_os_str();
        let path = path.to_string_lossy();

        PathBuf::from(
            shellexpand::full(&path)
                .expect("Failed to expand path")
                .to_string(),
        )
        .canonicalize()
        .expect("Failed to expand path, bailing!")
    }

    /// Does what it says on the tin
    pub fn setup_logging(&self) -> Result<(), flexi_logger::FlexiLoggerError> {
        let level = if self.debug { "debug" } else { "info" };
        Logger::try_with_env_or_str(level)?
            .format(log_format)
            .start()?;
        Ok(())
    }
}
