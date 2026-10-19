use std::{error::Error, time::Instant};
use rand::RngExt;
use csv::Writer;
use sorting_algorithms::sorting::*;

type Sorter<T> = fn(&mut [T]);


fn main() {
    let csv_data = write_builder(100, 10000, 200);
    write_to_csv("Sorintg_algorithm_runtimes".to_string(), csv_data);
}

fn write_builder(from: usize, to: usize , step: usize) -> Vec<Vec<String>> {
    assert!(from < to);
    let mut datas = vec![
        ["Ordered (bubble sort)","Ordered (shaker sort)","Ordered (insertion sort)","Ordered (selection sort)",
        "Reverse ordered (bubble sort)","Reverse ordered (shaker sort)","Reverse ordered (insertion sort)","Reverse ordered (selection sort)",
        "Random (bubble sort)","Random (shaker sort)","Random (insertion sort)","Random (selection sort)",
        "Binary alternating (bubble sort)","Binary alternating (shaker sort)","Binary alternating (insertion sort)","Binary alternating (selection sort)"].
        into_iter().map(String::from).collect()];

    let list_creators = [ordered, reverse_ordered, randoms, binary_alternating];
    let sorters: [Sorter<i32>; 4] = [bubble_sort, shaker_sort, insertion_sort, selection_sort];

    let to = to - ((to - from) % step);
    for n in (from..to).step_by(step) {
        let mut row = Vec::new();
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

fn write_to_csv<T: AsRef<[u8]>>(file_name: String, data: Vec<Vec<T>>) -> Result<(), Box<dyn Error>> {
    let file_name = file_name + ".csv";
    let mut wtr = Writer::from_path(&file_name)?;

    for record in data {
        match wtr.write_record(record) {
            Err(err) => panic!("Failed to write file {file_name}! {err}"),
            _ => (),
        }
    }

    wtr.flush()?;
    Ok(())
}


fn run_time_of<T: PartialOrd>(func: Sorter<T>, val: &mut [T]) -> u128 {
    let start = Instant::now();
    func(val);

    start.elapsed().as_micros()
}


fn ordered(n: usize) -> Vec<i32> {
    let mut list = Vec::new();

    for i in 0..n {
        list.push(i as i32);
    }
    list
}

fn reverse_ordered(n: usize) -> Vec<i32> {
    let mut list = Vec::new();

    for i in (0..n).rev() {
        list.push(i as i32);
    }
    list
}

fn randoms(n: usize) -> Vec<i32> {
    let mut list = Vec::new();

    for i in 0..n {
        let random = rand::rng().random_range(0..3141);
        list.push(random);
    }
    list
}

fn binary_alternating(n: usize) -> Vec<i32> {
    let mut list = Vec::new();
    
    for i in 0..n {
        list.push((i % 2) as i32);
    }
    list
}

