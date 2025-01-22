pub fn insertion_sort(mut arr_numbers: Vec<i32>) -> Vec<i32> {
    let range: Vec<i32> = (1..arr_numbers.len()).map(|x| x as i32).collect();

    for j in range {
        let key = arr_numbers[j as usize];
        let mut i = j - 1;

        while i >= 0 && arr_numbers[i as usize] > key {
            arr_numbers[(i + 1) as usize] = arr_numbers[i as usize];
            i = i - 1;
        }

        arr_numbers[(i + 1) as usize] = key;
    }

    arr_numbers
}

pub fn selection_sort(mut numbers: Vec<i32>) -> Vec<i32> {
    let range: Vec<i32> = (0..numbers.len()).map(|x| x as i32).collect();

    for i in &range {
        let mut lower_pos = *i;

        for j in (i + 1)..(range.len() as i32) {
            if numbers[j as usize] < numbers[lower_pos as usize] {
                lower_pos = j;
            }
        }

        numbers.swap(*i as usize, lower_pos as usize);
    }

    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_insertion_sort() {
        let numbers = vec![5, 2, 4, 6, 1, 3];
        let sorted_numbers = insertion_sort(numbers);
        dbg!(&sorted_numbers);

        assert_eq!(sorted_numbers, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn it_selection_sort() {
        let numbers = vec![5, 2, 4, 6, 1, 3];
        let sorted_numbers = selection_sort(numbers);
        dbg!(&sorted_numbers);

        assert_eq!(sorted_numbers, vec![1, 2, 3, 4, 5, 6]);
    }
}
