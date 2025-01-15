pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn insertion_sort(mut arr_numbers: Vec<i32>) -> Vec<i32> {
    for j in 1..arr_numbers.len() {
        let j: i32 = j as i32;
        let key = arr_numbers[j as usize];
        let mut i: i32 = j - 1;

        while i >= 0 && arr_numbers[i as usize] > key {
            arr_numbers[(i + 1) as usize] = arr_numbers[i as usize];
            i = i - 1;
        }

        arr_numbers[(i + 1) as usize] = key;
    }

    arr_numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_insertion_sort() {
        let numbers = vec![5, 2, 4, 6, 1, 3];
        let sorted_numbers = insertion_sort(numbers);
        println!("{:?}", &sorted_numbers);

        assert_eq!(sorted_numbers, vec![1, 2, 3, 4, 5, 6]);
    }
}
