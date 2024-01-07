use serde_json::to_string_pretty;
use crate::global_pkglist::GlobalPkgList;

/// Generates and save an empty pakage list.
pub fn gen_empty_pkglist() {
    let pkglist = GlobalPkgList::new_empty();
    let json = to_string_pretty(&pkglist).unwrap();
    std::fs::write("pkglist.json", json).unwrap();
}