pub fn quick_sort<T: Ord + Clone>(nums: &mut Vec<T>) {
    if nums.len() < 2 {
        return;
    }

    let mid_index = nums.len() >>1 ;
    let pivot = nums[mid_index].clone();

    let mut lesser = Vec::new();
    let mut greater = Vec::new();

    for (i, &ref value) in nums.iter().enumerate() {
        if i != mid_index {
            if value <= &pivot {
                lesser.push(value.clone());
            } else {
                greater.push(value.clone());
            }
        }
    }

    quick_sort(&mut lesser);
    quick_sort(&mut greater);

    nums.clear();
    nums.extend(lesser);
    nums.push(pivot);
    nums.extend(greater);
}


/*
def quicksort ( array ) :
    if len ( array) < 2 :
        return array
    else :
        pivot = array[0]
        less = [ i for i in array [1 :] if i<=pivot]
        greater = [ i for i in array [1 :] if i>pivot]
        return quicksort( less ) + [ pivot ] + quicksort(greater )

 */