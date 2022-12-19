use std::collections::HashSet;
use std::hash::Hash;

pub fn is_all_unique<T: Hash + Eq>(it: impl IntoIterator<Item = T>) -> bool {
    let mut set = HashSet::new();
    for item in it {
        let is_duplicate = !set.insert(item);
        if is_duplicate {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_unique() {
        let it = 0..10;
        assert!(is_all_unique(it));
    }

    #[test]
    fn test_not_all_unique() {
        let it = [1, 2, 3, 1];
        assert!(!is_all_unique(it));
    }
}
