fn bubble_sort<T: Ord>(values: &mut[T]) {
    let mut n = values.len();
    let mut swapped = true;

    while swapped {
        swapped = false;

        for i in 1..n {
            if values[i - 1] > values[i] {
                values.swap(i - 1, i);
                swapped = true;
            }
        }

        n = n - 1;
    }
}