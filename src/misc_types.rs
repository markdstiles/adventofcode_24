use std::{fmt::Display, ops::{Add, AddAssign, Sub, SubAssign}};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector2D {
    pub x: i32,
    pub y: i32,
}

impl Vector2D {
    pub fn new(x: i32, y: i32) -> Vector2D {
        Vector2D { x, y }
    }
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Rect {
        Rect { left, top, right, bottom }
    }

    pub fn is_inside(&self, pos: Point<i32>) -> bool {
        pos.x >= self.left
        && pos.x < self.right
        && pos.y >= self.top
        && pos.y < self.bottom
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rect(L:{},T:{},R:{},B:{})", self.left, self.top, self.right, self.bottom)
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> 
    where
        T: Sized,
        T: Ord,
        T: Copy {
            
    pub fn new(x: T, y: T) -> Point<T> {
        Point {
            x,
            y,
        }
    }

    pub fn clamp(self, min: T, max: T) -> Point<T> {
        Point {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
        }
    }
}

impl Point<i32> {
    pub fn inside(self, bounding_box: Rect) -> bool {
        bounding_box.is_inside(self)
    }
}

impl From<Point<i32>> for Point<usize> {
    fn from(value: Point<i32>) -> Self {
        Point {
            x: value.x.try_into().unwrap(),
            y: value.y.try_into().unwrap(),
        }
    }
}

impl From<Point<usize>> for Point<i32> {
    fn from(value: Point<usize>) -> Self {
        Point {
            x: value.x.try_into().unwrap(),
            y: value.y.try_into().unwrap(),
        }
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> AddAssign for Point<T>
    where T: AddAssign, {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T> SubAssign for Point<T>
    where T: SubAssign, {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T> Display for Point<T>
where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x:{},y:{})", self.x, self.y)
    }
}