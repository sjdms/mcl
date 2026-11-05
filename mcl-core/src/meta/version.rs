use crate::{error::Result, http};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VersionManifest {
    pub latest: LatestVersions,
    pub versions: Vec<Version>,
}

#[derive(Debug, Deserialize)]
pub struct LatestVersions {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Deserialize)]
pub struct Version {
    pub id: String,

    #[serde(rename = "type")]
    pub version_type: String,

    pub url: String,
    pub time: String,

    #[serde(rename = "releaseTime")]
    pub release_time: String,
}

pub async fn get_version_manifest(
    http_client: &http::HttpClient,
    cache_dir: impl AsRef<std::path::Path>,
) -> Result<VersionManifest> {
    http_client
        .get_json_cached(
            "https://launchermeta.mojang.com/mc/game/version_manifest.json",
            cache_dir,
        )
        .await
        .map_err(|e| e.into())
}
