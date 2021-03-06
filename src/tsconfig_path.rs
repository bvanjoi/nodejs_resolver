// Copy from https://github.com/dividab/tsconfig-paths

use super::tsconfig::TsConfig;
use crate::{parse::Request, RResult, ResolveInfo, Resolver, ResolverStats};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Default, Debug)]
pub struct TsConfigInfo {
    pub paths: Option<HashMap<String, Vec<String>>>,
    pub base_url: Option<String>,
}

#[derive(Debug, PartialEq)]
struct MappingEntry {
    pub(crate) pattern: String,
    /// The item in `paths` maybe contains '*' tag
    pub(crate) paths: Vec<PathBuf>,
}

impl Resolver {
    fn get_absolute_mapping_entries(
        absolute_base_url: &Path,
        paths: &HashMap<String, Vec<String>>,
    ) -> Vec<MappingEntry> {
        paths
            .iter()
            .map(|(key, paths)| {
                let pattern = key.to_string();
                let paths = paths
                    .iter()
                    .map(|path| absolute_base_url.join(path))
                    .collect();
                MappingEntry { pattern, paths }
            })
            .collect()
    }

    fn parse_tsconfig(location: &Path, resolver: &Resolver) -> RResult<TsConfigInfo> {
        let tsconfig = TsConfig::parse_file(location, resolver)?;
        let base_url = tsconfig
            .compiler_options
            .as_ref()
            .and_then(|options| options.base_url.clone());
        let paths = tsconfig.compiler_options.and_then(|options| options.paths);
        Ok(TsConfigInfo { base_url, paths })
    }

    fn match_star<'a>(pattern: &'a str, search: &'a str) -> Option<&'a str> {
        if search.len() < pattern.len() {
            return None;
        }
        if pattern == "*" {
            return Some(search);
        }
        pattern.find(|c| c == '*').and_then(|star_index| {
            let part1 = &pattern[..star_index];
            let part2 = &pattern[star_index + 1..];
            if &search[0..star_index] != part1 {
                return None;
            }
            if &search[search.len() - part2.len()..] != part2 {
                return None;
            }
            let len = search.len() - part2.len() - part1.len();
            Some(&search[star_index..star_index + len])
        })
    }

    fn create_match_list(
        location: &Path,
        base_url: &Option<String>,
        paths: &Option<HashMap<String, Vec<String>>>,
    ) -> Vec<MappingEntry> {
        let location_dir = location.parent().unwrap();

        let absolute_base_url = if let Some(base_url) = base_url.as_ref() {
            location_dir.join(base_url)
        } else {
            return vec![];
        };
        paths
            .as_ref()
            .map(|paths| Self::get_absolute_mapping_entries(&absolute_base_url, paths))
            .unwrap_or_default()
    }

    pub(super) fn _resolve_with_tsconfig(
        &self,
        info: ResolveInfo,
        location: &Path,
    ) -> ResolverStats {
        let tsconfig = match Self::parse_tsconfig(location, self) {
            Ok(tsconfig) => tsconfig,
            Err(error) => return ResolverStats::Error((error, info)),
        };
        let absolute_path_mappings =
            Resolver::create_match_list(location, &tsconfig.base_url, &tsconfig.paths);

        for entry in absolute_path_mappings {
            let star_match = if entry.pattern == info.request.target {
                ""
            } else {
                match Self::match_star(&entry.pattern, &info.request.target) {
                    Some(s) => s,
                    None => continue,
                }
            };

            for physical_path_pattern in &entry.paths {
                let physical_path = &physical_path_pattern
                    .display()
                    .to_string()
                    .replace('*', star_match);

                let path = PathBuf::from(physical_path);
                let result = self._resolve(ResolveInfo::from(path, Request::empty()));
                if result.is_success() {
                    return result;
                }
            }
        }
        self._resolve(info)
    }
}

#[test]
fn test_get_absolute_mapping_entries() {
    let result = Resolver::get_absolute_mapping_entries(
        &Path::new("/absolute/base/url"),
        &HashMap::from([
            (
                "*".to_string(),
                (vec!["/foo1", "./foo2"])
                    .into_iter()
                    .map(String::from)
                    .collect(),
            ),
            (
                "longest/pre/fix/*".to_string(),
                vec!["./foo2/bar".to_string()],
            ),
            ("pre/fix/*".to_string(), vec!["/foo3".to_string()]),
        ]),
    );
    assert!(result.len() == 3);
    assert!(result.contains(&MappingEntry {
        pattern: "longest/pre/fix/*".to_string(),
        paths: vec![PathBuf::from("/absolute/base/url/foo2/bar")],
    }));
    assert!(result.contains(&MappingEntry {
        pattern: "pre/fix/*".to_string(),
        paths: vec![PathBuf::from("/foo3")],
    },));
    assert!(result.contains(&MappingEntry {
        pattern: "*".to_string(),
        paths: vec![
            PathBuf::from("/foo1"),
            PathBuf::from("/absolute/base/url/foo2")
        ],
    }));

    let result = Resolver::get_absolute_mapping_entries(
        &Path::new("/absolute/base/url"),
        &HashMap::from([]),
    );
    assert!(result.len() == 0);
}
