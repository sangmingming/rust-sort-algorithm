use simple_sort::quick_sort;

fn main() {
    let mut list = [9, 7, 3, 10, 2, 8, 1];
    quick_sort(&mut list);
    println!("after sort: {:?}", list);
}
