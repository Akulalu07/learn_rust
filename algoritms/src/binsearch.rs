use std::vec::Vec;
pub fn bin_search<T: std::cmp::PartialEq + std::cmp::PartialOrd>(numbers: Vec<T>, search_number: T) -> Option<u64>{
    let mut left: usize = 0;
    let mut right: usize = (numbers.len() - 1) as usize;
    while left < right{
        let mid: usize = (left>>1)+(right>>1);
        if numbers[mid] == search_number{
            return Some(mid as u64);
        }
        if numbers[mid] > search_number{
            left = mid + 1;
        }
        if numbers[mid] < search_number{
            right = mid - 1;
        }
    }
    return None;
}