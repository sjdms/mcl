//! Shared HTTP client and small, disk-backed ETag cache for metadata requests.
//!
//! Cache files are private to the application and are treated as untrusted input. A
//! stale cached response is returned only when the network request fails; callers
//! should still validate any data they consume.

use std::{
	future::Future,
	path::{Path, PathBuf},
	time::Duration,
};

use reqwest::{header, Client, Method, RequestBuilder, Response, StatusCode, Url};
use serde::de::DeserializeOwned;
use thiserror::Error;

use crate::fsutil;

const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const DEFAULT_JSON_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug, Error)]
pub enum HttpError {
	#[error("HTTP request failed: {0}")]
	Request(#[from] reqwest::Error),
	#[error("HTTP request returned {status} for {url}")]
	Status { status: StatusCode, url: Url },
	#[error("invalid URL: {0}")]
	InvalidUrl(#[from] url::ParseError),
	#[error("HTTP request timed out")]
	Timeout,
	#[error("could not decode cached or remote JSON: {0}")]
	Json(#[from] serde_json::Error),
	
}

pub type Result<T> = std::result::Result<T, HttpError>;

#[derive(Clone, Debug)]
pub struct HttpConfig {
	pub user_agent: String,
	pub proxy: Option<String>,
	pub connect_timeout: Duration,
}

impl HttpConfig {
	pub fn new(version: &str, project_url: &str) -> Self {
		Self {
			user_agent: format!("mcl/{version} ({project_url})"),
			proxy: None,
			connect_timeout: DEFAULT_CONNECT_TIMEOUT,
		}
	}
}

impl Default for HttpConfig {
	fn default() -> Self {
		Self::new(env!("CARGO_PKG_VERSION"), "https://github.com/sjdms/mcl")
	}
}

/// Reusable client wrapper. Cloning this type cheaply clones reqwest's client handle.
#[derive(Clone, Debug)]
pub struct HttpClient {
	client: Client,
}

impl HttpClient {
	pub fn new(config: HttpConfig) -> Result<Self> {
		let mut builder = Client::builder()
			.user_agent(config.user_agent)
			.connect_timeout(config.connect_timeout)
			// Deliberately no global timeout: streamed downloads can take longer.
			.gzip(true)
			.http2_adaptive_window(true);
		if let Some(proxy) = config.proxy {
			builder = builder.proxy(reqwest::Proxy::all(proxy)?);
		}
		Ok(Self { client: builder.build()? })
	}

	/// Create a client with the package version as its user-agent version.
	pub fn shared_default() -> Result<Self> {
		Self::new(HttpConfig::default())
	}

	/// Start a request without imposing a total timeout. Useful for file streams.
	pub fn request(&self, method: Method, url: &str) -> Result<RequestBuilder> {
		Ok(self.client.request(method, Url::parse(url)?))
	}

	/// Fetch JSON with a bounded request timeout.
	pub async fn get_json<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
		let parsed = Url::parse(url)?;
		let response = self.client.get(parsed.clone()).timeout(DEFAULT_JSON_TIMEOUT).send().await?;
		let response = checked(response, parsed).await?;
		Ok(response.json().await?)
	}

	/// Fetch JSON using a URL-keyed ETag cache. A 304 reuses the cached body, and a
	/// network failure falls back to a previously cached body when available.
	pub async fn get_json_cached<T: DeserializeOwned>(
		&self,
		url: &str,
		cache_dir: impl AsRef<Path>,
	) -> Result<T> {
		let parsed = Url::parse(url)?;
		let (body_path, etag_path) = cache_paths(cache_dir.as_ref(), &parsed);
		let cached_body = tokio::fs::read(&body_path).await.ok();
		let etag = tokio::fs::read_to_string(&etag_path).await.ok();

		let mut request = self.client.get(parsed.clone()).timeout(DEFAULT_JSON_TIMEOUT);
		if let Some(etag) = etag.as_deref() {
			request = request.header(header::IF_NONE_MATCH, etag.trim());
		}
		let response = match request.send().await {
			Ok(response) => response,
			Err(error) => {
				if let Some(body) = cached_body {
					return Ok(serde_json::from_slice(&body)?);
				}
				return Err(error.into());
			}
		};

		if response.status() == StatusCode::NOT_MODIFIED {
			return cached_body
				.as_deref()
				.map(serde_json::from_slice)
				.transpose()?
				.ok_or(HttpError::Status { status: StatusCode::NOT_MODIFIED, url: parsed });
		}
		let response = checked(response, parsed).await?;
		let new_etag = response.headers().get(header::ETAG).and_then(|v| v.to_str().ok()).map(str::to_owned);
		let body = response.bytes().await?;
		let value = serde_json::from_slice(&body)?;

		// Cache updates are best-effort: a usable network response should not fail
		// merely because the user's cache directory is read-only.
		if tokio::fs::create_dir_all(cache_dir.as_ref()).await.is_ok() {
			let _ = fsutil::write_atomic(&body_path, &body).await;
			if let Some(etag) = new_etag {
				let _ = fsutil::write_atomic(&etag_path, etag.as_bytes()).await;
			}
		}
		Ok(value)
	}

	/// Expose the underlying client for specialized APIs while retaining one shared pool.
	pub fn as_reqwest(&self) -> &Client {
		&self.client
	}
}

async fn checked(response: Response, url: Url) -> Result<Response> {
	let status = response.status();
	if status.is_success() {
		Ok(response)
	} else {
		Err(HttpError::Status { status, url })
	}
}

fn cache_paths(cache_dir: &Path, url: &Url) -> (PathBuf, PathBuf) {
	// Hash the full URL rather than using URL components as filesystem paths.
	let key = format!("{:x}", md5::compute(url.as_str()));
	(cache_dir.join(format!("{key}.json")), cache_dir.join(format!("{key}.etag")))
}

/// Adapt a request to an operation-specific timeout without changing the shared client.
pub async fn with_timeout<T>(duration: Duration, operation: impl Future<Output = Result<T>>) -> Result<T> {
	tokio::time::timeout(duration, operation)
		.await
		.unwrap_or_else(|_| Err(HttpError::Timeout))
}
