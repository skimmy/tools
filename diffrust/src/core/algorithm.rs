use std::{ops::Range, path::Path, str::MatchIndices};

/// Support algorithms for diffrust.
///
/// Contains algorithms for string slices comparison:
/// - dice coefficient (with bigrams)
/// - exact substring matching in name part of a Path
///
/// This module should only use structs and types from the standard
/// library so that it could be extracted from the project and used in
/// other ones (or made a crate out of it). If the modules becomes too
/// big, consider making a distinct crate within the project.

fn bigrams(s: &str) -> Vec<String> {
    let v: Vec<char> = s.chars().collect();
    v.windows(2).map(|pair| pair.iter().collect()).collect()
}

/// The *Dice coefficient* is used to compare strings for their similarity.
///
/// The coefficients is computed counting the *bigrams* on the two strings
/// and their intersection. More precisely the coefficient is define by
/// `2*nt / (nx+nt)` where nt is the number of bigrams present on both strings,
/// `nx` is the number of bigrams in `x` and `ny` the number of bigrams in `y`.
pub fn dice_coefficient(x: &str, y: &str) -> f64 {
    if x.len() < 2 || y.len() < 2 {
        return 0.0;
    }
    let mut vx = bigrams(x.to_lowercase().as_str());
    let mut vy = bigrams(y.to_ascii_lowercase().as_str());
    vx.sort();
    vy.sort();
    let mut nt = 0.0;
    let mut ix = 0;
    let mut iy = 0;
    while ix < vx.len() {
        let mut j = iy;
        while j < vy.len() {
            if vx[ix] == vy[j] {
                nt += 1.0;
                iy = j + 1;
                break;
            }
            j += 1;
        }
        ix += 1;
    }
    2.0 * nt / (vx.len() + vy.len()) as f64
}

/// Returns all matches of the given pattern in the name part of the
/// given path.
pub fn substrings_in_name(path: &Path, pattern: &str) -> Vec<Range<usize>> {
    if let Some(name) = path.file_name() {
        return name
            .to_str()
            .unwrap_or("")
            .to_lowercase()
            .match_indices(&pattern.to_lowercase())
            .map(|(i, _)| Range {
                start: i,
                end: i + pattern.len(),
            })
            .collect();
    }
    vec![]
}

/// Splits a string into substrings at the given indexes.
fn _split_at_indexes(str: &str, indexes: Vec<usize>) -> Vec<&str> {
    let mut splits = vec![];
    let mut i = 0;
    for index in indexes {
        splits.push(&str[i..index]);
        i = index;
    }
    splits.push(&str[i..]);
    splits
}

fn _split_at_match_indices<'a>(str: &'a str, matches: MatchIndices<&str>) -> Vec<&'a str> 
{
    let mut indices = vec![];
    for m in matches {
        indices.push(m.0);
        indices.push(m.0 + m.1.len());
    }
    _split_at_indexes(str,indices)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn bigrams_empty() {
        let v = bigrams("");
        assert_eq!(v.len(), 0, "Non-empty bigrams vector");
    }

    #[test]
    fn bigrams_no_repetition() {
        let s = "abcd";
        let v = bigrams(s);
        assert_eq!(v, vec!["ab", "bc", "cd"]);
    }

    #[test]
    fn bigrams_with_repetition() {
        let s = "abbcab";
        let v = bigrams(s);
        assert_eq!(v, vec!["ab", "bb", "bc", "ca", "ab"]);
    }

    #[test]
    fn dice_is_zero() {
        let x = "abcd";
        let y = "efg";
        assert_eq!(0.0, dice_coefficient(x, y));
    }

    #[test]
    fn dice_with_empty() {
        let x = "";
        let y = "ab";
        assert_eq!(0.0, dice_coefficient(x, y));
        assert_eq!(0.0, dice_coefficient(x, x));
    }

    #[test]
    fn dice_with_singleton() {
        let x = "a";
        let y = "ab";
        assert_eq!(0.0, dice_coefficient(x, y));
    }

    #[test]
    fn dice_is_one() {
        let x = "abcd";
        let y = "abcd";
        assert_eq!(1.0, dice_coefficient(x, y));
    }

    #[test]
    fn dice_with_repetition() {
        let x = "abab";
        let y = "acba";
        assert_eq!(2.0 / 6.0, dice_coefficient(x, y));
    }

    #[test]
    fn dice_without_repetition() {
        let x = "yz";
        let y = "yzuvx";
        assert_eq!(2.0 / 5.0, dice_coefficient(x, y));
    }

    #[test]
    fn substring_in_path_name() {
        let path = PathBuf::from("/tmp/ab.txt");
        let pattern = "ab";
        assert_eq!(
            substrings_in_name(&path, pattern),
            // We obtain the index on the name
            [Range {start: 0, end: 2}],
            "Substring in path name not matched"
        );

        let path = PathBuf::from("books/Introduction.to.Algorithms.pdf");
        let pattern = "Introduction";
        assert!(
            substrings_in_name(&path, pattern).len() > 0,
            "Substring in path name not matched (mixed case test)"
        )
    }

    #[test]
    fn split_by_indexes() {
        let s = "/A/b/ccc.txt";
        let indexes = vec![0,2,4];
        let splits = _split_at_indexes(&s, indexes);
        assert_eq!(splits.len(), 4, "Incorrect size of split result");
        assert_eq!(
            splits,
            vec!["", "/A", "/b", "/ccc.txt"],
            "Incorrect split"    
        )
    }

    #[test]
    fn split_by_matches_indices() {
        let s = "/c/b/c.txt";
        let matches = s.match_indices("c");
        let splits = _split_at_match_indices(s, matches);
        assert_eq!(splits, vec!["/", "c", "/b/", "c", ".txt"]);
    }
}
