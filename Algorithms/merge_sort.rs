pub mod merge_sort {
    fn merge<T: PartialOrd + Copy>(elements: &mut [T]) {
        let mid = elements.len() / 2;
        let workarea = elements.to_vec();
        let mut i1 = 0;
        let mut i2 = mid;
        let mut counter = 0;
        while i1 < mid && i2 < elements.len() {
            if workarea[i1] <= workarea[i2] {
                elements[counter] = workarea[i1];
                i1 += 1;
            } else {
                elements[counter] = workarea[i2];
                i2 += 1;
            };
            counter += 1;
        }
        while i1 < mid {
            elements[counter] = workarea[i1];
            i1 += 1;
            counter += 1;
        }
        while i2 < elements.len() {
            elements[counter] = workarea[i2];
            i2 += 1;
            counter += 1;
        }
    }
    pub fn merge_sort<T: PartialOrd + Copy>(elements: &mut [T]) {
        if elements.len() <= 1 {
            return;
        }
        let mid = elements.len() / 2;
        {
            let (elements_a, elements_b) = elements.split_at_mut(mid);
            merge_sort(elements_a);
            merge_sort(elements_b);
        }
        merge(elements);
    }

}

#[cfg(test)]
extern crate rand;
#[cfg(test)]
mod tests {
    use rand;
    use std;
    use std::cmp::{min, max};
    use rand::Rng;
    use std::time::{Duration, Instant};
    use merge_sort::merge_sort;

    const CASES_CNT : u32 = 100;
    const MIN_VEC_SIZE : usize = 50000;
    const MAX_VEC_SIZE : usize = 100000;


    fn check_if_sorted<T: PartialOrd>(v: &[T]) -> bool {
        for i in 0..v.len() - 1 {
            if v[i] > v[i+1] {
                return false;
            }
        }
        return true;
    }

    fn generate_rand_vec<T: Copy>(len: usize, rand_gen_fn: &Fn() -> T) -> Vec<T> {
        let mut v = Vec::with_capacity(len);
        for _ in 0..len {
            v.push(rand_gen_fn());
        }
        v
    }

    fn test_sort_function_for_inp<T: Ord + Copy + std::fmt::Debug>(sort_fn: &Fn(&mut [T]) ,v: &mut [T]) -> Duration {

        let now = Instant::now();
        sort_fn(v);
        let time_elapsed = Instant::now() - now;

        assert!(check_if_sorted(v));
        return time_elapsed;
    }

    fn test_sort_function<T: Ord + Copy + std::fmt::Debug>(fn_name: &str, sort_fn: &Fn(&mut [T]), rand_gen_fn: &Fn() -> T) {
        let mut average_time = Duration::new(0, 0);
        let mut min_time = Duration::new(10000000, 0);
        let mut max_time = Duration::new(0, 0);
        for _ in 0..CASES_CNT {
            let mut v = generate_rand_vec(rand::thread_rng().gen_range(MIN_VEC_SIZE, MAX_VEC_SIZE), rand_gen_fn);
            let time_elapsed = test_sort_function_for_inp(sort_fn, &mut v[..]);
            average_time += time_elapsed / CASES_CNT;
            min_time = min(min_time, time_elapsed);
            max_time = max(max_time, time_elapsed);
        }
        println!("{}  : min: {} milis, max: {} milis, avg: {}", fn_name, min_time.subsec_millis(), max_time.subsec_millis(), average_time.subsec_millis());
    }

    #[test]
    fn int_test() {
        test_sort_function("std Sort  ", &|inp| {inp.sort();}, &|| -> i32 {rand::thread_rng().gen_range(-1000000, 1000000)});
        test_sort_function("Merge Sort", &merge_sort, &|| -> i32 {rand::thread_rng().gen_range(-1000000, 1000000)});
    }
}
