use demo01::{algorithm, structure::vector::Vector};

fn main() {
    unsafe {
        std::env::set_var("RUST_LOG", "Debug");   
    }
    tracing_subscriber::fmt::init();
    println!("{}", algorithm::binary_search::index_of_left_right_most_advanced00(&vec![1, 2, 3, 3, 4, 4, 5], 1, 3, 0, algorithm::binary_search::SearchMod::RightMost));
    let mut v = Vector::new();
    println!("{}", v);
    v.push(1);
    v.push(2);
    v.insert(2, 3);
    v.insert(1, 4);
    
    println!("{}", v);
    println!("{}", v.pop().unwrap());
    println!("{}", v);
    for i in v {
        println!("{:?}", i);
        return;
    }
}

