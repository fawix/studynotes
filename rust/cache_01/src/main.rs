extern crate bmemcached;

use bmemcached::MemcachedClient;

fn main() {
    println!("Hello, world!");
    
    let client = MemcachedClient::new(vec!["127.0.0.1:1121"], 5).unwrap();

    client.set("key", "value", 1000);
    let rv: String = client.get("key").unwrap();
    println!("rv {}", rv);
    assert_eq!(rv, "value");
}


