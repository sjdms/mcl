use std::path::PathBuf;

use crate::error::Result;

pub struct Paths {
    pub config: PathBuf,
    pub data: PathBuf,
    pub cache: PathBuf,
    pub logs: PathBuf,
}
impl Paths {
    pub fn resolve() -> Result<Self> { /* MCL_HOME override, else ProjectDirs */
        // If MCL_HOME is set, use it as the base directory. Otherwise, use the default project directories.
        let base_dir = std::env::var("MCL_HOME").map(PathBuf::from).unwrap_or_else(|_| {
            let proj_dirs = directories::ProjectDirs::from("dev", "sajed", "mcl").expect("Failed to determine project directories");
            proj_dirs.data_dir().to_path_buf()
        });
        Ok(Self {
            config: base_dir.join("config"),
            data: base_dir.join("data"),
            cache: base_dir.join("cache"),
            logs: base_dir.join("logs"),
        })
    }
    pub fn assets(&self) -> PathBuf {
        self.data.join("assets")
    }
    pub fn libraries(&self) -> PathBuf {
        self.data.join("libraries")
    }
    pub fn version_json(&self, id: &str) -> PathBuf {
        self.data
            .join("versions")
            .join(id)
            .join(format!("{id}.json"))
    }
    pub fn version_jar(&self, id: &str) -> PathBuf {
        self.data
            .join("versions")
            .join(id)
            .join(format!("{id}.jar"))
    }
    // …
}
