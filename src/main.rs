trait Shape {
    fn area(&self) -> i32;
}

struct Rect {
    width: i32,
    height: i32,
}

struct Circle {
    r: i32,
}

impl Shape for Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

impl Shape for Circle {
    fn area(&self) -> i32 {
        self.r * self.r * 3
    }
}

fn print_area<T: Shape>(x: &T) {
    println!("{}", x.area());
}

fn print_area_dyn(x: &dyn Shape) {
    println!("{}", x.area());
}

fn main(){
    let r = Rect { width: 3, height: 4 };
    let c = Circle { r: 5 };

    print_area(&r);
    print_area(&c);
}
