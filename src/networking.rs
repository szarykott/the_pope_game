use anyhow::*;
use reqwest::*;

pub fn get_page_content(url: impl IntoUrl) -> anyhow::Result<String> {
    let url = url.into_url()?;

    let response = reqwest::blocking::get(url.clone())
        .with_context(|| format!("failed to load contents of {url}"))?
        .error_for_status()?;

    response
        .text()
        .with_context(|| format!("failed to read page contents of {url}"))
}
