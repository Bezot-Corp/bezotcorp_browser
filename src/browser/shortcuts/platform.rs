use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) enum Platform {
    Linux,
    Windows,
    MacOs,
    Unknown,
}

impl Platform {
    pub(crate) fn current() -> Self {
        #[cfg(target_os = "linux")]
        {
            Self::Linux
        }

        #[cfg(target_os = "windows")]
        {
            Self::Windows
        }

        #[cfg(target_os = "macos")]
        {
            Self::MacOs
        }

        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        {
            Self::Unknown
        }
    }
}
