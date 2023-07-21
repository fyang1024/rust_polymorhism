mod shape_enum;
mod shape_trait;

fn main() {
    let rectangle = shape_enum::Shape::Rectangle(shape_enum::Rectangle::new(6., 5.));
    let circle = shape_enum::Shape::Circle(shape_enum::Circle::new(3.));
    let triangle = shape_enum::Shape::Triangle(shape_enum::Triangle::new(3., 4., 5.));
    let shapes = vec![rectangle, circle, triangle];
    for shape in shapes {
        match shape {
            shape_enum::Shape::Rectangle(rect) => {
                println!("The area of the rectangle is {}", rect.area());
            }
            shape_enum::Shape::Circle(circle) => {
                println!("The perimeter of the circle is {}", circle.perimeter());
            }
            shape_enum::Shape::Triangle(triangle) => {
                if triangle.is_isosceles() {
                    println!("The triangle is isosceles");
                } else {
                    println!("The triangle is not isosceles");
                }
            }
        }
    }

    let rectangle = shape_trait::Rectangle::new(6., 5.);
    let circle = shape_trait::Circle::new(3.);
    let triangle = shape_trait::Triangle::new(3., 4., 5.);
    let shapes: Vec<&dyn shape_trait::Shape> = vec![&rectangle, &circle, &triangle];
    for shape in shapes {
        println!("The perimeter is {}", shape.perimeter());
    }
}
