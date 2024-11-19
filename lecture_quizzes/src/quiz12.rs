/*
/* Question 5 */
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn distance_from_origin(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    fn is_on_axis(&self) -> bool {
        self.x == 0 || self.y == 0
    }
}

/* Question 6 */
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    // Polygon(Vec<(f64, f64)>), // cannot just write this and call it a day b/c fn describe does not have an exhaustible match case for this
}

fn describe(s: Shape) {
    match s {
        Shape::Circle(r) => 
              println!("Circle with radius {}", r),
        Shape::Rectangle(w, h) => 
              println!("Rectangle with width {} and height {}", w, h),
    }
}

/* Question 7 */
enum Status {
    Active,
    Inactive,
    Suspended,
}
*/

/*
/* Question 8 */
struct Circle {
    radius: f64,
}

impl Circle {
    fn double_radius(&mut self) {
        self.radius *= 2.0;
    }

    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
}

/* Question 9 */
#[derive(Debug)]
enum Status {
    Online(String),
    Offline,
}
*/

/* Question 11 */
struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

trait Drawable {

    fn draw(&self) {
        println!("Drawing some Shape");
    }
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a Circle with radius: {}", self.radius);
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing a Rectangle with width {} and height {}",
                                            self.width, self.height);
    }
}

/* question 11.4 */
// A works!
fn process<T: Drawable + Clone>(item: T) {
    item.draw();
}

//B does not compile
/*fn process<T>(item: T)
where
    T: Drawable + Clone,
{
    item.draw();
}*/

// C does not compile
/*fn process<T:Clone>(item: T) {
    item.draw();
 }*/

fn main() {
    /*
    /* Question 5 */
    let p = Point { x: 10, y: 20 };
    println!("Point: ({}, {})", p.x, p.y);

    let p = Point::new(3, 4);
    println!("{}", p.distance_from_origin());

    let mut p = Point::new(1, 1);
    p.translate(2, 3);
    println!("{}", p.distance_from_origin());

    /* Question 6 */
    let s = Shape::Circle(10.0);
    describe(s);
    // describe(s); // compilation error and can't do it twice b/c s moves to describe fn's closure and does not return anything (returns unit)

    let shape1 = Shape::Circle(5.0);
    let shape2 = Shape::Rectangle(3.0, 4.0);

    describe(shape1);
    describe(shape2);

    let s = Shape::Circle(10.0);
    // describe(&s); // compilation error b/c describe's argument expected 'Shape', not '&Shape'
    // describe(&s);
    */

    /* Question 7 */
    /*
    let s = Status::Active;
    match s {
        Status::Inactive => println!("Inactive"),
        Status::Active => println!("Active"),
        //Status::Suspended => todo!(), // you need this wildcard pattern to make match exhaustive!
    }*/

    /*
    /* Question 8 */
    let mut c = Circle { radius: 5.0 };
    c.double_radius();
    println!("Area: {:.2}", c.area());

    /* Question 9 */
    let user = Status::Online(String::from("Alice"));
    if let Status::Online(name) = user {
        println!("User {} is online.", name);
    } else {
        println!("User is offline.");
    }*/

    let c = Circle { radius: 3.0 };
    let r = Rectangle { width: 4.0, height: 5.0 };

    c.draw();
    r.draw();
}