mod binsearch;
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let search_number = 3;
    println!("Search number: {} in {:.?}", search_number,numbers );
    let result = binsearch::bin_search(numbers, search_number);
    match result {
        Some(index) => println!("Found at index: {}", index),
        None => println!("Not found"),
    }


}