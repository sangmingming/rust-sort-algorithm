use simple_sort::heap_sort;

fn main() {
    let mut list = [1, 3, 2, 9, 5, 19];
    heap_sort(&mut list);
    println!("heap sort: after sort: {:?}", list);
}
