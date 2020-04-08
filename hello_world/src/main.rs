//! this is my module documenation. My library is so nice!
//! four() is a function that returns `4`
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn four() -> i32 { 4 }

#[cfg(test)]
mod tests {
    use super::four;
    #[test]
    fn it_works() {
        assert_eq!(four(), 4);
    }
}

fn main() {
    println!("Hello, world!");
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5;
    let x = x + 1;
    let x = x + 2;
    println!("The value of x is: {}", x);

    let _x = 2.0;
    let _y: f32 = 3.0;

    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _remainder = 43 % 5;

    let _t = true;
    let _f: bool = false;
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;
    println!("The value  of y is {}", y);
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = x.0;

    let _six_point_four = x.1;

    let _one = x.2;

    let a = [1, 2, 3, 4, 5];
    let _first = a[0];
    let _second = a[1];
    another_function();
    another_function_2(5, 6);
    another_function_3();
    another_function_4();
    another_function_5();
    another_function_6();
    another_function_7();

    let _width1 = 30;
    let _height1 = 50;
    println!("The area of the rectangle is {} square pixels", area(_width1, _height1));

    let _rect1 = (30, 50);
    println!("The area of the rectangle is{} square pixels.", area_tuple(_rect1));

    let _rect1 = Rectangle { width: 30, height: 50};
    println!("The area of the rectangle is {} square pixels.", area_borrow(&_rect1));
    
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_borrow(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn another_function() {
    println!("Another function");
}

fn another_function_2(_x: i32, _y: i32) {
    println!("Another function 2");
    println!("The value of x is: {}", _x);
    println!("The value of y is: {}", _y);
}

fn another_function_3() {
    let mut _number = 3;
    while _number != 0 {
        println!("{}!", _number);
        _number -= 1;
    }
    println!("LIFTOFF!");
}

fn another_function_4() {
    let _x = 5;
    let _y = _x;
    let _s1 = String::from("hello world!");
    let _s2 = _s1;
}

fn another_function_5() {
    let _s1 = String::from("hello");
    let _s2 = _s1.clone();
    println!("s1 = {}, s2 = {}", _s1, _s2);
}

fn another_function_6() {
    let _s1 = String::from("hello");
    take_ownership(_s1);
    let _x = 5;
    makes_copy(_x);

}

fn take_ownership(_some_string: String) {
    println!("{}", _some_string);
}
fn makes_copy(_some_integer: i32) {
    println!("{}", _some_integer);
}


/*struct User {
    email: &str,
    username: &str,
    active: bool,
    sign_in_count: u64,
}*/
fn another_function_7() {

   /* let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,

    };

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count
    }; */

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}