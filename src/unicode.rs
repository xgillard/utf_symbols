//! Utilities to lookup unicode characters

use rayon::prelude::*;
use unicode_names2::name;

/// Struct comprising the character and its name
#[derive(Debug, Clone)]
pub struct CodePoint(pub char, pub String);

/// Lookup utf characters based on their names
pub fn lookup_by_name(s: &str, data: &mut Vec<CodePoint>) {
    data.clear();

    if s.len() >= 3 { 
        let canonical = s.to_uppercase();
        let chars = (' ' ..std::char::MAX).into_par_iter();
        let found = chars
            .filter_map(|c| {
                if let Some(n) = name(c) {
                    let name = n.collect::<String>();
                    Some(CodePoint(c, name))
                } else {
                    None
                }
            })
            .filter(|cp| cp.1.contains(canonical.as_str()));
        data.par_extend(found);
        data.sort_unstable_by_key(|x| x.1.len());
    }
}