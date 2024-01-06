use std::{collections::HashMap, thread, time::Duration};

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

struct Cacher<T: Fn(u32) -> u32> {
    calculation: T,
    cache_hashmap: HashMap<u32, u32>,
}
impl<T: Fn(u32) -> u32> Cacher<T> {
    fn new(calculation: T) -> Self {
        Self {
            calculation,
            cache_hashmap: HashMap::new(),
        }
    }
    fn call(&mut self, arg: u32) -> u32 {
        match self.cache_hashmap.get(&arg) {
            Some(&v) => v,
            None => {
                let value = (self.calculation)(arg);
                self.cache_hashmap.insert(arg, value);
                value
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut simulated_expensive_calculation = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation.call(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation.call(intensity)
        )
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!(
            "Today, run for {} minutes!",
            simulated_expensive_calculation.call(intensity)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn call_func() {
        let mut c = Cacher::new(|a| a);
        let _ = c.call(1);
        assert_eq!(c.call(2), 2);
    }
}
