pub fn binary_search(array: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = array.len();
    
    while low < high {
        let mid = low + (high - low) / 2;
        if array[mid] == target {
            return mid.try_into().unwrap();
        } else if array[mid] < target {
            low = mid + 1;
        } else {
            high = mid;
        }
        
    }
    None

}