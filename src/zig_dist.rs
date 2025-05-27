use proto_pdk::{HostArch, HostOS};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct ZigDist {
    pub master: Master,
    #[serde(flatten)]
    pub versions: HashMap<String, VersionInfo>,
}

#[derive(Deserialize)]
pub struct Master {
    pub version: String,
    #[serde(rename = "x86_64-macos")]
    pub x86_64_macos: Option<PlatformInfo>,
    #[serde(rename = "aarch64-macos")]
    pub aarch64_macos: Option<PlatformInfo>,
    #[serde(rename = "x86_64-linux")]
    pub x86_64_linux: Option<PlatformInfo>,
    #[serde(rename = "aarch64-linux")]
    pub aarch64_linux: Option<PlatformInfo>,
    #[serde(rename = "x86-linux")]
    pub x86_linux: Option<PlatformInfo>,
    #[serde(rename = "x86_64-windows")]
    pub x86_64_windows: Option<PlatformInfo>,
    #[serde(rename = "aarch64-windows")]
    pub aarch64_windows: Option<PlatformInfo>,
    #[serde(rename = "x86-windows")]
    pub x86_windows: Option<PlatformInfo>,
}

#[derive(Deserialize)]
pub struct VersionInfo {
    #[serde(rename = "x86_64-macos")]
    pub x86_64_macos: Option<PlatformInfo>,
    #[serde(rename = "aarch64-macos")]
    pub aarch64_macos: Option<PlatformInfo>,
    #[serde(rename = "x86_64-linux")]
    pub x86_64_linux: Option<PlatformInfo>,
    #[serde(rename = "aarch64-linux")]
    pub aarch64_linux: Option<PlatformInfo>,
    #[serde(rename = "x86-linux")]
    pub x86_linux: Option<PlatformInfo>,
    #[serde(rename = "x86_64-windows")]
    pub x86_64_windows: Option<PlatformInfo>,
    #[serde(rename = "aarch64-windows")]
    pub aarch64_windows: Option<PlatformInfo>,
    #[serde(rename = "x86-windows")]
    pub x86_windows: Option<PlatformInfo>,
}

#[derive(Deserialize)]
pub struct PlatformInfo {
    pub tarball: String,
    pub shasum: String,
    pub size: String,
}

impl Master {
    pub fn get_platform_info(&self, os: HostOS, arch: HostArch) -> Option<&PlatformInfo> {
        match (os, arch) {
            (HostOS::MacOS, HostArch::X64) => self.x86_64_macos.as_ref(),
            (HostOS::MacOS, HostArch::Arm64) => self.aarch64_macos.as_ref(),
            (HostOS::Linux, HostArch::X64) => self.x86_64_linux.as_ref(),
            (HostOS::Linux, HostArch::Arm64) => self.aarch64_linux.as_ref(),
            (HostOS::Linux, HostArch::X86) => self.x86_linux.as_ref(),
            (HostOS::Windows, HostArch::X64) => self.x86_64_windows.as_ref(),
            (HostOS::Windows, HostArch::Arm64) => self.aarch64_windows.as_ref(),
            (HostOS::Windows, HostArch::X86) => self.x86_windows.as_ref(),
            _ => None,
        }
    }
}

impl VersionInfo {
    pub fn get_platform_info(&self, os: HostOS, arch: HostArch) -> Option<&PlatformInfo> {
        match (os, arch) {
            (HostOS::MacOS, HostArch::X64) => self.x86_64_macos.as_ref(),
            (HostOS::MacOS, HostArch::Arm64) => self.aarch64_macos.as_ref(),
            (HostOS::Linux, HostArch::X64) => self.x86_64_linux.as_ref(),
            (HostOS::Linux, HostArch::Arm64) => self.aarch64_linux.as_ref(),
            (HostOS::Linux, HostArch::X86) => self.x86_linux.as_ref(),
            (HostOS::Windows, HostArch::X64) => self.x86_64_windows.as_ref(),
            (HostOS::Windows, HostArch::Arm64) => self.aarch64_windows.as_ref(),
            (HostOS::Windows, HostArch::X86) => self.x86_windows.as_ref(),
            _ => None,
        }
    }
}
