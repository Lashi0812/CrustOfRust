pub struct Counter {
    count: usize,
}

impl Counter {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod simple_example_tests {
    use crate::simple_example::*;

    #[test]
    fn count5() {
        let counter: Counter = Counter::new();
        assert!(counter.eq(vec![1, 2, 3, 4, 5].into_iter()));
    }
}
