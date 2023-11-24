// fn main() {
//     let number_list: Vec<i32> = vec![34,50,250,100,65];
//     let largest = largest(&number_list);
//
//     println!("the largest number is: {}", largest);
// }

struct Point<T> {
    x: T,
    y: T
}

// struct Point2<T, U> {
//     x: T,
//     y: U
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        return (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point3<X1, Y1> {
    x: X1,
    y: Y1
}

impl<X1, Y1> Point3 <X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y
        }
    }
}
fn main () {
    // let integer = Point2 {x: 5, y: 1.0};
    // let float = Point {x: 5.1, y: 1.0};

    // let p = Point {x: 5, y: 10};
    // println!("p.x = {}", p.x());

    // let p2 = Point {x: 5.0, y: 10.0};
    // println!("p2.x = {}", p2.distance_from_origin());
    //
    // let p_a = Point3 { x: 5, y: 10.4};
    // let p_b = Point3 { x: "Hello", y: 'y'};
    //
    // let p_c = p_a.mixup(p_b);
    //
    // println!("p_c.x = {}, p_c.y = {}", p_c.x, p_c.y);

    let list: Vec<i32> = vec![34, 23, 53, 23];
    let largest = largest(&list);

    println!("{}", largest);
}

fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest: T = list[0];

    for &number in list {
        if number > largest {
            largest = number; // replace the number to the higher number
        }
    }

    return largest;
}

// fn largest(list: &[i32]) -> i32 {
//     let mut largest: i32 = list[0];
//
//     for &number in list {
//         if number > largest {
//             largest = number; // replace the number to the higher number
//         }
//     }
//
//     return largest;
// }