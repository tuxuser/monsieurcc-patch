use anyhow::{anyhow, Context, Result};
use glob::glob;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MatchType {
    Literal,
    Regex,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Patch {
    name: String,
    info: String,
    filepaths: Vec<String>,
    match_type: MatchType,
    match_pattern: String,
    replace: String,
}

pub enum MatchPattern {
    Literal(String),
    Regex(Regex),
}

pub fn deserialize_patches(patchdata: &str) -> Result<Vec<Patch>> {
    serde_yaml::from_str(patchdata).context(format!("Deserialization failed, input: {}", patchdata))
}

fn get_match_pattern(match_type: MatchType, pattern: String) -> Result<MatchPattern> {
    let ret = match match_type {
        MatchType::Literal => MatchPattern::Literal(pattern),
        MatchType::Regex => {
            let regex = Regex::new(&pattern).context("Failed to compile regex")?;
            MatchPattern::Regex(regex)
        }
    };

    Ok(ret)
}

fn patch_file(mut data: String, match_pattern: &MatchPattern, replace: &str) -> (bool, String) {
    let mut found = false;
    match match_pattern {
        MatchPattern::Regex(re) => {
            if re.is_match(&data) {
                found = true;
                data = re.replace_all(&data, replace).to_string()
            }
            (found, data)
        }
        MatchPattern::Literal(str_pattern) => {
            if data.contains(str_pattern) {
                found = true;
                data = data.replace(str_pattern, replace);
            }
            (found, data)
        }
    }
}

pub fn apply_patches(base_path: &Path, patches: Vec<Patch>) -> Result<()> {
    for patch in patches {
        println!("-> Patch: {}", patch.name);
        let mut find_count = 0; // Per-patch

        let match_pattern = get_match_pattern(patch.match_type, patch.match_pattern)
            .context("Failed to init match pattern")?;

        for fp in patch.filepaths {
            let path = {
                let mut p = base_path.to_path_buf();
                p.push(fp);
                p.to_str()
                    .context(format!("Failed to convert {:?} to string", p))?
                    .to_string()
            };
            let paths_iter = glob(&path).context("Invalid glob pattern")?;

            for entry in paths_iter {
                let path = entry.context("Failed to get globbed entry")?;
                let filebuf =
                    std::fs::read_to_string(&path).context("Failed to read file to patch")?;

                let (found, patched_data) = patch_file(filebuf, &match_pattern, &patch.replace);
                if found {
                    find_count += 1;
                    println!("* Match on file {:?}", &path);
                }

                std::fs::write(&path, patched_data).context("Failed to write patched file")?;
            }
            if find_count == 0 {
                return Err(anyhow!("No file matched!"));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const YAML_DATA: &str = r#"
---
- name: Disable SSL (Part 3)
  info: Removes call to `.setSSLSocketFactory` on `java.net.HttpURLConnection` (there is no such method for non-SSL URL connection ;))
  filepaths:
    - "smali/*.smali"
  match_type: regex
  match_pattern: '\.line\s([0-9]+)\n\s+sget-object\sp2,\s.+Ljavax\/net\/ssl\/SSLContext;\n\s+invoke-virtual[\S\s]+setSSLSocketFactory\(Ljavax\/net\/ssl\/SSLSocketFactory;\)V'
  replace: .line $1\n    nop

- name: Patch in custom domain or host ip
  info: Affected files, smali/mcapi/McApi.smali, smali/mcapi/McUsageApi.smali, additionally, on version >= 1.1.22-248 also smali/mcapi/APIServiceFactory.smali
  filepaths:
    - "smali/*.smali"
  match_type: literal
  match_pattern: "mc20.monsieur-cuisine.com"
  replace: "<###DOMAIN###>"
"#;

    #[test]
    fn deserialize() {
        let res = deserialize_patches(YAML_DATA);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 2);
    }
}
