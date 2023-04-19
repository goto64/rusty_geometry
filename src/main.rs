use crate::geometry::shape_enum;
use crate::geometry::shape_trait;

mod geometry;

fn main() {

    /* ********** Enum Variant: ********** */

    // Create the shapes with shape_enum
    let circle = shape_enum::Shape::Circle(shape_enum::Circle::new(shape_enum::Point::new(0.0, 0.0),4.0));
    let rectangle = shape_enum::Shape::Rectangle(shape_enum::Rectangle::new(shape_enum::Point::new(0.0, 0.0), 5.0, 3.0));
    let triangle = shape_enum::Shape::Triangle(shape_enum::Triangle::new(
        shape_enum::Point { x: 0.0, y: 7.0 },
        shape_enum::Point { x: 5.0, y: 7.0 },
        shape_enum::Point { x: 5.0, y: 0.0 },
    ));

    // Put all the shapes in a list
    let shape_list: Vec<shape_enum::Shape> = vec![circle, rectangle, triangle];

    // Iterate through all shapes and print out area
    println!("Enum variant:");
    for shape in shape_list {
       println!("  Area of {}: {}", shape_enum::shape_type_name(&shape), shape_enum::shape_area(&shape));
    }


    /* ********** Trait Variant: ********** */

    // Create the shapes with shape_trait
    let circle = shape_trait::Circle::new(shape_trait::Point::new(0.0, 0.0), 4.0);
    let rectangle = shape_trait::Rectangle::new(shape_trait::Point::new(0.0, 0.0), 5.0, 3.0);
    let triangle = shape_trait::Triangle::new(
        shape_trait::Point::new(0.0, 7.0),
        shape_trait::Point::new(5.0, 7.0),
        shape_trait::Point::new(5.0, 0.0));

    let shape_trait_list: Vec<Box<dyn shape_trait::IsShape>> = vec![Box::new(circle), Box::new(rectangle), Box::new(triangle)];

    // Iterate through all shapes and print out area
    println!("Trait variant:");
    for shape in shape_trait_list {
        println!("  Area of {}: {}", shape.type_name(), shape.area());
    }
}
