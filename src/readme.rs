use crate::parser::BundleSettings;
use anyhow::Context;
use chrono::{Datelike, Utc};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use text_template::Template;

pub async fn build_readme(settings: &BundleSettings, base_path: &Path) -> anyhow::Result<()> {
    let template_string = if let Some(path) = settings.readme_template_path.as_ref() {
        fs::read_to_string(path).with_context(|| {
            format!(
                "Failed to read the readme template from {:?}",
                path.as_os_str()
            )
        })?
    } else {
        include_str!("README.tpl").to_string()
    };

    let template = Template::from(template_string.as_str());

    let mut parameters = HashMap::new();
    parameters.insert("RELEASE", settings.version.as_str());

    let now = Utc::now();
    let day = now.day0().to_string();
    let month = now.month0().to_string();
    let year = now.year().to_string();

    parameters.insert("YEAR", &year);
    parameters.insert("MONTH", &month);
    parameters.insert("DAY", &day);

    let content = template.fill_in(&parameters).to_string();

    fs::write(base_path.join("README.txt"), content)?;
    Ok(())
}
