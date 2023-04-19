use std::any::{Any, TypeId};

pub trait IsShape {
    /// Returns the area of the shape
    fn area(&self) -> f64;

    /// Moves every point of the shape by a certain distance
    /// * t_x: Horizontal shift
    /// * t_y: Vertical shift
    fn translate(&mut self, t_x: f64, t_y: f64);

    /// Returns the type name of the shape
    fn type_name(&self) -> &'static str;
}

pub struct Point {
    x: f64,
    y: f64,
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

impl IsShape for Rectangle {
    fn area(&self) -> f64 {
        return self.width * self.height
    }
    fn translate(&mut self, t_x: f64, t_y: f64) {
        self.origin.translate(t_x, t_y);
    }

    fn type_name(&self) -> &'static str {
        return "Rectangle"
    }
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

impl IsShape for Circle {
    fn area(&self) -> f64 {
        return std::f64::consts::PI * self.radius * self.radius;
    }
    fn translate(&mut self, t_x: f64, t_y: f64) {
        self.origin.translate(t_x, t_y);
    }

    fn type_name(&self) -> &'static str {
        return "Circle"
    }
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

impl IsShape for Triangle {
    fn area(&self) -> f64 {
        return (self.a.x * (self.b.y - self.c.y)
            + self.b.x * (self.c.y - self.a.y)
            + self.c.x * (self.a.y - self.b.y)).abs() / 2.0;
    }
    fn translate(&mut self, t_x: f64, t_y: f64) {
        self.a.translate(t_x, t_y);
        self.b.translate(t_x, t_y);
        self.c.translate(t_x, t_y);
    }

    fn type_name(&self) -> &'static str {
        return "Triangle"
    }
}

pub fn is_rectangle(shape: Box<dyn IsShape>) -> bool {  // Like instanceof
    return shape.type_id() == TypeId::of::<Rectangle>();
}