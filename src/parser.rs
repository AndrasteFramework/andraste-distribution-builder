use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RepoSpec {
    pub organisation: String,
    pub repository: String,
}

impl std::str::FromStr for RepoSpec {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 2 {
            return Err("Repository must be in the format 'organisation/repository'".into());
        }
        Ok(RepoSpec {
            organisation: parts[0].to_string(),
            repository: parts[1].to_string(),
        })
    }
}

impl RepoSpec {
    pub fn new(organisation: &str, repository: &str) -> Self {
        Self {
            organisation: organisation.to_string(),
            repository: repository.to_string(),
        }
    }
}

#[derive(Debug, Clone, Args)]
pub struct BundleSettings {
    #[arg(
        long = "version",
        help = "The Version (i.e. tag/release) to use when fetching content",
        required = true
    )]
    pub version: String,

    #[arg(
        long = "framework-repo",
        help = "The repository to use for the framework (defaults to AndrasteFramework/Payload.Generic)",
        required = false
    )]
    pub framework_repo: Option<RepoSpec>,

    #[arg(
        long = "readme-template",
        help = "Path to a README.txt template file (defaults to internal template)",
        required = false
    )]
    pub readme_template_path: Option<PathBuf>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Operation {
    CreateBundle(BundleSettings),
    Clear,
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub operation: Operation,

    #[command(flatten)]
    pub verbosity: clap_verbosity_flag::Verbosity,
}
