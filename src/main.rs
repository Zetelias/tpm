mod download;
mod platform;
mod build_script;
mod package;
mod global_pkglist;
mod local_pkglist;
mod consts;
mod genpkglist;

use std::collections::HashMap;
use serde_json::to_string_pretty;
use download::download_and_extract;
use crate::build_script::BuildScript;
use crate::global_pkglist::GlobalPkgList;
use crate::local_pkglist::LocalPkgList;
use crate::package::Package;
use crate::platform::PlatformSpecificInfo;

#[tokio::main]
async fn main() {
    genpkglist::gen_empty_pkglist();
}