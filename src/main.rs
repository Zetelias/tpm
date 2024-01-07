mod download;
mod platform;
mod build_script;
mod package;
mod global_pkglist;
mod local_pkglist;
mod consts;
mod genpkglist;

use std::collections::HashMap;
use futures_util::TryFutureExt;
use serde_json::to_string_pretty;
use download::download_and_extract;
use crate::build_script::BuildScript;
use crate::consts::GLOBAL_PKG_LIST_URL;
use crate::global_pkglist::GlobalPkgList;
use crate::local_pkglist::LocalPkgList;
use crate::package::Package;
use crate::platform::PlatformSpecificInfo;

fn load_pkgslists() -> (GlobalPkgList, LocalPkgList) {
    // Load the global package list. If there is no local copy, get it from the url
    let mut global_pkglist = match GlobalPkgList::load_local() {
        Some(pkglist) => {
            println!("Found local copy of the global package list.");
            pkglist
        },
        None => {
            println!("No local copy of the global package list found. Downloading...");
            let pkglist= GlobalPkgList::load_from_url(GLOBAL_PKG_LIST_URL).unwrap();
            pkglist.save_local().unwrap();
            println!("Downloaded the global package list.");
            pkglist
        }
    };

    // Load the local package list. If there is no local copy, create an empty one
    let mut local_pkglist = match LocalPkgList::load() {
        Some(pkglist) => {
            println!("Found local copy of the local package list.");
            pkglist
        },
        None => {
            println!("No local copy of the local package list found. Creating an empty one...");
            let pkglist = LocalPkgList::new_empty();
            pkglist.save().unwrap();
            println!("Created an empty local package list.");
            pkglist
        }
    };

    (global_pkglist, local_pkglist)
}

#[tokio::main]
async fn main() {
    // Load the package lists
    let (mut global_pkglist, mut local_pkglist) = load_pkgslists();
    dbg!(&global_pkglist);
    dbg!(&local_pkglist);
}