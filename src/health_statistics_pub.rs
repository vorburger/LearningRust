// https://google.github.io/comprehensive-rust/exercises/day-2/health-statistics.html

// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

struct User {
    pub name: String,
    pub age: u32,
    pub weight: f32,
}

impl User {
    // new() is just a convention, not a fixed keyword
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        User {
            name: name,
            age: age,
            weight: weight,
        }
    }
}

pub fn main() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    println!("I'm {} and my age is {}", bob.name, bob.age);
}

#[test]
fn test_weight() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.weight, 155.2);
}

#[test]
fn test_set_age() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.age, 32);
    bob.age = 33;
    assert_eq!(bob.age, 33);
}
