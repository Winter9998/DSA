use std::collections::LinkedList;

pub fn new_linked_list() {
    let mut new_list = LinkedList::from([1, 5, 4]);
    new_list.push_back(9);
    new_list.pop_front();

    println!("{:?}", new_list);

}