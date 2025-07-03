use spacetimedb::SpacetimeType;

#[derive(SpacetimeType, Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub z: f32,
}

impl std::ops::Add<&Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            z: self.z + other.z,
        }
    }
}

impl std::ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.z += rhs.z;
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
            z: self.z - other.z,
        }
    }
}

impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            z: self.z - other.z,
        }
    }
}

impl std::ops::SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.z -= rhs.z;
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, other: f32) -> Vec2 {
        Vec2 {
            x: self.x * other,
            z: self.z * other,
        }
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, other: f32) -> Vec2 {
        if other != 0.0 {
            Vec2 {
                x: self.x / other,
                z: self.z / other,
            }
        } else {
            Vec2 { x: 0.0, z: 0.0 }
        }
    }
}

impl Vec2 {
    pub fn new(x: f32, z: f32) -> Self {
        Self { x, z }
    }

    pub fn sqr_magnitude(&self) -> f32 {
        self.x * self.x + self.z * self.z
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.z * self.z).sqrt()
    }

    pub fn normalized(self) -> Vec2 {
        self / self.magnitude()
    }
}
