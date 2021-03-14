//! The local_file module implements the `PlatformDataProvider` trait for gathering userdata from
//! local file

use super::{PlatformDataProvider, SettingsJson};
use crate::compression::expand_file_maybe;
use snafu::ResultExt;
use std::fs;

pub(crate) struct LocalFileDataProvider;

impl LocalFileDataProvider {
    pub(crate) const USER_DATA_FILE: &'static str = "/etc/early-boot-config/user-data";
}

impl PlatformDataProvider for LocalFileDataProvider {
    fn platform_data(&self) -> std::result::Result<Vec<SettingsJson>, Box<dyn std::error::Error>> {
        let mut output = Vec::new();
        info!("'{}' exists, using it", Self::USER_DATA_FILE);

        // Read the file, decompressing it if compressed.
        let user_data_str =
            expand_file_maybe(Self::USER_DATA_FILE).context(error::InputFileRead {
                path: Self::USER_DATA_FILE,
            })?;

        if user_data_str.is_empty() {
            return Ok(output);
        }

        let json = SettingsJson::from_toml_str(&user_data_str, "user data")?;
        output.push(json);

        Ok(output)
    }
}

mod error {
    use snafu::Snafu;
    use std::io;
    use std::path::PathBuf;

    #[derive(Debug, Snafu)]
    #[snafu(visibility = "pub(super)")]
    pub(crate) enum Error {
        #[snafu(display("Unable to read input file '{}': {}", path.display(), source))]
        InputFileRead { path: PathBuf, source: io::Error },

        #[snafu(context(false))]
        SettingsToJSON { source: crate::settings::Error },
    }
}
