use std::{fs::{self, File}, io::{Error, ErrorKind, Write}, path::{Path, PathBuf}, process::Command, str::FromStr};
use apt_sources_lists::*;

#[derive(Debug, Clone, Default)]
pub struct LegacyAptSource {
    pub enabled: bool,
    pub is_source: bool,
    pub components: String,
    pub filepath: PathBuf,
    pub name: String,
    pub options: Option<String>,
    pub suite: String,
    pub url: String,
}

impl LegacyAptSource {
    pub fn get_legacy_sources() -> std::io::Result<Vec<Self>> {
        let mut sources_vec = Vec::new();
        let lists = SourcesLists::scan().map_err(|err|Error::new(ErrorKind::Other, err))?;
        for file in lists.iter() {
            for entry in &file.lines {
                if let SourceLine::Entry(ref entry) = *entry {
                    let source = LegacyAptSource {
                        enabled: true,
                        is_source: entry.source,
                        components: entry.components.iter().map(|x| x.to_string() + " ").collect::<String>().trim().to_owned(),
                        filepath: file.clone().path,
                        name: entry.clone().filename(),
                        options: entry.clone().options,
                        suite: entry.clone().suite,
                        url: entry.clone().url
                    };
                    sources_vec.push(source);
                }
                if let SourceLine::Comment(ref entry) = *entry {
                    let comments = entry.lines();
                    for comment in comments {
                        if comment.starts_with("#deb") || comment.starts_with("#deb-src") {
                            match SourceLine::from_str(comment.trim_start_matches("#")) {
                                Ok(t) => {
                                    if let SourceLine::Entry(ref entry) = t {
                                        let source = LegacyAptSource {
                                            enabled: false,
                                            is_source: entry.source,
                                            components: entry.components.iter().map(|x| x.to_string() + " ").collect::<String>().trim().to_owned(),
                                            filepath: file.clone().path,
                                            name: entry.clone().filename(),
                                            options: entry.clone().options,
                                            suite: entry.clone().suite,
                                            url: entry.clone().url
                                        };
                                        sources_vec.push(source);
                                    }
                                }
                                Err(_) => {}
                            };
                        }
                    }
                }
            }
        }
        Ok(sources_vec)
    }

    pub fn save_to_apt(target_source: Self, legacy_sources_list: Vec<Self>) -> std::io::Result<()> {
        let mut sources_of_same_list = Vec::new();
        let mut pharsed_output = String::new();
        for source in legacy_sources_list {
            if source.filepath == target_source.filepath {
                sources_of_same_list.push(source)
            }
        }
        for source in sources_of_same_list {
            let string_prefix = match (source.enabled, source.is_source) {
                (true, true) => "deb-src",
                (true, false) => "deb",
                (false, true) => "#deb-src",
                (false, false) => "#deb"
            };
            match source.options {
                Some(t) => {
                    pharsed_output.push_str(&format!("{} [{}] {} {} {}\n", string_prefix, t, source.url, source.suite, source.components))
                }
                None => {
                    pharsed_output.push_str(&format!("{} {} {} {}\n", string_prefix, source.url, source.suite, source.components))
                }
            }
        }
        if target_source.filepath.exists() {
            fs::remove_file(&target_source.filepath)?
        }
        let mut file = File::create(target_source.filepath)?;
        file.write_all(pharsed_output.as_bytes())?;
        Ok(())
    }
    pub fn save_to_file_via_pkexec(target_source: Self, legacy_sources_list: Vec<Self>, filepath: &str) -> std::io::Result<()> {
        let mut sources_of_same_list = Vec::new();
        let mut pharsed_output = String::new();
        for source in legacy_sources_list {
            if source.filepath == target_source.filepath {
                sources_of_same_list.push(source)
            }
        }
        for source in sources_of_same_list {
            let string_prefix = match (source.enabled, source.is_source) {
                (true, true) => "deb-src",
                (true, false) => "deb",
                (false, true) => "#deb-src",
                (false, false) => "#deb"
            };
            match source.options {
                Some(t) => {
                    pharsed_output.push_str(&format!("{} [{}] {} {} {}\n", string_prefix, t, source.url, source.suite, source.components))
                }
                None => {
                    pharsed_output.push_str(&format!("{} {} {} {}\n", string_prefix, source.url, source.suite, source.components))
                }
            }
        }
        Command::new("pkexec")
            .arg("bash")
            .arg("-c")
            .arg(format!("echo -e {} > {}", pharsed_output.replace("\n", "\\\\n"), filepath))
            .output()?;
        Ok(())
    }
    pub fn save_to_file(target_source: Self, legacy_sources_list: Vec<Self>, filepath: &str) -> std::io::Result<()> {
        let mut sources_of_same_list = Vec::new();
        let mut pharsed_output = String::new();
        for source in legacy_sources_list {
            if source.filepath == target_source.filepath {
                sources_of_same_list.push(source)
            }
        }
        for source in sources_of_same_list {
            let string_prefix = match (source.enabled, source.is_source) {
                (true, true) => "deb-src",
                (true, false) => "deb",
                (false, true) => "#deb-src",
                (false, false) => "#deb"
            };
            match source.options {
                Some(t) => {
                    pharsed_output.push_str(&format!("{} [{}] {} {} {}\n", string_prefix, t, source.url, source.suite, source.components))
                }
                None => {
                    pharsed_output.push_str(&format!("{} {} {} {}\n", string_prefix, source.url, source.suite, source.components))
                }
            }
        }
        let mut file = File::create(filepath)?;
        file.write_all(pharsed_output.as_bytes())?;
        Ok(())
    }
}
