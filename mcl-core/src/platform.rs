#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Os { Windows, MacOs, Linux }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Arch { X86, X86_64, Aarch64 }

#[derive(Clone, Copy, Debug)]
pub struct Platform { pub os: Os, pub arch: Arch }

impl Platform {
    pub fn current() -> Self {
        let os = match std::env::consts::OS {
            "windows" => Os::Windows,
            "macos"   => Os::MacOs,
            _         => Os::Linux,
        };
        let arch = match std::env::consts::ARCH {
            "x86" => Arch::X86,
            "aarch64" => Arch::Aarch64,
            _ => Arch::X86_64,
        };
        Self { os, arch }
    }
    /// Name used in Mojang rule objects: "windows" | "osx" | "linux"
    pub fn mojang_os(&self) -> &'static str {
        match self.os { Os::Windows => "windows", Os::MacOs => "osx", Os::Linux => "linux" }
    }
    pub fn classpath_separator(&self) -> &'static str {
        if matches!(self.os, Os::Windows) { ";" } else { ":" }
    }
}