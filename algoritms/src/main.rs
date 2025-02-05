mod binsearch;
mod sort;
mod tree;
use time;

fn main() {
    let now_utc = time::OffsetDateTime::now_utc().time();
    let numbers = vec![1, 2, 3, 4, 5];
    let search_number = 3;
    println!("Search number: {} in {:.?}", search_number,numbers );
    let result = binsearch::bin_search(numbers, search_number);
    match result {
        Some(index) => println!("Found at index: {}", index),
        None => println!("Not found"),
    }
    let mut numbers = vec![4,3,1,5,2];
    sort::quick_sort(&mut numbers);
    println!("{:?}",numbers);
    println!("2_i 32.pow(16) = {:.?}",2_i32.pow(16));
    print!("{}" , time::OffsetDateTime::now_utc().time() - now_utc);
}
