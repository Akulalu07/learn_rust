pub fn quick_sort<T> (nums: &mut Vec<T>) {
    if nums.len() <=2 {
        return;
    }
    let mid_ind = (nums.len()-1)>>1;
    let mid = &nums[mid_ind];
    let lesser : Vec<T> = Vec::new();
    let bigger : Vec<T> = Vec::new();

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