struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let vector:Vec<i32> = vec![1, 2, 3];

    let vector_iter = vector.iter();

    for val in vector_iter {
        println!("value form the vector: {}", val);
    }

    // Next method in iterator.
    println!("Next method in iterator--------------");
    let vector:Vec<i32> = vec![5, 6, 7];

    let mut vector_iter = vector.iter();
    println!("{:?}",vector_iter.next());
    println!("{:?}",vector_iter.next());
    println!("{:?}",vector_iter.next());
    println!("{:?}",vector_iter.next());

    // The next method for using counter iterator.
    println!("Counter method in iterator--------------");
    let mut counter = Counter::new();

    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
}
