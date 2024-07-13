use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use cocoa::foundation::NSInteger;

pub fn from_str(s: &str) -> NSInteger {
    let mut hasher = DefaultHasher::new();

    s.hash(&mut hasher);

    let hash = hasher.finish();

    hash as NSInteger
}
