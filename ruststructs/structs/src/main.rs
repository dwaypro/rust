

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,

}

impl Rectangle {

    fn square(size: u32) -> Rectangle{
        Rectangle { width: size, height: size}
    }

    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}


fn main() {
    println!("I love rust!");

    let x = 2.0;
    let y: f32 = 3.3;
    println!("this is the value of x {} , {}", x, y);

    // to allocate space on the heap instead of at compile time:
    let s = String::from("hello");

    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    
    println!("the value of len = {}", len);

    let width = 30;
    let height = 50;
    let rect1 = Rectangle {width: 30, height: 50};
    let sq1 = Rectangle::square(3);

    println!("rect 1 is {:#?}", rect1);
    println!("area ==> {}", area(&rect1));

    println!(" the area of the reactangle is {:#?} square pixels.", rect1.area());
    println!("We have a square, {:#?}", sq1);
}

fn calculate_length(s: &String) -> usize{
    s.len()
}

fn area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}

