use std::collections::HashMap;

pub fn mean(list: &[i32]) -> Option<f64> {
    let len = list.len();
    if len > 0 {
        Some(list.iter().sum::<i32>() as f64 / len as f64)
    } else {
        None
    }
}

pub fn median(list: &[i32]) -> Option<i32> {
    if list.len() > 0 {
        Some(list[list.len() / 2])
    } else {
        None
    }
}

pub fn mode(list: &[i32]) -> Option<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for &i in list.iter() {
        *map.entry(i).or_insert(0) += 1;
    }

    let result = map.iter().reduce(|a, b| if a.1 > b.1 { a } else { b });

    match result {
        Some(a) => Some(*a.0),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_basic() {
        assert_eq!(Some(2.5), mean(&[1, 2, 3, 4]));
    }

    #[test]
    fn median_basic() {
        assert_eq!(Some(2), median(&[1, 2, 3]));
    }

    #[test]
    fn median_empty() {
        assert_eq!(None, median(&[]));
    }

    #[test]
    fn mode_basic() {
        assert_eq!(Some(1), mode(&[1, 1, 1, 2, 2, 3]));
    }
}
