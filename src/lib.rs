use std::path::{Path, PathBuf};

use convert_case::{Case, Casing};

#[derive(Debug, Clone)]
pub struct Manager {
    pub name: String,
    pub description: String,
    pub homepage: String,
    pub repository: String,
    pub homebrew_tap_path: PathBuf,
    pub bin: Option<String>,
}

impl Manager {
    pub fn write_homebrewtap_workflows_update_formula(&self) -> anyhow::Result<()> {
        let tap_github_workflows_update_formula_fpath = {
            let path = self
                .homebrew_tap_path
                .join(".github/workflows/update-formula-cargodist.yml");
            std::fs::create_dir_all(&path.parent().unwrap())?;
            path
        };
        let tap_github_workflows_update_formula_contnet =
            include_str!("templates/update-formula-cargodist.yml");

        std::fs::write(
            &tap_github_workflows_update_formula_fpath,
            tap_github_workflows_update_formula_contnet,
        );

        println!(
            "{} was written",
            tap_github_workflows_update_formula_fpath
                .into_os_string()
                .into_string()
                .unwrap()
        );
        Ok(())
    }

    pub fn write_homebrewtap_templates_formula(&self) -> anyhow::Result<()> {
        let context = {
            let mut context = tera::Context::new();
            context.insert("name", &self.name);
            context.insert("name_pascal_case", &self.name.to_case(Case::Pascal));
            context.insert("description", &self.description);
            context.insert("homepage", &self.homepage);
            context.insert("repository", &self.repository);
            context.insert("repository_url", &self.repository.trim_end_matches(".git"));
            context
        };

        match tera::Tera::one_off(include_str!("templates/formula.rb"), &context, false) {
            Ok(formula_content) => {
                let tap_templates_formula_fpath = {
                    let path = self
                        .homebrew_tap_path
                        .join("templates")
                        .join(format!("{}.rb", &self.name));
                    std::fs::create_dir_all(&path.parent().unwrap())?;
                    path
                };
                std::fs::write(&tap_templates_formula_fpath, formula_content);

                println!(
                    "{} was written",
                    tap_templates_formula_fpath
                        .into_os_string()
                        .into_string()
                        .unwrap()
                );
            }
            Err(err) => eprintln!("{}", err),
        }
        Ok(())
    }

    pub fn write_project_templates_formula(&self) -> anyhow::Result<()> {
        let github_workflows_release_content = {
            let bin_option = if let Some(bin_name) = &self.bin {
                format!("--bin {}", &bin_name)
            } else {
                "".to_string()
            };
            let release_template = include_str!("templates/release.yml");
            release_template
                .replace("{% name %}", &self.name)
                .replace("{% bin_option %}", &bin_option)
        };

        let github_workflows_release_fpath = {
            let s = format!(".github/workflows/release-{}-cargodist.yml", &self.name);
            let path = std::env::current_dir()?.join(s);
            std::fs::create_dir_all(&path.parent().unwrap())?;
            path
        };

        std::fs::write(
            &github_workflows_release_fpath,
            github_workflows_release_content,
        );

        println!(
            "{} was written",
            github_workflows_release_fpath
                .to_owned()
                .into_os_string()
                .into_string()
                .unwrap()
        );
        Ok(())
    }
}
