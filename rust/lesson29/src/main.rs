fn main() {
    println!("Hello, Iterators!");

    /*
        Yay! Iterators is a fun topic :)

        Interators are designed to allow us 
        to process some task in a sequence of items
        in turns (sort of ... one at a time).

        The interator abstracts the logic of 'iterating' 
        over each item. In Rust iterators are __lazy__
        (meaning they don't do anything until we use it).
    */

    //code that does nothing...
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();//...besides creating the interator ...
    
    //...well, not until we use it that is:
    for val in v1_iter {
        println!("Got: {}", val);
    }

    /*
        iterator implementation uses associated types ... 
        Looks something like this:

        trait Iterator {
            type Item;
            fn next(&mut self) -> Option<Self::Item>;
            ...
        }

        the Item type is the associated type that is returned in the 
        next Option value. (check tests below)
    */
}

#[cfg(test)]
mod test {
    #[test]
    fn iterator_one_by_one() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();
        //Note how we made this mutable refrence
        //The for loop does this behind the scenes

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
        assert_eq!(v1_iter.next(), None);
    }

    //Iterator have a bunch of additional methods
    //for example sum():
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);

    }
    /*
        The method sum calls the next method, these types (that call next) consume
        the interator (as seen above); they are called consuming adaptors.

        There are also methods that produce other iterator, those are called
        iterator adaptors.
    */
    #[test]
    fn iterator_map() {
        let v1: Vec<i32> = vec![1, 2, 3];
        v1.iter().map(|x| x+1); //this doesn't do anything
        //because the iterator is lazy (check warning)
    }

    #[test]
    fn iterator_map_collect() {
        let v1: Vec<i32> = vec![1, 2, 3];
        let v2: Vec<_>   = v1.iter().map(|x| x+1).collect();
        //but when we call collect the map is 
        //actually applied to our vector
        assert_eq!(v2, vec![2, 3, 4]);
    }

}