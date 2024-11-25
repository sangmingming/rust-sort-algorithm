
pub fn heap_sort<T>(list: &mut [T])
where
    T: PartialOrd,
{
    build_heap(list);
    let mut h_size: usize = list.len();
    for i in (1..list.len()).rev() {
        h_size -= 1;
        list.swap(i, 0);
        max_heapify(list, 0, h_size);
    }
}

fn build_heap<T>(list: &mut [T])
where
    T: PartialOrd,
{
    let s = list.len();
    for i in (0..(s / 2)).rev() {
        max_heapify(list, i, s);
    }
}

fn max_heapify<T>(list: &mut [T], i: usize, heap_size: usize)
where
    T: PartialOrd,
{
    let mut largest = i;
    let left = i * 2 + 1;
    let right = i * 2 + 2;
    if left < heap_size && list[left] > list[largest] {
        largest = left;
    }
    if right < heap_size && list[right] > list[largest] {
        largest = right;
    }
    if largest != i {
        list.swap(i, largest);
        max_heapify(list, largest, heap_size);
    }
}

pub fn select_sort<T>(list: &mut [T])
where
    T: PartialOrd,
{
    if list.len() < 2 {
        return;
    }
    for i in 0..(list.len() - 1) {
        let mut min_index = i;
        for j in i..list.len() {
            if list[min_index] > list[j] {
                min_index = j;
            }
        }
        if min_index != i {
            list.swap(i, min_index);
        }
    }
}

pub fn bubble_sort<T>(list: &mut [T])
where
    T: PartialOrd,
{
    if list.len() < 2 {
        return;
    }
    for i in 1..list.len() {
        let mut flag = true;
        for j in 0..(list.len() - i) {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
                flag = false;
            }
        }
        if flag {
            break;
        }
    }
}

pub fn insert_sort<T>(list: &mut [T])
where
    T: PartialOrd + Copy,
{
    if list.len() < 2 {
        return;
    }
    for i in 1..list.len() {
        let current = list[i];
        let mut j: i32 = (i - 1) as i32;
        while j >= 0 && list[j as usize] > current {
            list[j as usize + 1] = list[j  as usize];
            j -= 1;
            
        }
        list[(j + 1) as usize] = current;
    }
}

pub fn quick_sort<T>(list: &mut [T])
where
    T: PartialOrd,
{
    quick_sort_v2(list, 0, (list.len() - 1) as i32);
}

fn quick_sort_v2<T>(list: &mut [T], mut left: i32, mut right: i32)
where
    T: PartialOrd,
{
    while left < right {
        let pivot = partition(list, left, right);
        if pivot - left < right - pivot {
            quick_sort_v2(list, left, pivot - 1);
            left = pivot + 1;
        } else {
            quick_sort_v2(list, pivot + 1, right);
            right = pivot - 1;
        }
    }
}

#[allow(unused)]
fn quick_sort_internal(list: &mut [i32], left: usize, right: usize) {
    if left >= right {
        return;
    }

    let pivot: usize = partition(list, left as i32, right as i32) as usize;
    println!("current left {} right {} pivot {}", left, right, pivot);
    quick_sort_internal(list, left, pivot - 1);
    quick_sort_internal(list, pivot + 1, right);
}

fn partition<T>(list: &mut [T], left: i32, right: i32) -> i32
where
    T: PartialOrd,
{
    let m = median_three(list, left, right, (left + right) / 2);
    list.swap(left as usize, m as usize);
    let mut i = left as usize;
    let mut j = right as usize;
    while i < j {
        while i < j && list[j] >= list[left as usize] {
            j -= 1;
        }
        while i < j && list[i] <= list[left as usize] {
            i += 1;
        }
        list.swap(i , j);
    }
    list.swap(i, left as usize);
    return i as i32;
}

fn median_three<T>(list: &mut [T], left: i32, right: i32, mid: i32) -> i32
where
    T: PartialOrd,
{
    let (l, m, r) = (&list[left as usize], &list[mid as usize], &list[right as usize]);
    if (l <= m && m <= r) || (l >= m && m >= r) {
        return mid;
    } else if (m >= l && l >= r) || m <= l && l <= r {
        return left;
    } else {
        return right;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert_sort() {
        let mut list = [1, 3, 2, 9, 5, 19];
        insert_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 5, 9, 19]);

        let mut list = [1, 3, 2, 5, 9, 5];
        insert_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 5, 5, 9]);

        let mut list = [9, 7, 3, 10, 2, 8, 1];
        insert_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 7, 8, 9, 10]);

        let mut list = [2, 1];
        insert_sort(&mut list);
        assert_eq!(list, [1, 2]);
    }

    #[test]
    fn test_bubble_sort() {
        let mut list = [1, 3, 2, 9, 5, 19];
        bubble_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 5, 9, 19]);

        let mut list = [1, 3, 2, 5, 9, 5];
        bubble_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 5, 5, 9]);

        let mut list = [9, 7, 3, 10, 2, 8, 1];
        bubble_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 7, 8, 9, 10]);

        let mut list = [2, 1];
        bubble_sort(&mut list);
        assert_eq!(list, [1, 2]);
    }

    #[test]
    fn test_quick_sort() {
        let mut list = [1, 3, 2, 9, 5, 19];
        quick_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 5, 9, 19]);

        let mut list = [1, 3, 2, 5, 9, 5];
        quick_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 5, 5, 9]);

        let mut list = [9, 7, 3, 10, 2, 8, 1];
        quick_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 7, 8, 9, 10]);

        let mut list = [2, 1];
        quick_sort(&mut list);
        assert_eq!(list, [1, 2]);
    }

    #[test]
    fn test_select_sort() {
        let mut list = [1, 3, 2, 9, 5, 19];
        select_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 5, 9, 19]);

        let mut list = [1, 3, 2, 5, 9, 5];
        select_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 5, 5, 9]);

        let mut list = [9, 7, 3, 10, 2, 8, 1];
        select_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 7, 8, 9, 10]);

        let mut list = [2, 1];
        select_sort(&mut list);
        assert_eq!(list, [1, 2]);
    }

    #[test]
    fn test_heap_sort() {
        let mut list = [1, 3, 2, 9, 5, 19];
        heap_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 5, 9, 19]);

        let mut list = [1, 3, 2, 5, 9, 5];
        heap_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 5, 5, 9]);

        let mut list = [9, 7, 3, 10, 2, 8, 1];
        heap_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 7, 8, 9, 10]);

        let mut list = [2, 1];
        heap_sort(&mut list);
        assert_eq!(list, [1, 2]);
    }
}
