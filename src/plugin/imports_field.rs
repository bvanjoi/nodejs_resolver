use std::sync::Arc;

use crate::{description::PkgFileInfo, plugin::ExportsFieldPlugin, Resolver, MODULE};

use super::{AliasFieldPlugin, Plugin};
use crate::{
    map::{Field, ImportsField},
    PathKind, ResolveInfo, ResolverStats,
};

pub struct ImportsFieldPlugin<'a> {
    pkg_info: &'a Option<Arc<PkgFileInfo>>,
}

impl<'a> ImportsFieldPlugin<'a> {
    pub fn new(pkg_info: &'a Option<Arc<PkgFileInfo>>) -> Self {
        Self { pkg_info }
    }

    fn check_target(info: ResolveInfo, target: &str) -> ResolverStats {
        if info.get_path().is_file() && ImportsField::check_target(&info.request.target) {
            ResolverStats::Resolving(info)
        } else {
            ResolverStats::Error((format!("Package path {target} is not exported"), info))
        }
    }
}

impl<'a> Plugin for ImportsFieldPlugin<'a> {
    fn apply(&self, resolver: &Resolver, info: ResolveInfo) -> ResolverStats {
        if let Some(pkg_info) = self.pkg_info {
            if !info.request.target.starts_with('#') {
                return ResolverStats::Resolving(info);
            }

            let target = &info.request.target;
            let list = if let Some(root) = &pkg_info.imports_field_tree {
                match ImportsField::field_process(root, target, &resolver.options.condition_names) {
                    Ok(list) => list,
                    Err(err) => return ResolverStats::Error((err, info)),
                }
            } else {
                return ResolverStats::Resolving(info);
            };

            if let Some(item) = list.first() {
                let request = resolver.parse(item);
                let is_normal_kind = matches!(request.kind, PathKind::Normal);
                let is_internal_kind = matches!(request.kind, PathKind::Internal);
                let info = ResolveInfo::from(
                    if is_normal_kind {
                        pkg_info.abs_dir_path.join(MODULE)
                    } else {
                        pkg_info.abs_dir_path.to_path_buf()
                    },
                    request,
                );

                if is_normal_kind {
                    let path = info.get_path();
                    // TODO: should optimized
                    let pkg_info = match resolver.load_pkg_file(&path) {
                        Ok(info) => info,
                        Err(err) => return ResolverStats::Error((err, info)),
                    };
                    if let Some(ref pkg_info) = pkg_info {
                        if !pkg_info.abs_dir_path.display().to_string().contains(MODULE) {
                            return ResolverStats::Resolving(info);
                        }
                    }

                    ExportsFieldPlugin::new(&pkg_info)
                        .apply(resolver, info)
                        .and_then(|info| ImportsFieldPlugin::new(&pkg_info).apply(resolver, info))
                        .and_then(|info| AliasFieldPlugin::new(&pkg_info).apply(resolver, info))
                        .and_then(|info| ImportsFieldPlugin::check_target(info, target))
                } else if is_internal_kind {
                    self.apply(resolver, info)
                } else {
                    ImportsFieldPlugin::check_target(info, target)
                }
            } else {
                ResolverStats::Error((format!("Package path {target} is not exported"), info))
            }
        } else {
            ResolverStats::Resolving(info)
        }
    }
}
