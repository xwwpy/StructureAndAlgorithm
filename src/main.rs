use std::rc::Rc;

use demo01::{algorithm, structure::linked_list::LinkedList};

fn main() {
    let mut ll = LinkedList::new();
    ll.push(1);
    ll.push(2);
    ll.push(3);
    println!("{}", ll);
    ll.insert(0, 0);
    println!("{}", ll);
    ll.pop();
    ll.pop();
    ll.insert(2, 2);
    ll.insert(9, 1);
    println!("{}", ll);
    println!("{}", ll.remove0(0));
    println!("{}", ll);
    println!("{}", ll.remove0(1));
    println!("{}", ll);
}

