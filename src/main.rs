use std::path::{Path, PathBuf};

use convert_case::{Case, Casing};
use parse_display::{Display, FromStr};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Opt {
    #[structopt(name = "build")]
    Build {
        /// ~/homebrew-tap
        #[structopt(long)]
        tap: PathBuf,
    },
}

fn main() {
    match Opt::from_args() {
        Opt::Build { tap } => {
            let package = {
                let content = std::fs::read_to_string("Cargo.toml").unwrap();
                let cargo = toml::from_str::<cargo_toml::Manifest>(&content).unwrap();
                cargo.package.unwrap()
            };

            let name = package.name;
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
                    let release_template = include_str!("templates/release.yml");
                    release_template.replace("{ name }", &name)
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
            }
        }
    }
}
