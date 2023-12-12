use crate::linked_list::new_linked_list;

mod array_hex;
mod binary_search;
mod linked_list;

fn main() {
    println!("\n A random generated array with raw pointers :");
    array_hex::make_array_hex();

    let mut arr = [10,8,1,4,15,59,24,5,67,2,91,481,1,3,7];
    arr.sort();
    println!("{:?}", arr);
    let target = 7;
    println!("\n A binary search :");
    match binary_search::binary_search(&arr, target){
        Some(index) => println!("Found {} at index {}", target, index),
        None => println!("{} not found in array", target),
    }

    println!("\n A linked List :");
    new_linked_list();

}