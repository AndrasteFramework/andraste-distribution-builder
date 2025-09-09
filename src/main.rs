use crate::parser::{BundleSettings, Cli, RepoSpec};
use crate::readme::build_readme;
use crate::util::{download_and_unzip_asset_to, get_assets_for_repo_and_tag};
use anyhow::Context;
use clap::Parser;
use dircpy::copy_dir;
use log::info;
use reqwest::Client;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use zip::CompressionMethod;
use zip::write::SimpleFileOptions;

mod parser;
mod readme;
mod util;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbosity.into())
        .init();

    match &args.operation {
        parser::Operation::CreateBundle(bundle) => create_bundle(bundle).await?,
        parser::Operation::Clear => clear()?,
    }
    Ok(())
}

fn clear() -> anyhow::Result<()> {
    fs::remove_dir_all("out").or_else(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            Ok(())
        } else {
            Err(e)
        }
    })?;
    info!("Cleared output directory");
    Ok(())
}

async fn create_bundle(bundle: &BundleSettings) -> anyhow::Result<()> {
    let now = std::time::Instant::now();
    let base_path = PathBuf::from("out");
    let dist_path = PathBuf::from("dist");
    fs::create_dir_all(&base_path)?;
    let generic_framework_path = base_path.join("GenericFramework");
    fs::create_dir_all(&generic_framework_path)?;

    let client = Client::new();

    tokio::try_join!(
        download_ui_launcher(&bundle.version, &client, &base_path),
        download_launcher(&bundle.version, &client, &generic_framework_path),
        download_payload_generic(bundle, &client, &generic_framework_path),
        build_readme(bundle, &base_path)
    )?;

    fs::create_dir_all(&dist_path)?;
    let zip_path = dist_path.join(format!("AndrasteBundle-{}.zip", bundle.version));
    zip_folder(&base_path, &zip_path).await?;

    info!("Compiled bundle in {:?}", now.elapsed());

    Ok(())
}

async fn zip_folder(source_folder: &Path, destination: &Path) -> anyhow::Result<()> {
    let mut writer = zip::ZipWriter::new(fs::File::create(destination)?);
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .compression_level(Some(9));
    let walkdir = WalkDir::new(source_folder);

    let mut dirs = Vec::new();
    let mut files = Vec::new();

    for entry in walkdir.into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_dir() {
            dirs.push(entry);
        } else {
            files.push(entry);
        }
    }

    for dir in dirs {
        let name = dir.path().strip_prefix(source_folder)?;
        let dir_name = name
            .to_str()
            .to_owned()
            .with_context(|| format!("{name:?} is not valid UTF-8"))?
            .replace("\\", "/");

        if dir_name.is_empty() {
            continue; // root folder
        }

        writer.add_directory(&dir_name, options)?;
    }

    for file in files {
        let name = file.path().strip_prefix(source_folder)?;
        let file_name = name
            .to_str()
            .to_owned()
            .with_context(|| format!("{name:?} is not valid UTF-8"))?
            .replace("\\", "/");

        writer.start_file(file_name, options)?;
        let mut f = fs::File::open(file.path())?;
        std::io::copy(&mut f, &mut writer)?;
    }
    writer.finish()?;
    Ok(())
}

async fn download_launcher(version: &str, client: &Client, base_path: &Path) -> anyhow::Result<()> {
    let assets =
        get_assets_for_repo_and_tag("AndrasteFramework", "Andraste.Launcher", version).await?;

    if assets.len() != 2 {
        anyhow::bail!(
            "Expected exactly two assets in the release, found {}",
            assets.len()
        );
    }

    let asset_x64 = assets
        .iter()
        .find(|asset| asset.name.contains("x64"))
        .ok_or_else(|| anyhow::anyhow!("Could not find x64 asset"))?;
    let path_x64 = base_path.join("x64");
    let fut_x64 = download_and_unzip_asset_to(client, asset_x64, &path_x64);

    let asset_x86 = assets
        .iter()
        .find(|asset| asset.name.contains("x86"))
        .ok_or_else(|| anyhow::anyhow!("Could not find x86 asset"))?;
    let path_x86 = base_path.join("x86");
    let fut_x86 = download_and_unzip_asset_to(client, asset_x86, &path_x86);

    tokio::try_join!(fut_x64, fut_x86)?;
    Ok(())
}

async fn download_ui_launcher(
    version: &str,
    client: &Client,
    base_path: &Path,
) -> anyhow::Result<()> {
    let assets = get_assets_for_repo_and_tag("AndrasteFramework", "UILauncher", version).await?;

    if assets.len() != 1 {
        anyhow::bail!(
            "Expected exactly one asset in the release, found {}",
            assets.len()
        );
    }

    download_and_unzip_asset_to(client, &assets[0], base_path).await?;
    Ok(())
}

async fn download_payload_generic(
    settings: &BundleSettings,
    client: &Client,
    base_path: &Path,
) -> anyhow::Result<()> {
    let default_spec = RepoSpec::new("AndrasteFramework", "Payload.Generic");
    let repo_spec = settings.framework_repo.as_ref().unwrap_or(&default_spec);
    let assets = get_assets_for_repo_and_tag(
        &repo_spec.organisation,
        &repo_spec.repository,
        &settings.version,
    )
    .await?;

    if assets.len() != 1 {
        anyhow::bail!(
            "Expected exactly one asset in the release, found {}",
            assets.len()
        );
    }

    let path = base_path.join("x86");
    let path_x64 = base_path.join("x64");

    download_and_unzip_asset_to(client, &assets[0], &path).await?;
    tokio::task::spawn_blocking(move || copy_dir(path, path_x64)).await??;
    Ok(())
}
