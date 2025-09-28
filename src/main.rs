use demo01::{algorithm, structure::linked_list::LinkedList};

fn main() {
    let mut ll = LinkedList::new();
    ll.push(1);
    ll.push(2);
    ll.push(3);
    println!("{}", ll);
    println!("{}", ll.pop().unwrap().borrow());
    println!("{}", ll);
}

