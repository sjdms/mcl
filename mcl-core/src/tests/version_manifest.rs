use crate::meta::version::VersionManifest;

#[test]
fn test_version_manifest_deserialization() {
    let json = r#"
    {
        "latest": {
            "release": "1.19.4",
            "snapshot": "1.20.1"
        },
        "versions": [
            {
                "id": "1.19.4",
                "type": "release",
                "url": "https://launchermeta.mojang.com/v1/packages/...",
                "time": "2022-06-08T14:00:00Z",
                "releaseTime": "2022-06-08T14:00:00Z"
            }
        ]
    }
    "#;

    let version_manifest: VersionManifest = serde_json::from_str(json).unwrap();
    assert_eq!(version_manifest.latest.release, "1.19.4");
    assert_eq!(version_manifest.latest.snapshot, "1.20.1");
    assert_eq!(version_manifest.versions.len(), 1);
}