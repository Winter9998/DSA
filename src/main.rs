mod array_hex;
mod binary_search;

fn main() {
    array_hex::make_array_hex();

    let mut arr = [10,8,1,4,15,59,24,5,67,2,91,481,1,3,7];
    arr.sort();
    println!("{:?}", arr);
    let target = 7;
    match binary_search::binary_search(&arr, target){
        Some(index) => println!("Found {} at index {}", target, index),
        None => println!("{} not found in array", target),
    }

}