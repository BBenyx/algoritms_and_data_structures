pub fn merge_sort<T: PartialOrd>(list: &[T]) -> Vec<&T> {
    if list.len() < 2 {
        return list.iter().collect();
    }

    let middle_idx = list.len() / 2;
    let left_sorted = merge_sort(&list[..middle_idx]);
    let right_sorted = merge_sort(&list[middle_idx..]);

    let mut left = left_sorted.iter().peekable();
    let mut right = right_sorted.iter().peekable();

    let mut ordered = Vec::with_capacity(list.len());

    loop {
        if left.peek().is_none() {
            for i in right {
                ordered.push(*i);
            }
            break;
        } else if right.peek().is_none() {
            for i in left {
                ordered.push(*i);
            }
            break;
        }

        if let (Some(current_left), Some(current_right)) = (left.peek(), right.peek()) {
            if current_left <= current_right {
                ordered.push(**current_left);
                left.next();
            } else {
                ordered.push(**current_right);
                right.next();
            }
        }
    }
    ordered
}
