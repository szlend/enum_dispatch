//! Procedural macros don't offer a good way to store information between macro invocations.  In
//! addition, all syntax-related structures implement `!Send` and `!Sync`, making it impossible to
//! keep them in any sort of static storage. This module uses some workarounds to add that
//! functionality.
//!
//! Fortunately, `TokenStream`s can be converted to and from `String`s, which can be stored
//! statically. Unfortunately, doing so strips any related `Span` information, preventing error
//! messages from being as informative as they could be. For now, it seems this is the best option
//! available.
use proc_macro::Ident;
use quote::ToTokens;
use syn;

use lazy_static::lazy_static;

use std::collections::HashMap;
use std::sync::Mutex;

use crate::enum_dispatch_item;

// Magical storage for trait definitions so that they can be used when parsing other syntax
// structures.
lazy_static! {
    static ref TRAIT_DEFS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref ENUM_DEFS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref DEFERRED_LINKS: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
}

/// Store a trait definition for future reference.
pub fn cache_trait(item: syn::ItemTrait) {
    let identname = item.ident.to_string();
    TRAIT_DEFS
        .lock()
        .unwrap()
        .insert(identname, item.into_token_stream().to_string());
}

/// Store an enum definition for future reference.
pub fn cache_enum_dispatch(item: enum_dispatch_item::EnumDispatchItem) {
    let identname = item.ident.to_string();
    ENUM_DEFS
        .lock()
        .unwrap()
        .insert(identname, item.into_token_stream().to_string());
}

/// Cache a "link" to be fulfilled once the needed definition is also cached.
pub fn defer_link(needed: &Ident, cached: &::proc_macro2::Ident) {
    // cached is a proc_macro2::Ident until there is a good way to convert into proc_macro::Ident.
    let (needed, cached) = (needed.to_string(), cached.to_string());
    let mut deferred_links = DEFERRED_LINKS.lock().unwrap();
    if deferred_links.contains_key(&needed) {
        deferred_links.get_mut(&needed).unwrap().push(cached.to_owned());
    } else {
        deferred_links.insert(needed.to_owned(), vec![cached.to_owned()]);
    }
    if deferred_links.contains_key(&cached) {
        deferred_links.get_mut(&cached).unwrap().push(needed);
    } else {
        deferred_links.insert(cached, vec![needed]);
    }
}

/// Returns a list of all of the trait definitions that were previously linked to the supplied enum
/// name.
pub fn fulfilled_by_enum(defname: &::proc_macro2::Ident) -> Vec<syn::ItemTrait> {
    let idents = match DEFERRED_LINKS.lock().unwrap().remove_entry(&defname.to_string()) {
        Some((_, links)) => links,
        None => vec![],
    };
    idents.iter().filter_map(|ident_string| {
        match TRAIT_DEFS.lock().unwrap().get(ident_string) {
            Some(entry) => Some(syn::parse(entry.parse().unwrap()).unwrap()),
            None => None,
        }
    }).collect()
}

/// Returns a list of all of the enum definitions that were previously linked to the supplied trait
/// name.
pub fn fulfilled_by_trait(defname: &::proc_macro2::Ident) -> Vec<enum_dispatch_item::EnumDispatchItem> {
    let idents = match DEFERRED_LINKS.lock().unwrap().remove_entry(&defname.to_string()) {
        Some((_, links)) => links,
        None => vec![],
    };
    idents.iter().filter_map(|ident_string| {
        match ENUM_DEFS.lock().unwrap().get(ident_string) {
            Some(entry) => Some(syn::parse(entry.parse().unwrap()).unwrap()),
            None => None,
        }
    }).collect()
}

pub fn remove_entry(defname: &::proc_macro2::Ident) {
    DEFERRED_LINKS.lock().unwrap().remove_entry(&defname.to_string());
}

