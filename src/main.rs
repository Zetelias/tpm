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
    // Load the local package list, or create a new one if it doesn't exist.
    let mut local_pkglist = match LocalPkgList::load() {
        Some(pkglist) => pkglist,
        None => {
            let pkglist = LocalPkgList::new_empty();
            pkglist.save().unwrap();
            pkglist
        }
    };

    // Load the global package list, or load from url if it doesn't exist.
    let mut global_pkglist = match GlobalPkgList::load_local() {
        Some(pkglist) => pkglist,
        None => {
            let pkglist = GlobalPkgList::load_from_url(consts::GLOBAL_PKG_LIST_URL).unwrap();
            pkglist.save_local().unwrap();
            pkglist
        }
    };
}