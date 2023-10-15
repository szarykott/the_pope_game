mod html;
mod networking;

use anyhow::*;
use std::io::*;

fn main() -> anyhow::Result<()> {
    let url = "https://pl.wikipedia.org/wiki/Jan_Pawe%C5%82_II";

    let page_contents = networking::get_page_content(url)?;
    let links = html::get_unique_links(url, page_contents)?;

    let mut file = std::fs::File::create("./.dump/test.html")?;
    for href in links {
        file.write(href.as_bytes())?;
        file.write(b"\n")?;
    }

    Ok(())
}
