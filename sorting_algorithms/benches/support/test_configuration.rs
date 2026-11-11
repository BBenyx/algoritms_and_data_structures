use super::test_cases;
use sorting_algorithms::sorting::{
    bubble_sort::bubble_sort, heap_sort::heap_sort, insertion_sort::insertion_sort,
    merge_sort::merge_sort, selection_sort::selection_sort, shaker_sort::shaker_sort,
    counting_sort::counting_sort, quick_sort::quick_sort,
};

const SAMPLE_RANGE: std::ops::RangeInclusive<usize> = 1000..=4000;
const STEP_SIZE: usize = 1000;
const SAMPLE_SIZE: usize = 30;

type ListCreator = fn(usize) -> Vec<u32>;
type InPlaceSorter<T> = fn(&mut [T]);
type NonInPlaceSorter<T> = fn(&[T]) -> Vec<&T>;
type NonComparisonSorter = fn(&[u32]) -> Vec<u32>;

pub enum SorterType<T> {
    InPlace(InPlaceSorter<T>),
    NotInPlace(NonInPlaceSorter<T>),
    NonComparison(NonComparisonSorter),
}

pub struct TestConfiguration<T> {
    pub range: std::ops::RangeInclusive<usize>,
    pub step: usize,
    pub sample_size: usize,
    pub algorithms: Vec<(&'static str, SorterType<T>)>,
    pub distributions: Vec<(&'static str, ListCreator)>,
}

impl<T: Ord> TestConfiguration<T> {
    pub fn in_place_sorters() -> Self {
        Self {
            range: SAMPLE_RANGE,
            step: STEP_SIZE,
            sample_size: SAMPLE_SIZE,
            algorithms: vec![
                ("Bubble sort", SorterType::InPlace(bubble_sort)),
                ("Shaker sort", SorterType::InPlace(shaker_sort)),
                ("Insertion sort", SorterType::InPlace(insertion_sort)),
                ("Selection sort", SorterType::InPlace(selection_sort)),
                ("Heap sort", SorterType::InPlace(heap_sort)),
                ("Quick sort", SorterType::InPlace(quick_sort)),
            ],
            distributions: vec![
                ("Ordered", test_cases::ordered),
                ("Reverse", test_cases::reverse_ordered),
                ("Random", test_cases::randoms),
            ],
        }
    }

    pub fn non_in_place_sorters() -> Self {
        Self {
            range: SAMPLE_RANGE,
            step: STEP_SIZE,
            sample_size: SAMPLE_SIZE,
            algorithms: vec![
                ("Merge sort", SorterType::NotInPlace(merge_sort))
            ],
            distributions: vec![
                ("Ordered", test_cases::ordered),
                ("Reverse", test_cases::reverse_ordered),
                ("Random", test_cases::randoms),
            ],
        }
    }

    pub fn non_comparison_sorters() -> Self {
        Self {
            range: SAMPLE_RANGE,
            step: STEP_SIZE,
            sample_size: SAMPLE_SIZE,
            algorithms: vec![
                ("Counting sort", SorterType::NonComparison(counting_sort))
            ],
            distributions: vec![
                ("Ordered", test_cases::ordered),
                ("Reverse", test_cases::reverse_ordered),
                ("Random", test_cases::randoms),
            ],
        }
    }
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
            ],
        }
    }
}
