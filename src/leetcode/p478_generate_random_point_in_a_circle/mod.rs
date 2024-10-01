use rand::Rng;

pub struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    pub fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Solution {
            radius,
            x_center,
            y_center,
        }
    }

    pub fn rand_point(&self) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        let mut dx = rng.gen_range(-self.radius..self.radius);
        let mut dy = rng.gen_range(-self.radius..self.radius);
        while dx * dx + dy * dy > self.radius * self.radius {
            dx = rng.gen_range(-self.radius..self.radius);
            dy = rng.gen_range(-self.radius..self.radius);
        }
        vec![self.x_center + dx, self.y_center + dy]
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(radius, x_center, y_center);
 * let ret_1: Vec<f64> = obj.rand_point();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let radius = 1.0;
        let x_center = 0.0;
        let y_center = 0.0;
        let solution = Solution::new(radius, x_center, y_center);
        fn check(point: Vec<f64>, radius: f64, x_center: f64, y_center: f64) -> bool {
            (point[0] - x_center) * (point[0] - x_center)
                + (point[1] - y_center) * (point[1] - y_center)
                <= radius * radius
        }
        assert!(check(solution.rand_point(), radius, x_center, y_center));
        assert!(check(solution.rand_point(), radius, x_center, y_center));
        assert!(check(solution.rand_point(), radius, x_center, y_center));
    }
}
