//! Utilities to lookup unicode characters

use rayon::prelude::*;
use unicode_names2::name;

/// Struct comprising the character and its name
#[derive(Debug, Clone, Copy)]
pub struct CodePoint(pub char, pub &'static str);

/// Lookup utf characters based on their names
pub fn lookup_by_name(s: &str, data: &mut Vec<CodePoint>) {
    data.clear();

    if s.len() >= 3 { 
        let canonical = s.to_uppercase();
        let chars = (' ' ..std::char::MAX).into_par_iter();
        let found = chars
            .filter_map(|c| {
                if let Some(mut n) = name(c) {
                    if let Some(n) = n.next() {
                        Some(CodePoint(c, n))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .filter(|cp| cp.1.contains(canonical.as_str()));
        data.par_extend(found);
    }
}