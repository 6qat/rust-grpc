use std::time::{Duration, Instant};

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
}
