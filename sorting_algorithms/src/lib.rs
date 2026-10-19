pub mod sorting {
    pub fn bubble_sort<T: PartialOrd>(list: &mut [T]) {
        if list.is_empty() {return}
        
        let mut swap_flag = true;

        while swap_flag {
            swap_flag = false;

            for i in 0..list.len() - 1 {
                if list[i] > list[i + 1] {
                    list.swap(i, i+1);
                    swap_flag = true;
                }
            }
        }
    }

    pub fn shaker_sort<T: PartialOrd>(list: &mut [T]) {
        if list.is_empty() {return}

        let mut swap_flag = true;

        while swap_flag {
            swap_flag = false;
            for i in 0..list.len() - 1 {
                if list[i] > list[i + 1] {
                    list.swap(i, i+1);
                    swap_flag = true;
                }
            }
            for i in (0..list.len() - 1).rev() {
                if list[i] > list[i + 1] {
                    list.swap(i, i+1);
                    swap_flag = true;
                }
            }
        }
    }

    pub fn insertion_sort<T: PartialOrd>(list: &mut [T]) {
        if list.is_empty() { return; }

        for key in 1..list.len() {
            let mut left_next = key - 1;

            while list[left_next] > list[left_next + 1] {
                list.swap(left_next + 1, left_next);

                if left_next == 0 {
                    break;      
                } else {
                    left_next -= 1;
                } 
            }
        }
    }

    pub fn selection_sort<T: PartialOrd>(list: &mut [T]) {
        if list.is_empty() { return; }

        for i in 0..list.len() {
            let mut smallest = i;
            
            for n in i+1..list.len() {
                if list[n] < list[smallest] {
                    smallest = n;
                }
            }
            for n in (i+1..=smallest).rev() {
                list.swap(n-1, n);
            }
        }
    }
}