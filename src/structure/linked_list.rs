use std::fmt::{Debug, Display};
use std::rc::Rc;
use std::cell::RefCell;

type OptionRefNode<T> = Option<Rc<RefCell<Node<T>>>>;
type RefNode<T> = Rc<RefCell<Node<T>>>;

pub struct Node<T> {
    data: T,
    prev: OptionRefNode<T>,
    next: OptionRefNode<T>,
}

fn get_option_ref_node_str<T: Debug>(data: &OptionRefNode<T>) -> String{
    if let Some(node) = data {
        format!("{:?}", node.borrow().data)
    } else {
        "None".to_string()
    }
}

impl<T> Display for Node<T> 
    where T: Display + Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Node: [data: {}, prev-data: {:?}, next-data: {:?}]", self.data, get_option_ref_node_str(&self.prev), get_option_ref_node_str(&self.next)))?;
        Ok(())
    }
}

pub struct LinkedList<T> {
    head: OptionRefNode<T>,
    tail: OptionRefNode<T>,
    size: usize,
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

impl<T> Display for LinkedList<T>
    where T: Display + Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display_size = self.size;
        f.write_str("LinkList {")?;
        f.write_str(&format!("head-data: {}, tail_data: {} --- ", get_option_ref_node_str(&self.head), get_option_ref_node_str(&self.tail)))?;
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
            tail_node.borrow_mut().set_next(new_node.clone());
            new_node.borrow_mut().set_prev(tail_node.clone());
            self.tail = Some(new_node.clone());
        }
        self.size += 1;
    }

    #[inline]
    pub fn check_index(&self, index: usize){
        if index >= self.size {
            panic!("Index out of bounds")
        }
    }

    pub fn clone_from_index(&self, index: usize) -> RefNode<T> {

        self.check_index(index);

        let mut temp_size = index;

        let mut res = self.head.clone().unwrap();

        while temp_size != 0 {
            let tmp = res.borrow().next.clone().unwrap();
            res = tmp;
            temp_size -= 1;
        }

        res
    }


    pub fn remove(&mut self, index: usize) -> RefNode<T> {

        self.check_index(index);
        
        if self.size - 1 == index {
            return self.pop().unwrap();// 这里size已经减了1
        }

        let return_node = self.clone_from_index(index);

        let prev_node = return_node.borrow().prev.clone(); // 可能为None
        let next_node = return_node.borrow().next.clone().unwrap(); // 一定不是None
        match prev_node {
            Some(ref_node) => {
                ref_node.borrow_mut().set_next(next_node.clone());
                next_node.borrow_mut().set_prev(ref_node);
            },
            None => {
                // 说明取走的是首节点
                self.head = Some(next_node.clone());
                next_node.borrow_mut().prev = None;
            }
        }
        self.size -= 1;
        return_node
    }

     pub fn remove0(&mut self, index: usize) -> T {

        self.check_index(index);
        
        if self.size - 1 == index {
            return unsafe { self.pop().unwrap().as_ptr().read().data };// 这里size已经减了1 
        }

        let return_node = self.clone_from_index(index);

        let prev_node = return_node.borrow().prev.clone(); // 可能为None
        let next_node = return_node.borrow().next.clone().unwrap(); // 一定不是None
        match prev_node {
            Some(ref_node) => {
                ref_node.borrow_mut().set_next(next_node.clone());
                next_node.borrow_mut().set_prev(ref_node);
            },
            None => {
                // 说明取走的是首节点
                self.head = Some(next_node.clone());
                next_node.borrow_mut().prev = None;
            }
        }
        self.size -= 1;
        return_node.borrow_mut().prev = None;
        return_node.borrow_mut().next = None; // 为什么不加上这两行代码 就会导致内存错误
        unsafe { return_node.as_ptr().read().data }
    }

    pub fn insert(&mut self, data: T, index: usize) {

        if self.size < index {
            panic!("Index out of bounds")
        }

        if self.size == 0 {
            self.push(data);
            return;
        }
        if index == self.size {
            self.push(data);
            return;
        }
        
        let new_node = Rc::new(RefCell::new(Node::new(data)));

        let next_node = self.clone_from_index(index);

        let prev_node = next_node.borrow().prev.clone();

        if let Some(prev_node_temp) = prev_node {
            prev_node_temp.borrow_mut().set_next(new_node.clone());
            new_node.borrow_mut().set_prev(prev_node_temp.clone());
        } else {
            self.head = Some(new_node.clone());
        }

        next_node.borrow_mut().set_prev(new_node.clone());
        new_node.borrow_mut().set_next(next_node.clone());
        self.size += 1;

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
                let tail_prev = self.tail.clone().unwrap().borrow_mut().prev.clone();
                self.tail.clone().unwrap().borrow_mut().prev = None;
                self.tail = tail_prev.clone();
                tail_prev.unwrap().borrow_mut().next = None;
            }
            self.size -= 1;
            res
        }
        
    }
}


impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self { data, prev: None, next: None }
    }

    pub fn set_prev(&mut self, ref_node: RefNode<T>){
        self.prev.replace(ref_node);
    }

    pub fn set_next(&mut self, ref_node: RefNode<T>){
        self.next.replace(ref_node); // 替换值，并且会返回原来的值，这里直接进行释放
    }
}
