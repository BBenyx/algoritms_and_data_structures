pub fn counting_sort(list: &[u32]) -> Vec<u32> {
    let (max, min): (usize, usize) = match (list.iter().max(), list.iter().min()) {
        (Some(max), Some(min)) => (*max as usize, *min as usize),
        _ => return Vec::from(list),
    };

    let mut cnt_arr: Vec<u32> = vec![0; max + 1 - min];

    for elem in list{
        let idx = *elem as usize - min;
        cnt_arr[idx] += 1;
    }
    for i in 1..cnt_arr.len() {
        cnt_arr[i] += cnt_arr[i - 1];
    }let mut ordered: Vec<u32> = vec![0; list.len()];

    for elem in list {
        let cnt_idx = *elem as usize - min;

        ordered[cnt_arr[cnt_idx] as usize - 1] = *elem;
        cnt_arr[cnt_idx] -= 1;
    }
    ordered
}