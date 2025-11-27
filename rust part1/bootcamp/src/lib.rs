// src/lib.rs

// ---------- 1. BASICS ----------
pub fn greet(name: &str) -> String {
    // TODO: return "Hello, <name>!"
    //todo!()
    return format!("Hello, {}!", name);
}

pub fn sum(nums: &[i32]) -> i32 {
    // TODO: sum all elements using an iterator
   return nums.iter().sum(); 
}

pub fn flip(b: bool) -> bool {
    // TODO: return the opposite boolean
    return !b;
}

// ---------- 2. OWNERSHIP & BORROWING ----------
pub fn take_ownership(s: String) -> usize {
    // TODO: return the length of s (taking ownership)
    // hint: no borrowing here; s moves in
    // took out return, I use implizit returns
    s.len() 
}

pub fn borrow_first_char(s: &str) -> Option<char> {
    // TODO: return the first char without taking ownership
    // todo!()
    let first_character = s.chars().next();
    first_character
}

pub fn push_exclamation(s: &mut String) {
    // TODO: mutate s by appending a single '!' character
    //todo!()
    s.push('!');
}

// ---------- 3. STRUCTS, ENUMS, METHODS ----------
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance_to(&self, other: &Point) -> f64 {
        // TODO: Euclidean distance
        todo!()
    }

    pub fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    Circle { center: Point, radius: f64 },
    Rect { top_left: Point, w: f64, h: f64 },
}

impl Shape {
    pub fn area(&self) -> f64 {
        // TODO: match on self, compute area
        todo!()
    }
}

// ---------- 4. TRAITS & GENERICS ----------
pub trait Plottable {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
}

impl Plottable for Point {
    fn x(&self) -> f64 { self.x }
    fn y(&self) -> f64 { self.y }
}

impl Plottable for (f64, f64) {
    fn x(&self) -> f64 { self.0 }
    fn y(&self) -> f64 { self.1 }
}

// Return a reference to the item farthest from the origin.
// Note the explicit lifetime tying the returned reference to the input slice.
pub fn furthest_from_origin<T: Plottable>(items: &[T]) -> Option<&T> {
    // TODO: iterate, compute squared distance, track max, return reference
    todo!()
}

// ---------- 5. ERRORS & OPTION/RESULT ----------
pub fn parse_port(s: &str) -> Result<u16, String> {
    // TODO: parse string into u16; map errors into a friendly String
    // hint: use s.parse::<u16>()
    todo!()
}

// ---------- 6. ITERATORS & CLOSURES ----------
pub fn even_squares(n: u32) -> Vec<u32> {
    // TODO: all even numbers from 0..=n, squared, collected to Vec
    // hint: (0..=n).filter(...).map(...).collect()
    todo!()
}

// ---------- 7. USING A CRATE (rand) ----------
pub fn roll_dice(sides: u8) -> u8 {
    // TODO: return a value in 1..=sides using rand::Rng
    // note: assume sides >= 1
    todo!()
}

