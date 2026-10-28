use std::{error::Error, time::Instant};
use sorting_algorithms::sorting::{
    bubble_sort::bubble_sort,
    shaker_sort::shaker_sort,
    insertion_sort::insertion_sort,
    selection_sort::selection_sort,
    heap_sort::heap_sort,
};

mod test_cases;
mod csv_writer;

type ListSorter<T> = fn(&mut [T]);

fn main() -> Result<(), Box<dyn Error>>{
    // Creates the results folder if it doesn't exists
    std::fs::create_dir_all("results")?;

    let csv_data = write_builder(100, 1000, 200);

    csv_writer::write_to_csv(
        "results/Sorting_algorithms_runtimesN".to_string(),
        csv_data
    )?;

    Ok(())
}

fn write_builder(from: usize, to: usize , step: usize) -> Vec<Vec<String>> {
    assert!(from < to);
    let mut datas = vec![
        ["Input (n)","Ordered (bubble sort)","Ordered (shaker sort)","Ordered (insertion sort)","Ordered (selection sort)","Ordered (heap sort)",
        "Reverse ordered (bubble sort)","Reverse ordered (shaker sort)","Reverse ordered (insertion sort)","Reverse ordered (selection sort)", "Reverse ordered (heap sort)",
        "Random (bubble sort)","Random (shaker sort)","Random (insertion sort)","Random (selection sort)","Random (heap sort)"].
        into_iter().map(String::from).collect()];

    let list_creators = [test_cases::ordered, test_cases::reverse_ordered, test_cases::randoms];
    let sorters: [ListSorter<i32>; 5] = [bubble_sort, shaker_sort, insertion_sort, selection_sort, heap_sort];

    let to = to - ((to - from) % step);
    for n in (from..to).step_by(step) {
        let mut row = Vec::new();
        row.push(n.to_string());

        for creator in list_creators {
            let list = creator(n);

            for sorter in sorters {
                let mut data = list.clone();
                row.push(run_time_of(sorter, &mut data).to_string());
            }
        }
        datas.push(row);
    }
    datas
}

fn run_time_of<T: PartialOrd>(func: ListSorter<T>, val: &mut [T]) -> u128 {
    let start = Instant::now();
    func(val);

    start.elapsed().as_micros()
}



