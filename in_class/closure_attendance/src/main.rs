use std::{thread, time::Duration};

struct ComputeCache<T>
where
    T: Fn() -> String,
{
    function: T,
    data: Option<String>
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    fn new(computation: T) -> Self {
        return ComputeCache {
            function: computation, data: None
        }
    }

    fn get_result(&mut self) -> String {
        match &self.data {
            Some(d) => {
                println!("Retrieved from cache instantly!");
                return d.to_string();
            }
            None => {
                let d = (self.function)();
                self.data = Some(d.clone());
                return d.to_string();
            }
        }
    }
}

fn main() {
    let mut cache = ComputeCache::new(|| {
        println!("Computing (this will take 2 seconds)...");
        thread::sleep(Duration::from_secs(2));
        "Hello, world!".to_string()
    });

    println!("First call:");
    println!("Result: {}", cache.get_result());
    
    println!("\nSecond call:");
    println!("Result (cached): {}", cache.get_result());
}