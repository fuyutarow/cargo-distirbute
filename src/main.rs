use std::path::{Path, PathBuf};

use convert_case::{Case, Casing};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Opt {
    #[structopt(name = "build")]
    Build {
        /// ~/homebrew-tap repository path
        #[structopt(short, long)]
        tap: PathBuf,

        /// Cargo.toml file path
        #[structopt(short, long, default_value = "Cargo.toml")]
        file: PathBuf,

        /// Cargo.toml bin
        #[structopt(short, long)]
        bin: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    match Opt::from_args() {
        Opt::Build { tap, file, bin } => {
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
                    // anyhow::bail!(&format!(
                    //     "Not found bin `{}` in {}",
                    //     bin_name,
                    //     file.into_os_string().to_str().unwrap()
                    // ));
                }
            } else {
                package.name
            };
            let name_pascal_case = name.to_case(Case::Pascal);
            let descripton = package.description;
            let homepage = package.homepage;
            let repository = package.repository;

            {
                let mut context = tera::Context::new();
                context.insert("name", &name);
                context.insert("name_pascal_case", &name_pascal_case);
                context.insert("description", &descripton);
                context.insert("homepage", &homepage);
                context.insert("repository", &repository);

                match tera::Tera::one_off(include_str!("templates/formula.rb"), &context, false) {
                    Ok(formula) => {
                        let tap_templates_dir = {
                            let path = tap.join("templates");
                            std::fs::create_dir_all(&path).unwrap();
                            path
                        };

                        let formula_fpath = tap_templates_dir.join(format!("{}.rb", name));

                        {
                            std::fs::write(&formula_fpath, formula);
                            println!(
                                "{} was written",
                                formula_fpath.into_os_string().into_string().unwrap()
                            );
                        }
                    }
                    Err(err) => eprintln!("{}", err),
                }
            }

            {
                let release = {
                    let bin_option = if let Some(bin_name) = bin {
                        format!("--bin {}", &bin_name)
                    } else {
                        "".to_string()
                    };
                    let release_template = include_str!("templates/release.yml");
                    release_template
                        .replace("{% name %}", &name)
                        .replace("{% bin_option %}", &bin_option)
                };

                let release_fpath = {
                    let github_workflows_dir = {
                        let path = Path::new(".github/workflows");
                        std::fs::create_dir_all(&path);
                        path
                    };
                    github_workflows_dir.join("release.yml")
                };

                {
                    std::fs::write(&release_fpath, release);
                    println!(
                        "{} was written",
                        release_fpath.into_os_string().into_string().unwrap()
                    );
                }

                Ok(())
            }
        }
    }
}
