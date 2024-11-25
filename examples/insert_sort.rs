use simple_sort::insert_sort;

fn main() {
    let mut list = [9, 7, 3, 10, 2, 8, 1];
    insert_sort(&mut list);
    println!("heap sort: after sort: {:?}", list);
}
