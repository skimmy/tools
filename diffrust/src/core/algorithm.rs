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
                iy = j+1;
                break;
            }
            j += 1;
        }
        ix += 1;
    }
    2.0*nt / (vx.len() + vy.len()) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bigrams_empty() {
        let v = bigrams("");
        assert_eq!(0, v.len(), "Non-empty bigrams vector");
    }

    #[test]
    fn bigrams_no_repetition() {
        let s = "abcd";
        let v = bigrams(s);
        assert_eq!(v, vec!["ab","bc", "cd"]);
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
        assert_eq!(2.0 / 6.0, dice_coefficient(x,y));
    }

    #[test]
    fn dice_without_repetition() {
        let x = "yz";
        let y = "yzuvx";
        assert_eq!(2.0 / 5.0, dice_coefficient(x, y));
    }
}