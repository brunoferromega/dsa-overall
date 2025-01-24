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

pub fn merge_sort(numbers: &mut Vec<i32>, left: i32, right: i32) {
    if left >= right {
        return;
    }

    let mid = (left + right) / 2;
    merge_sort(numbers, left, mid);
    merge_sort(numbers, mid + 1, right);
    merge(numbers, left, mid, right);
}

fn merge(numbers_to: &mut Vec<i32>, left: i32, middle: i32, right: i32) {
    let left_mid: std::ops::Range<usize> = (left as usize)..(middle as usize);
    let mid_right: std::ops::Range<usize> = (middle as usize)..(right as usize);

    let vec_l = numbers_to[left_mid].to_vec(); // Clone the left part
    let vec_r = numbers_to[mid_right].to_vec(); // Clone the right part

    let (mut i, mut j) = (0, 0);
    let mut k = left;

    while i < vec_l.len() && j < vec_r.len() {
        if vec_l[i as usize] <= vec_r[j as usize] {
            numbers_to[k as usize] = vec_l[i as usize];
            i += 1;
        } else {
            numbers_to[k as usize] = vec_r[j as usize];
            j += 1;
        }
        k += 1;
    }

    // Copy remaining elements from vec_l
    while i < vec_l.len() {
        numbers_to[k as usize] = vec_l[i as usize];
        i += 1;
        k += 1;
    }

    // Copy remaining elements from vec_r
    while j < vec_r.len() {
        numbers_to[k as usize] = vec_r[j as usize];
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_insertion_sort() {
        let numbers = vec![5, 2, 4, 6, 1, 3];
        let sorted_numbers = insertion_sort(numbers);

        assert_eq!(sorted_numbers, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn it_selection_sort() {
        let numbers = vec![5, 2, 4, 6, 1, 3];
        let sorted_numbers = selection_sort(numbers);

        assert_eq!(sorted_numbers, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn it_merge_sort() {
        let mut numbers: Vec<i32> = vec![11, 12, 13, 5, 6, 7];
        let (left, right) = (0, numbers.len() as i32);
        merge_sort(&mut numbers, left, right);

        assert_eq!(numbers, vec![5, 6, 7, 11, 12, 13]);
    }
}
