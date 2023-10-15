use std::collections::HashSet;

use anyhow::*;
use tl::*;
use url::*;

pub fn get_unique_links(from_url: &str, page: String) -> anyhow::Result<HashSet<String>> {
    let all_hrefs = find_all_hrefs(page)?;
    let normalized_urls = normalize_urls(from_url, all_hrefs)?;
    let from_same_domain = filter_domain(from_url, normalized_urls)?;
    Ok(unique(from_same_domain))
}

fn find_all_hrefs(page: String) -> anyhow::Result<Vec<String>> {
    let dom = tl::parse(page.as_str(), ParserOptions::default())?;
    let parser = dom.parser();
    let anchors_iterator = dom
        .query_selector("a[href]")
        .with_context(|| format!("could not query for anchors"))?;

    let mut results = Vec::default();
    for matched_tag_node in anchors_iterator {
        let href = matched_tag_node
            .get(&parser)
            .and_then(|n| n.as_tag())
            .and_then(|t| t.attributes().get("href"));

        if let Some(Some(value)) = href {
            let string_value = String::from_utf8_lossy(value.as_bytes());
            results.push(string_value.into_owned());
        }
    }

    Ok(results)
}

fn normalize_urls(from_url: &str, urls: Vec<String>) -> anyhow::Result<Vec<String>> {
    let mut results = Vec::default();

    for url in urls {
        if url.starts_with("#") {
            continue; // link to itself
        } else if url.starts_with("//") {
            let from_url =
                Url::parse(&from_url).with_context(|| format!("could not parse {from_url}"))?;
            let scheme = from_url.scheme();
            results.push(format!("{scheme}:{url}"));
        } else if url.starts_with("/") {
            let from_url =
                Url::parse(&from_url).with_context(|| format!("could not parse {from_url}"))?;

            let scheme = from_url.scheme();
            let domain = from_url
                .domain()
                .with_context(|| format!("domain missing in {from_url}"))?;
            results.push(format!("{scheme}://{domain}{url}"));
        } else {
            results.push(url)
        }
    }

    Ok(results)
}

fn filter_domain(from_url: &str, urls: Vec<String>) -> anyhow::Result<Vec<String>> {
    let from_url = Url::parse(&from_url).with_context(|| format!("could not parse {from_url}"))?;

    let domain = from_url
        .domain()
        .with_context(|| format!("domain missing in {from_url}"))?;

    let filtered = urls
        .into_iter()
        .filter(|url| url.contains(domain))
        .collect();

    Ok(filtered)
}

fn unique(urls: Vec<String>) -> HashSet<String> {
    HashSet::from_iter(urls.into_iter())
}
