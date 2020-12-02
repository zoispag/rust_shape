struct Shape {
    width: u32,
    height: u32,
}

impl Shape {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        (self.width + self.height) * 2
    }
}

#[cfg(test)]
mod shape_tests {
    use super::*;
    #[test]
    fn test_area_square() {
        let square = Shape {
            width: 3,
            height: 3,
        };
        assert_eq!(square.area(), 9);
    }
    #[test]
    fn test_area_rect() {
        let rectangle = Shape {
            width: 3,
            height: 5,
        };
        assert_eq!(rectangle.area(), 15);
    }
    #[test]
    fn test_perimeter_square() {
        let square = Shape {
            width: 3,
            height: 3,
        };
        assert_eq!(square.perimeter(), 12);
    }
    #[test]
    fn test_perimeter_rect() {
        let rectangle = Shape {
            width: 3,
            height: 5,
        };
        assert_eq!(rectangle.perimeter(), 16);
    }
}

fn volume(shape: &Shape, depth: u32) -> u32 {
    shape.area() * depth
}

fn main() {
    let square = Shape {
        width: 3,
        height: 3,
    };
    println!(
        "A square with side {} has {}m perimeter and {}m2 area",
        square.width,
        square.perimeter(),
        square.area()
    );

    let rectangle = Shape {
        width: 3,
        height: 5,
    };
    println!(
        "A rectangle with width {} and height {} has {}m perimeter and {}m2 area",
        rectangle.width,
        rectangle.height,
        rectangle.perimeter(),
        rectangle.area()
    );

    println!(
        "Volume for square with depth {} is {}m3",
        square.height,
        volume(&square, square.height)
    )
}
