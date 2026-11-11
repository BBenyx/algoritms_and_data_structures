use sorting_algorithms::sorting::{
    bubble_sort::bubble_sort,
    shaker_sort::shaker_sort,
    insertion_sort::insertion_sort,
    selection_sort::selection_sort,
    heap_sort::heap_sort,
};
use super::test_cases;

const SAMPLE_RANGE: std::ops::RangeInclusive<usize> = 1000..=4000;
const STEP_SIZE: usize = 1000;
const SAMPLE_SIZE: usize = 30;


type ListCreator = fn(usize) -> Vec<u32>;
type ListSorter<T> = fn(&mut [T]);


pub struct TestConfiguration<T> {
    pub range: std::ops::RangeInclusive<usize>,
    pub step: usize,
    pub sample_size: usize,
    pub algorithms: Vec<(&'static str, ListSorter<T>)>,
    pub distributions: Vec<(&'static str, ListCreator)>,
}

impl<T: Ord> TestConfiguration<T> {
    pub fn in_place_sorters() -> Self {
        Self {
            range: SAMPLE_RANGE,
            step: STEP_SIZE,
            sample_size: SAMPLE_SIZE,
            algorithms: vec![
                ("Bubble sort", bubble_sort),
                ("Shaker sort", shaker_sort),
                ("Insertion sort", insertion_sort),
                ("Selection sort", selection_sort),
                ("Heap sort", heap_sort),
            ],
            distributions: vec![
                ("Ordered", test_cases::ordered),
                ("Reverse", test_cases::reverse_ordered),
                ("Random", test_cases::randoms),
            ]
        }
    }

    /*pub fn non_in_place_sorters() -> Self {
        Self {
            range: SAMPLE_RANGE,
            step: STEP_SIZE,
            sample_size: SAMPLE_SIZE,
            algorithms: vec![
                todo!()
            ],
            distributions: vec![
                ("Ordered", test_cases::ordered),
                ("Reverse", test_cases::reverse_ordered),
                ("Random", test_cases::randoms),
            ]
        }
    }*/
}

impl<T: Ord> Default for TestConfiguration<T> {
    fn default() -> Self {
        Self {
            range: SAMPLE_RANGE,
            step: STEP_SIZE,
            sample_size: SAMPLE_SIZE,
            algorithms: Vec::new(),
            distributions: vec![
                ("Ordered", test_cases::ordered),
                ("Reverse", test_cases::reverse_ordered),
                ("Random", test_cases::randoms),
            ]
        }
    }
}