fn main() {
    println!("Hello, Interators + Closures!");
    //in a fun way
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: i32,
    style: String
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: i32) -> Vec<Shoe> {
    shoes.into_iter()
         .filter(|s| s.size == shoe_size) //closure that capture the env
         .collect()
} // SOCOOL!!!
/*
    much like scala we can filter and collect all in one shot!
    Super cool :) 
*/

#[test]
fn filters_by_size () {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );

    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];//this thing about ownership is super misleading.
    //the vec had alerady been moved so I needed to create another one... 
    //phew (rather do that then change the fn for now LOL ... just for studying
    //if real code should do and adhere to DRY as much as possible).

    let in_my_size = shoes_in_my_size(shoes, 13);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 13, style: String::from("sandal") },
        ]
    );
}

/*
    When implementing an interator all we need to do 
    is implement the next function, since all the other
    functions use next and the default implementation will
    work. 
    see below implementation.
*/
pub struct Counter {
    count: u32
}//will count stuff ...

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    //let's say our counter can only 
    //count to 5. Here is a next method for it:
    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= 5 {
            None
        } else {
            self.count += 1;
            Some(self.count)
        }
    }
}


#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
    assert_eq!(counter.next(), None);
    assert_eq!(counter.next(), None);
    assert_eq!(counter.next(), None);
    assert_eq!(counter.next(), None);
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a, b)| a * b)
                                 .filter(|x| x % 3 == 0)
                                 .sum();
    assert_eq!(18, sum);
}//Fun note: theoretical pair of zip (5, None) is never produced in Rust implementation
//because zip returns when either of the input interators return None
//this is bound to bite back when using this for real Math... 

