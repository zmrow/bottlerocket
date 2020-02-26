#![deny(rust_2018_idioms)]

use migration_helpers::{migrate, Result};
use migration_helpers::common_migrations::RemoveSettingMigration;
use std::process;

/// This is a test of our ability to remove a setting in Bottlerocket.
fn run() -> Result<()> {
    migrate(RemoveSettingMigration("settings.motd"))
}

// Returning a Result from main makes it print a Debug representation of the error, but with Snafu
// we have nice Display representations of the error, so we wrap "main" (run) and print any error.
// https://github.com/shepmaster/snafu/issues/110
fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
