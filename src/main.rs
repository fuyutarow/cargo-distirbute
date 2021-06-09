use std::path::{Path, PathBuf};

use colored::*;
use structopt::StructOpt;

use cli::Manager;

#[derive(StructOpt, Debug)]
#[structopt(bin_name = "cargo")]
struct Opt {
    /// ~/homebrew-tap repository path
    #[structopt(short = "t", long = "tap")]
    homebrew_tap_path: PathBuf,

    /// Cargo.toml file path
    #[structopt(short, long, default_value = "Cargo.toml")]
    file: PathBuf,

    /// Cargo.toml bin
    #[structopt(short, long)]
    bin: Option<String>,

    /// Set the channel used to build the rust project
    #[structopt(long, possible_values=&["stable", "beta", "nightly"], default_value="stable")]
    channel: String,

    /// Set the features used to build the rust project
    #[structopt(long)]
    features: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let Opt {
        homebrew_tap_path,
        file,
        bin,
        channel,
        features,
    } = Opt::from_args();

    let cargo = {
        let content = std::fs::read_to_string(&file).unwrap();
        let cargo = toml::from_str::<cargo_toml::Manifest>(&content).unwrap();
        cargo
    };

    let package = cargo.package.unwrap();

    let name = if let Some(bin_name) = bin.clone() {
        if cargo
            .bin
            .into_iter()
            .filter(|product| product.name.as_ref().unwrap().to_string() == bin_name)
            .count()
            > 0
        {
            bin_name
        } else {
            anyhow::bail!("Not found bin");
        }
    } else {
        package.name
    };

    let mut fields_are_filled = true;
    if package.description.is_none() {
        fields_are_filled = false;
        eprintln!(
            "❌ The {} field is required in cargo.toml.",
            "description".red()
        );
    }

    if package.homepage.is_none() {
        fields_are_filled = false;
        eprintln!(
            "❌ The {} field is required in cargo.toml. (e.g. https://github.com/fuyutarow/cargo-distribute)",
            "homepage".red()
        );
    }

    if package.repository.is_none() {
        fields_are_filled = false;
        eprintln!(
            "❌ The {} field is required in cargo.toml. (e.g. https://github.com/fuyutarow/cargo-distribute.git)",
            "repository".red()
        );
    }

    if package.license.is_none() {
        fields_are_filled = false;
        eprintln!(
            "❌ The {} field is required in cargo.toml. (e.g. MIT)",
            "license".red()
        );
    }

    if fields_are_filled {
        let manager = Manager {
            bin,
            channel,
            features,
            name,
            description: package.description.unwrap_or_default(),
            homepage: package.homepage.unwrap_or_default(),
            repository: package.repository.unwrap_or_default(),
            license: package.license.unwrap_or_default(),
            homebrew_tap_path,
        };

        manager.write_homebrewtap_workflows_update_formula()?;
        manager.write_homebrewtap_templates_formula()?;
        manager.write_scoop_bucket()?;
        manager.write_project_templates_formula()?;
    }

    Ok(())
}
