use crate::Direction;
use std::ops;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

impl Vec2 {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn left() -> Self {
        Vec2 { x: -1, y: 0 }
    }

    pub fn right() -> Self {
        Vec2 { x: 1, y: 0 }
    }

    pub fn up() -> Self {
        Vec2 { x: 0, y: -1 }
    }

    pub fn down() -> Self {
        Vec2 { x: 0, y: 1 }
    }

    pub fn reverse(&self) -> Self {
        let Vec2 { x, y } = self;
        Vec2 { x: -x, y: -y }
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let Vec2 { x: ax, y: ay } = self;
        let Vec2 { x: bx, y: by } = rhs;
        Vec2 {
            x: ax - bx,
            y: ay - by,
        }
    }
}

impl ops::Sub<&Vec2> for &Vec2 {
    type Output = Vec2;
    #[inline]
    fn sub(self, rhs: &Vec2) -> Self::Output {
        let Vec2 { x: ax, y: ay } = self;
        let Vec2 { x: bx, y: by } = rhs;
        Vec2 {
            x: ax - bx,
            y: ay - by,
        }
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    #[inline]
    fn add(self, rhs: Vec2) -> Self::Output {
        let Vec2 { x: ax, y: ay } = self;
        let Vec2 { x: bx, y: by } = rhs;
        Vec2 {
            x: ax + bx,
            y: ay + by,
        }
    }
}

impl ops::AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Mul<i64> for Vec2 {
    type Output = Vec2;
    #[inline]
    fn mul(self, rhs: i64) -> Self::Output {
        let Vec2 { x, y } = self;
        Vec2 {
            x: x * rhs,
            y: y * rhs,
        }
    }
}

impl ops::MulAssign<i64> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: i64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl From<(i64, i64)> for Vec2 {
    #[inline]
    fn from((x, y): (i64, i64)) -> Self {
        Self { x, y }
    }
}

impl From<(usize, usize)> for Vec2 {
    #[inline]
    fn from((x, y): (usize, usize)) -> Self {
        assert!(x <= i64::MAX as usize && y <= i64::MAX as usize);
        Self {
            x: x as i64,
            y: y as i64,
        }
    }
}

impl From<Vec2> for (i64, i64) {
    #[inline]
    fn from(Vec2 { x, y }: Vec2) -> Self {
        (x, y)
    }
}

impl From<Direction> for Vec2 {
    #[inline]
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Self::up(),
            Direction::Right => Self::right(),
            Direction::Down => Self::down(),
            Direction::Left => Self::left(),
        }
    }
}
