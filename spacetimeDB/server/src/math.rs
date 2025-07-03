use spacetimedb::SpacetimeType;

#[derive(SpacetimeType, Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl std::ops::Add<&Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::iter::Sum<Vec2> for Vec2 {
    fn sum<I: Iterator<Item = Vec2>>(iter: I) -> Self {
        let mut r = Vec2::new(0.0, 0.0);
        for val in iter {
            r += val;
        }
        r
    }
}

impl std::ops::Sub<&Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, other: f32) -> Vec2 {
        Vec2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, other: f32) -> Vec2 {
        if other != 0.0 {
            Vec2 {
                x: self.x / other,
                y: self.y / other,
            }
        } else {
            Vec2 { x: 0.0, y: 0.0 }
        }
    }
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn sqr_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalized(self) -> Vec2 {
        self / self.magnitude()
    }
}
