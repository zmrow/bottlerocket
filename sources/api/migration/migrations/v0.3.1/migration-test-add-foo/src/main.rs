#![deny(rust_2018_idioms)]

use migration_helpers::{migrate, Result};
use migration_helpers::common_migrations::AddSettingMigration;
use std::process;

/// This is a test of our ability to add a setting in Bottlerocket.
fn run() -> Result<()> {
    migrate(AddSettingMigration("settings.foo"))
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
