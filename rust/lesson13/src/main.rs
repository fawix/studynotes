#[derive(Debug)] //this allows us to use the {:?} to print 
//good only for debugging ... although I prefer a debugger (stepping through)
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    println!("Calculate the area of a Rectangle");
    //using sctructs -- thought evolution below

    //not using any special types
    let length = 50;
    let width = 30;
    println!("The area_1 is {} pixels squared", area_1(length, width));

    //using tuples
    let rectangle1 = (50, 30);
    println!("The area_2 is {} pixels squared", area_2(rectangle1));

    let rectangle2 = Rectangle { length: 50, width: 30 };
    println!("The area_3 is {} pixels squared", area_3(&rectangle2));

    //These are using traits:
    println!("rectangle2 = {:?}", rectangle2);
    println!("rectangle2 = {:#?}", rectangle2);
}



fn area_1 (length: u32, width: u32) -> u32 {
    length * width
}

fn area_2 (rectangle: (u32, u32)) -> u32 {
    rectangle.0 * rectangle.1 //eww
}

fn area_3(rectangle: &Rectangle) -> u32 { //borrowing (without ownership)
    rectangle.length * rectangle.width
} // this makes me realize we really have to think about 
// borrowing and owning, because if I had taken ownership here for a simple 
// calculation we would have lost ownership on main program and wouldn't be able to use it 
// unless we returned it. This makes Rust very unique, but rather powerful too.
