pub struct Solution;

impl Solution {
    pub fn min_number_of_hours(
        initial_energy: i32,
        initial_experience: i32,
        energy: Vec<i32>,
        experience: Vec<i32>,
    ) -> i32 {
        let need_energy = (energy.into_iter().sum::<i32>() + 1 - initial_energy).max(0);
        let mut need_experience = 0;
        let mut sum = 0;
        for experience in experience {
            need_experience = need_experience.max(experience + 1 - sum);
            sum += experience;
        }
        need_energy + (need_experience - initial_experience).max(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_number_of_hours(5, 3, [1, 4, 3, 2].to_vec(), [2, 6, 3, 1].to_vec()),
            8
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_number_of_hours(2, 4, [1].to_vec(), [3].to_vec()),
            0
        );
    }
}
