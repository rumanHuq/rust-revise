// operator overloading
use std::ops::Add;
#[derive(Debug)]
struct Point {
    a: i32,
    b:i32
}
#[derive(Debug)]
struct Another {
    a: i64,
    b: i64,
}

impl Add<Another> for Point {
    type Output = Point;
    fn add(self, other: Another)->Point{
        Point {
            a: self.a + other.a as i32,
            b: self.b + other.b as i32,
        }
    }
}

fn main(){
    let first = Point { a: 1, b: 2 };
    let second = Another { a: 2, b: 4 };
    
    let third = first.add(second);
    
    println!("{:?}", third);
}