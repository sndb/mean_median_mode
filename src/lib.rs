use std::collections::HashMap;

pub fn mean(list: &[i32]) -> Option<f64> {
    if list.len() > 0 {
        Some(list.iter().sum::<i32>() as f64 / list.len() as f64)
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
    let mut number_count: HashMap<i32, i32> = HashMap::new();

    list.iter()
        .for_each(|&i| *number_count.entry(i).or_insert(0) += 1);

    match number_count
        .iter()
        .reduce(|a, b| if a.1 > b.1 { a } else { b })
    {
        Some((&a, _)) => Some(a),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        assert_eq!(Some(2.5), mean(&[1, 2, 3, 4]));
    }

    #[test]
    fn test_mean_empty() {
        assert_eq!(None, mean(&[]));
    }

    #[test]
    fn test_median() {
        assert_eq!(Some(2), median(&[1, 2, 3]));
    }

    #[test]
    fn test_median_empty() {
        assert_eq!(None, median(&[]));
    }

    #[test]
    fn test_mode() {
        assert_eq!(Some(1), mode(&[1, 1, 1, 2, 2, 3]));
        assert_eq!(Some(2), mode(&[1, 2, 2, 2, 3, 3]));
        assert_eq!(Some(3), mode(&[1, 2, 2, 3, 3, 3]));
    }

    #[test]
    fn test_mode_empty() {
        assert_eq!(None, mode(&[]));
    }
}
