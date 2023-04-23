use rapier2d::na::{Rotation2, Vector2};

#[derive(Clone, Debug, Default)]
pub struct Transform {
    pub location: (f32, f32),
    pub angle: f32,
}

impl Transform {
    pub fn new(location: (f32, f32), angle: f32) -> Self {
        Self { location, angle }
    }
}

pub fn rotate_vec2(radian: f32, vector: (f32, f32)) -> (f32, f32) {
    let rotated = Rotation2::new(radian) * Vector2::from([vector.0, vector.1]);

    (rotated.x, rotated.y)
}

#[cfg(test)]
mod tests {
    use crate::theory::geometry::rotate_vec2;
    use std::f32::consts;

    #[test]
    fn rotate_vec2_should_rotate_vec2_correctly() {
        assert_approx_eq(rotate_vec2(consts::FRAC_PI_2, (0.0, 1.0)), (-1.0, 0.0));

        assert_approx_eq(rotate_vec2(-consts::FRAC_PI_2, (0.0, 1.0)), (1.0, 0.0));
    }

    fn assert_approx_eq(left: (f32, f32), right: (f32, f32)) {
        if left == right {
            return;
        }

        if (left.0 - right.0).abs() >= f32::EPSILON {
            assert_eq!(left, right);
        }

        if (left.1 - right.1).abs() >= f32::EPSILON {
            assert_eq!(left, right);
        }
    }
}
