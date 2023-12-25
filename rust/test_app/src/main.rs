use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("key".to_string(), 10);
    map.insert("key2".to_string(), 20);
    for (key, value) in map.iter() {
        println!("{key}: {value}");
    }
}
