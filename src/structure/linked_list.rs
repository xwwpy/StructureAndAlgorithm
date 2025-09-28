use core::fmt;
use std::fmt::Display;
use std::rc::Rc;
use std::cell::RefCell;

type OptionRefNode<T> = Option<Rc<RefCell<Node<T>>>>;
type RefNode<T> = Rc<RefCell<Node<T>>>;

pub struct Node<T> {
    data: T,
    prev: OptionRefNode<T>,
    next: OptionRefNode<T>,
}

impl<T> Display for Node<T> 
    where T: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Node: [data: {}, prev-ptr: {:p}, next-ptr: {:p}]", self.data, &self.prev, &self.next))?;
        Ok(())
    }
}

pub struct LinkedList<T> {
    head: OptionRefNode<T>,
    tail: OptionRefNode<T>,
    size: usize,
}

impl<T> Display for LinkedList<T>
    where T: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display_size = self.size;
        f.write_str("LinkList {")?;
        let mut node = self.head.clone();
        while display_size > 0 {
            f.write_str(&format!("{} -> ", node.clone().unwrap().borrow()))?;
            node = node.unwrap().borrow_mut().next.clone();
            display_size -= 1;
        }
        f.write_str("}")?;
        Ok(())
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self{
        LinkedList { head: None, tail: None, size: 0 }
    }

    pub fn push(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));
        if self.size == 0 {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node.clone());
        } else {
            let tail_node = self.tail.clone().unwrap();
            tail_node.borrow_mut().next(new_node.clone());
            new_node.borrow_mut().prev(tail_node.clone());
        }
    }


    pub fn pop(&mut self) -> OptionRefNode<T> {
        if self.size == 0 {
            None
        } else {
            let res = self.tail.clone();
            if self.size == 1 {
                self.head = None;
                self.tail = None;
            } else {
                self.tail.clone().unwrap().borrow_mut().prev = None;
                let tail_prev = self.tail.clone().unwrap().borrow_mut().prev.clone();
                self.tail = tail_prev.clone();
                tail_prev.unwrap().borrow_mut().next = None;
            }
            res
        }
        
    }
}


impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self { data, prev: None, next: None }
    }

    pub fn prev(&mut self, ref_node: RefNode<T>) -> &mut Self{
        self.prev.replace(ref_node);
        self
    }

    pub fn next(&mut self, ref_node: RefNode<T>) -> &mut Self{
        self.next.replace(ref_node); // 替换值，并且会返回原来的值，这里直接进行释放
        self
    }
}
