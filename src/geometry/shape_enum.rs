
pub enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
    Triangle(Triangle),
}

pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[allow(dead_code)]
impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    fn translate(&mut self, t_x: f64, t_y: f64) {
        self.x += t_x;
        self.y += t_y;
    }
}

pub struct Rectangle {
    origin: Point,
    width: f64,
    height: f64,
}

#[allow(dead_code)]
impl Rectangle {
    pub fn new(origin: Point, width: f64, height: f64) -> Self {
        Self { origin, width, height }
    }

    pub fn origin(&self) -> &Point { &self.origin }
    pub fn width(&self) -> f64 { self.width }
    pub fn height(&self) -> f64 { self.height }
}

pub struct Circle {
    origin: Point,
    radius: f64,
}

#[allow(dead_code)]
impl Circle {
    pub fn new(origin: Point, radius: f64) -> Self {
        Self { origin, radius }
    }
    pub fn origin(&self) -> &Point { &self.origin }
    pub fn radius(&self) -> f64 { self.radius }
}

pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

#[allow(dead_code)]
impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        Self { a, b, c }
    }
    pub fn a(&self) -> &Point { &self.a }
    pub fn b(&self) -> &Point { &self.b }
    pub fn c(&self) -> &Point { &self.c }
}

/// Returns the area of the shape
pub fn shape_area(shape: &Shape) -> f64 {
    match shape {
        Shape::Rectangle(rect) => rect.width * rect.height,
        Shape::Circle(circle) => std::f64::consts::PI * circle.radius * circle.radius,
        Shape::Triangle(triangle) => {
            (triangle.a.x * (triangle.b.y - triangle.c.y)
                + triangle.b.x * (triangle.c.y - triangle.a.y)
                + triangle.c.x * (triangle.a.y - triangle.b.y)).abs() / 2.0
        }
    }
}

/// Returns the type name of the shape
pub fn shape_type_name(shape: &Shape) -> &'static str {
    match shape {
        Shape::Rectangle(_) => { "rectangle" }
        Shape::Circle(_) => { "circle" }
        Shape::Triangle(_) => { "triangle" }
    }
}

/// Moves every point of the shape by a certain distance
/// * t_x: Horizontal shift
/// * t_y: Vertical shift
fn translate(shape: &mut Shape, t_x: f64, t_y: f64) {
    match shape {
        Shape::Rectangle(rect) => { rect.origin.translate(t_x, t_y); }
        Shape::Circle(circle) => { circle.origin.translate(t_x, t_y) }
        Shape::Triangle(triangle) => {
            triangle.a.translate(t_x, t_y);
            triangle.b.translate(t_x, t_y);
            triangle.c.translate(t_x, t_y);
        }
    }
}