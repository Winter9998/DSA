use std::collections::LinkedList;

pub fn new_linked_list() {
    // Adds a new Linked List with 1, 5, 4
    let mut new_list = LinkedList::from([1, 5, 4]);
    new_list.push_back(9); // adds to the end 9 
    new_list.pop_front(); // remove front which is first element of the linked list

    println!("{:?}", new_list);

}