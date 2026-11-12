use crate::sorting::insertion_sort::insertion_sort;

pub fn bucket_sort(list: &[usize]) -> Vec<usize> {
    let mut buckets = vec![Vec::new(); list.len()];

    let Some(max) = list.iter().max() else {
        return Vec::from(list);
    };

    for elem in list {
        let normalized = *elem as f64 / (*max + 1) as f64;
        let bucket_idx = (buckets.len() as f64 * normalized) as usize;

        buckets[bucket_idx].push(*elem);
    }
    for bucket in &mut buckets {
        if bucket.len() > 100 {
            *bucket = bucket_sort(bucket);
        } else {
            insertion_sort(bucket);
        }
    }

    let mut ordered = Vec::with_capacity(list.len());
    for mut bucket in buckets {
        ordered.append(&mut bucket);
    }
    ordered
}
