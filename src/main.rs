use std::path::{Path, PathBuf};

use structopt::StructOpt;

use cli::Manager;

#[derive(StructOpt, Debug)]
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
}

fn main() -> anyhow::Result<()> {
    match Opt::from_args() {
        Opt {
            homebrew_tap_path,
            file,
            bin,
        } => {
            let cargo = {
                let content = std::fs::read_to_string(&file).unwrap();
                let cargo = toml::from_str::<cargo_toml::Manifest>(&content).unwrap();
                cargo
            };

            let package = cargo.package.unwrap();

            let name = if let Some(bin_name) = bin.clone() {
                let products = cargo
                    .bin
                    .into_iter()
                    .filter(|product| product.name.as_ref().unwrap().to_string() == bin_name)
                    .collect::<Vec<_>>();
                if products.len() > 0 {
                    bin_name
                } else {
                    anyhow::bail!("Not found bin");
                }
            } else {
                package.name
            };

            let manager = Manager {
                name,
                description: package.description.unwrap_or("".to_owned()),
                homepage: package.homepage.unwrap_or("".to_owned()),
                repository: package.repository.unwrap_or("".to_string()),
                homebrew_tap_path,
                bin,
            };

            manager.write_homebrewtap_workflows_update_formula()?;
            manager.write_homebrewtap_templates_formula()?;
            manager.write_project_templates_formula()?;

            Ok(())
        }
    }
}
