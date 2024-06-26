use std::cmp::Ordering::{Equal, Greater, Less};
pub fn binary_search<T: Ord>(item: &T, array: &[T]) -> Option<usize> {
    let mut is_asc = true;
    if array.len() > 1 {
	is_asc = array[0] < array[1]
    }

    let mut left = 0;
    let mut right = array.len();

    while left < right {
	let mid = left + (right - left) / 2;

	if is_asc {
	    match item.cmp(&array[mid]) {
		Equal => return Some(mid),
		Less => right = mid,
		Greater => left = mid + 1,
	    }
	} else {
	    match item.cmp(&array[mid]) {
		Equal => return Some(mid),
		Less => left = mid + 1,
		Greater => right = mid,
	    }
	}
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
	let index = binary_search(&"a", &vec![]);
	assert_eq!(index, None);
    }

    #[test]
    fn one_item() {
	let index = binary_search(&"a", &vec!["a"]);
	assert_eq!(index, Some(0));
    }

    #[test]
    fn search_strings_asc() {
	let index = binary_search(&"a", &vec!["a", "b", "c", "d", "google", "zoo"]);
	assert_eq!(index, Some(0));

	let index = binary_search(&"google", &vec!["a", "b", "c", "d", "google", "zoo"]);
	assert_eq!(index, Some(4));
    }

    #[test]
    fn search_strings_desc() {
	let index = binary_search(&"a", &vec!["zoo", "google", "d", "c", "b", "a"]);
	assert_eq!(index, Some(5));

	let index = binary_search(&"zoo", &vec!["zoo", "google", "d", "c", "b", "a"]);
	assert_eq!(index, Some(0));

	let index = binary_search(&"google", &vec!["zoo", "google", "d", "c", "b", "a"]);
	assert_eq!(index, Some(1));
    }

    #[test]
    fn search_ints_asc() {
	let index = binary_search(&4, &vec![1, 2, 3, 4]);
	assert_eq!(index, Some(3));

	let index = binary_search(&3, &vec![1, 2, 3, 4]);
	assert_eq!(index, Some(2));

	let index = binary_search(&2, &vec![1, 2, 3, 4]);
	assert_eq!(index, Some(1));

	let index = binary_search(&1, &vec![1, 2, 3, 4]);
	assert_eq!(index, Some(0));
    }

    #[test]
    fn search_ints_desc() {
	let index = binary_search(&4, &vec![4, 3, 2, 1]);
	assert_eq!(index, Some(0));

	let index = binary_search(&3, &vec![4, 3, 2, 1]);
	assert_eq!(index, Some(1));

	let index = binary_search(&2, &vec![4, 3, 2, 1]);
	assert_eq!(index, Some(2));

	let index = binary_search(&1, &vec![4, 3, 2, 1]);
	assert_eq!(index, Some(3));
    }

    #[test]
    fn not_found() {
	let index = binary_search(&5, &vec![1, 2, 3, 4]);
	assert_eq!(index, None);

	let index = binary_search(&5, &vec![4, 3, 2, 1]);
	assert_eq!(index, None);
    }
}
