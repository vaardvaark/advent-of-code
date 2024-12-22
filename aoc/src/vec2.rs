use crate::Cardinal;
use std::ops;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

impl Vec2 {
    #[inline]
    #[must_use]
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    #[inline]
    #[must_use]
    pub fn left() -> Self {
        Vec2 { x: -1, y: 0 }
    }

    #[inline]
    #[must_use]
    pub fn right() -> Self {
        Vec2 { x: 1, y: 0 }
    }

    #[inline]
    #[must_use]
    pub fn up() -> Self {
        Vec2 { x: 0, y: -1 }
    }

    #[inline]
    #[must_use]
    pub fn down() -> Self {
        Vec2 { x: 0, y: 1 }
    }

    #[inline]
    #[must_use]
    pub fn reverse(&self) -> Self {
        let Vec2 { x, y } = self;
        Vec2 { x: -x, y: -y }
    }

    pub fn with_x(mut self, x: i64) -> Self {
        self.x = x;
        self
    }

    pub fn with_y(mut self, y: i64) -> Self {
        self.y = y;
        self
    }

    pub fn translate(mut self, r: impl Into<Vec2>) -> Self {
        let r = r.into();
        self.x += r.x;
        self.y += r.y;
        self
    }
}

impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
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

impl<T: Into<i64>> ops::Mul<T> for Vec2 {
    type Output = Vec2;
    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        let Vec2 { x, y } = self;
        let rhs = rhs.into();
        Vec2 {
            x: x * rhs,
            y: y * rhs,
        }
    }
}

impl<T: Into<i64>> ops::MulAssign<T> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.x *= rhs;
        self.y *= rhs;
    }
}

macro_rules! impl_from_tuple {
    ($t:ty) => {
        impl From<($t, $t)> for Vec2 {
            #[inline]
            fn from((x, y): ($t, $t)) -> Self {
                Self {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                }
            }
        }
    };
    ($t:ty, $($tail:ty),+) => {
        impl_from_tuple!($t);
        impl_from_tuple!($($tail),+);
    };
}

impl_from_tuple!(isize, i64, i32, i16, i8, usize, u64, u32, u16, u8);

macro_rules! impl_into_tuple {
    ($t:ty) => {
        impl From<Vec2> for ($t, $t) {
            #[inline]
            fn from(Vec2 { x, y }: Vec2) -> Self {
                (x.try_into().unwrap(), y.try_into().unwrap())
            }
        }
    };
    ($t:ty, $($tail:ty),+) => {
        impl_into_tuple!($t);
        impl_into_tuple!($($tail),+);
    };
}

impl_into_tuple!(isize, i64, i32, i16, i8, usize, u64, u32, u16, u8);

impl From<&Vec2> for Vec2 {
    fn from(value: &Vec2) -> Self {
        *value
    }
}

impl From<Cardinal> for Vec2 {
    #[inline]
    fn from(value: Cardinal) -> Self {
        match value {
            Cardinal::North => Self::up(),
            Cardinal::East => Self::right(),
            Cardinal::South => Self::down(),
            Cardinal::West => Self::left(),
        }
    }
}

impl TryFrom<Vec2> for Cardinal {
    type Error = String;
    fn try_from(value: Vec2) -> Result<Self, Self::Error> {
        match value {
            Vec2 { x, y: 0 } if x > 0 => Ok(Cardinal::East),
            Vec2 { x, y: 0 } if x < 0 => Ok(Cardinal::West),
            Vec2 { x: 0, y } if y > 0 => Ok(Cardinal::North),
            Vec2 { x: 0, y } if y < 0 => Ok(Cardinal::West),
            _ => Err(format!("{value} is not a cardinal direction")),
        }
    }
}
