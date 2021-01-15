//! The provider module owns the `PlatformDataProvider` trait as well as the methods needed to
//! convert submodule errors to a single error type for further consumption in `main()`.

use crate::settings::SettingsJson;

#[cfg(any(bottlerocket_platform = "aws", bottlerocket_platform = "aws-dev"))]
pub(crate) mod aws;

#[cfg(bottlerocket_platform = "aws-dev")]
pub(crate) mod local_file;

#[cfg(bottlerocket_platform = "vmware")]
pub(crate) mod cdrom;

/// Support for new platforms can be added by implementing this trait.
pub(crate) trait PlatformDataProvider {
    /// You should return a list of SettingsJson, representing the settings changes you want to
    /// send to the API.
    ///
    /// This is a list so that handling multiple data sources within a platform can feel more
    /// natural; you can also send all changes in one entry if you like.
    fn platform_data(&self) -> Result<Vec<SettingsJson>>;
}

mod error {
    use snafu::Snafu;

    #[derive(Debug, Snafu)]
    #[snafu(visibility = "pub(super)")]
    pub(crate) enum Error {
        #[cfg(any(bottlerocket_platform = "aws", bottlerocket_platform = "aws-dev"))]
        #[snafu(display("Provider error: {}", source))]
        AwsProvider { source: crate::provider::aws::Error },

        #[cfg(bottlerocket_platform = "aws-dev")]
        #[snafu(display("Provider error: {}", source))]
        LocalFileProvider {
            source: crate::provider::local_file::Error,
        },

        #[cfg(bottlerocket_platform = "vmware")]
        #[snafu(display("Provider error: {}", source))]
        CdromProvider {
            source: crate::provider::cdrom::Error,
        },
    }

    #[cfg(any(bottlerocket_platform = "aws", bottlerocket_platform = "aws-dev"))]
    use crate::provider::aws::Error as AWSError;
    #[cfg(any(bottlerocket_platform = "aws", bottlerocket_platform = "aws-dev"))]
    impl From<AWSError> for Error {
        fn from(e: crate::provider::aws::Error) -> Error {
            Error::AwsProvider { source: e }
        }
    }

    #[cfg(bottlerocket_platform = "aws-dev")]
    use crate::provider::local_file::Error as LocalFileError;
    #[cfg(bottlerocket_platform = "aws-dev")]
    impl From<LocalFileError> for Error {
        fn from(e: crate::provider::local_file::Error) -> Error {
            Error::LocalFileProvider { source: e }
        }
    }

    #[cfg(bottlerocket_platform = "vmware")]
    use crate::provider::cdrom::Error as CdromError;
    #[cfg(bottlerocket_platform = "vmware")]
    impl From<CdromError> for Error {
        fn from(e: crate::provider::cdrom::Error) -> Error {
            Error::CdromProvider { source: e }
        }
    }
}

pub(crate) use error::Error;
type Result<T> = std::result::Result<T, error::Error>;
