/// This module demonstrates operator overloading in Rust using the `Add`, `Sub`, `Mul`, and `Div` traits.

use std::ops::{Add, Div, Mul, Sub};

pub fn start() {
    println!(
        "============================= Example 1: Overloading the `+` operator for a custom type ============================="
    );
    #[derive(Debug)]
    struct Person {
        first_name: String,
        last_name: String,
    }
    #[derive(Debug)]
    struct Marriage {
        husband: Person,
        wife: Person,
        location: String,
        date: chrono::NaiveDate,
    }
    // Implementing the Add trait for Person to create a Marriage
    impl Add for Person {
        type Output = Marriage;

        fn add(self, other: Person) -> Self::Output {
            Marriage {
                husband: self,
                wife: other,
                location: "Unknown".to_string(),
                date: chrono::NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            }
        }
    }
    // Creating instances of Person
    let husband = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
    };
    let wife = Person {
        first_name: "Jane".to_string(),
        last_name: "Smith".to_string(),
    };
    // Using the Add trait to create a Marriage instance
    let marriage = husband + wife;
    println!("Marriage: {:#?}", marriage);

    println!(
        "============================= Example 2: Overloading the `+` operator for a custom type ============================="
    );
    example_add();

    println!(
        "============================= Example 3: Overloading the `*` operator for a custom type ============================="
    );
    example_mul();

    println!(
        "============================= Example 4: Overloading the `-` operator for a custom type ============================="
    );
    example_sub();

    println!(
        "============================= Example 5: Overloading the `/` operator for a custom type ============================="
    );
    example_div();
}

fn example_add() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Self::Output {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;
    println!("Point after addition: {:?}", p3);
}

fn example_mul() {
    #[derive(Debug)]
    struct Vector {
        x: f64,
        y: f64,
    }

    impl Mul<f64> for Vector {
        type Output = Vector;

        fn mul(self, scalar: f64) -> Self::Output {
            Vector {
                x: self.x * scalar,
                y: self.y * scalar,
            }
        }
    }

    let v = Vector { x: 2.0, y: 3.0 };
    let scaled_v = v * 2.0;
    println!("Scaled vector: {:?}", scaled_v);
}

fn example_sub() {
    #[derive(Debug)]
    struct Time {
        hours: i32,
        minutes: i32,
    }

    impl Sub for Time {
        type Output = Time;

        fn sub(self, other: Time) -> Self::Output {
            let total_minutes_self = self.hours * 60 + self.minutes;
            let total_minutes_other = other.hours * 60 + other.minutes;
            let diff = total_minutes_self - total_minutes_other;

            Time {
                hours: diff / 60,
                minutes: diff % 60,
            }
        }
    }

    let t1 = Time {
        hours: 5,
        minutes: 30,
    };
    let t2 = Time {
        hours: 2,
        minutes: 15,
    };
    let t3 = t1 - t2;
    println!("Time after subtraction: {:?}", t3);
}

fn example_div() {
    #[derive(Debug)]
    struct Fraction {
        numerator: i32,
        denominator: i32,
    }

    impl Div for Fraction {
        type Output = Fraction;

        fn div(self, other: Fraction) -> Self::Output {
            Fraction {
                numerator: self.numerator * other.denominator,
                denominator: self.denominator * other.numerator,
            }
        }
    }

    let f1 = Fraction {
        numerator: 1,
        denominator: 2,
    };
    let f2 = Fraction {
        numerator: 3,
        denominator: 4,
    };
    let f3 = f1 / f2;
    println!("Fraction after division: {:?}", f3);
}
