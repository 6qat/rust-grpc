use std::mem;
use std::time::{Duration, Instant};

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    let mut count = 0;
    let time_limit = Duration::new(000_001, 111_222_222);
    let start = Instant::now();
    while (Instant::now() - start) < time_limit {
        count += 1;
    }
    println!("{}", count);

    #[derive(Debug)]
    enum List<T> {
        Cons(T, Box<List<T>>),
        Nil,
    }

    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{:?}", list);

    use std::net::IpAddr;
    let _home: IpAddr = "127.0.0.1".parse().unwrap();

    println!("Var {} has {} bytes", _home, mem::size_of_val(&_home));
}
