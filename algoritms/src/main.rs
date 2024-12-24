mod binsearch;
mod sort;
fn main() {
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

}
