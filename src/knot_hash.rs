pub fn reverse_circular<T>(slice: &mut [T], mut first: usize, mut last: usize) {
    while first != last {
        slice.swap(first, last);
        first = (first + 1) % slice.len();
        if first == last {
            break;
        }
        last = if last == 0 { slice.len() - 1 } else { last - 1 };
    }
}

pub fn knot_hash_list<const LIST_LENGTH: usize, const ROUNDS: usize>(
    lengths: &[usize],
) -> Vec<usize> {
    let mut lst: Vec<usize> = (0..LIST_LENGTH).into_iter().collect();
    let mut current_position = 0;
    let mut skip_size = 0;
    for _ in 0..ROUNDS {
        for length in lengths {
            if *length > 1 {
                reverse_circular(
                    &mut lst,
                    current_position,
                    (current_position + *length - 1) % LIST_LENGTH,
                );
            }
            current_position = (current_position + *length + skip_size) % LIST_LENGTH;
            skip_size += 1;
        }
    }
    lst
}

pub fn dense_to_sparse(dense: &[usize]) -> Vec<usize> {
    assert!(
        dense.len() % 16 == 0,
        "Dense hash lenght must be divisible by 16"
    );
    dense
        .chunks(16)
        .map(|chunk| {
            chunk
                .iter()
                .copied()
                .reduce(|acc, item| acc ^ item)
                .unwrap()
        })
        .collect()
}

pub fn knot_hash(s: &str) -> Vec<u8> {
    let mut lengths: Vec<usize> = s.chars().map(|c| c as u8 as usize).collect();
    lengths.extend_from_slice(&[17, 31, 73, 47, 23]);
    dense_to_sparse(&knot_hash_list::<256, 64>(&lengths))
        .into_iter()
        .map(|i| i as u8)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_circular() {
        let mut arr = [0, 1, 2, 3];
        reverse_circular(&mut arr, 0, 3);
        assert_eq!(arr, [3, 2, 1, 0]);
        reverse_circular(&mut arr, 0, 2);
        assert_eq!(arr, [1, 2, 3, 0]);
        reverse_circular(&mut arr, 2, 0);
        assert_eq!(arr, [3, 2, 1, 0]);
        reverse_circular(&mut arr, 2, 2);
        assert_eq!(arr, [3, 2, 1, 0]);
    }

    #[test]
    fn test_knot_hash_list() {
        assert_eq!(knot_hash_list::<5, 1>(&[3, 4, 1, 5]), vec![3, 4, 2, 1, 0]);
    }

    fn vec_to_hex(v: Vec<u8>) -> String {
        v.into_iter().map(|n| format!("{:02x}", n)).collect()
    }

    #[test]
    fn test_knot_hash() {
        assert_eq!(
            vec_to_hex(knot_hash("")),
            "a2582a3a0e66e6e86e3812dcb672a272".to_owned()
        );
        assert_eq!(
            vec_to_hex(knot_hash("AoC 2017")),
            "33efeb34ea91902bb2f59c9920caa6cd".to_owned()
        );
        assert_eq!(
            vec_to_hex(knot_hash("1,2,3")),
            "3efbe78a8d82f29979031a4aa0b16a9d".to_owned()
        );
        assert_eq!(
            vec_to_hex(knot_hash("1,2,4")),
            "63960835bcdc130f0b66d7ff4f6a5a8e".to_owned()
        );
    }
}
