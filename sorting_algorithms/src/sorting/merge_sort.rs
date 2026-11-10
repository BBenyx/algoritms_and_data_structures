use crate::sorting::bubble_sort::bubble_sort;

pub fn merge_sort<T: PartialOrd + Clone>(list: &[T]) ->  Vec<T> {
    if list.len() < 2 {
        return Vec::from(list);
    } else if list.len() == 2 {
        return if list[0] > list[1] {
            vec![list[1].clone(), list[0].clone()]
        } else {
            Vec::from(list)
        }
    }

    let middle_idx = list.len() / 2;
    let mut left = merge_sort(&list[..middle_idx]);
    let mut right = merge_sort(&list[middle_idx..]);

    let mut ordered = Vec::new();

    loop {
        if left.is_empty() {
            ordered.append(&mut right);
            break;
        } else if right.is_empty() {
            ordered.append(&mut left);
            break;
        }

        if left[0] < right[0] {
            ordered.push(left.remove(0));
        } else {
            ordered.push(right.remove(0));
        }
    }

    ordered
}