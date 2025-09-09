use log::info;
use octocrab::models::repos::Asset;
use reqwest::Client;
use std::io::{Cursor, Error};
use std::path::Path;
use std::{fs, io};
use zip::ZipArchive;

pub fn unpack_zip(
    archive: &mut ZipArchive<Cursor<&[u8]>>,
    prefix_path: &Path,
) -> Result<(), Error> {
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let Some(outpath) = file.enclosed_name() else {
            continue;
        };

        if file.is_dir() {
            fs::create_dir_all(prefix_path.join(&outpath))?;
        } else {
            // TODO: Why is the example the way it is? Are there empty folders as dirs or is this a problem by going via index and not hierarchical?
            if let Some(p) = outpath.parent()
                && !p.exists()
            {
                fs::create_dir_all(prefix_path.join(p))?;
            }

            let mut outfile = fs::File::create(prefix_path.join(&outpath))?;
            io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

pub async fn get_assets_for_repo_and_tag(
    organisation: &str,
    repo: &str,
    tag: &str,
) -> anyhow::Result<Vec<Asset>> {
    let release = octocrab::instance()
        .repos(organisation, repo)
        .releases()
        .get_by_tag(tag)
        .await?;
    info!(
        "Found release {} for {}/{}/{}",
        release.name.unwrap_or("N/A".into()),
        organisation,
        repo,
        tag
    );
    Ok(release.assets)
}

pub async fn download_and_unzip_asset_to(
    client: &Client,
    asset: &Asset,
    prefix: &Path,
) -> anyhow::Result<()> {
    // TODO: Caching, but we only have the AssetId maybe?
    info!(
        "Downloading {} from {}",
        asset.name, asset.browser_download_url
    );

    // TODO: Streaming?
    let result = client
        .get(asset.browser_download_url.clone())
        .send()
        .await?
        .bytes()
        .await?;

    let mut archive = ZipArchive::new(Cursor::new(result.as_ref()))?;
    unpack_zip(&mut archive, prefix)?;

    Ok(())
}
