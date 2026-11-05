use std::path::{Path, PathBuf};
use crate::error::Result;

#[derive(thiserror::Error, Debug)]
pub enum IOError {
    #[error("HTTP cache I/O error at {path}: {source}")]
    CacheIo {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

pub async fn write_atomic(path: &Path, bytes: &[u8]) -> Result<()> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    tokio::fs::create_dir_all(parent)
        .await
        .map_err(|source| IOError::CacheIo {
            path: parent.into(),
            source,
        })?;
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    let tmp = parent.join(format!(".{name}.tmp-{}", std::process::id()));
    tokio::fs::write(&tmp, bytes)
        .await
        .map_err(|source| IOError::CacheIo {
            path: tmp.clone(),
            source,
        })?;
    if let Err(source) = tokio::fs::rename(&tmp, path).await {
        let _ = tokio::fs::remove_file(&tmp).await;
        return Err(IOError::CacheIo {
            path: path.into(),
            source,
        }.into());
    }
    Ok(())
}
