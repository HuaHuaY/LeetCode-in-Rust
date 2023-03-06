pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
        fn create_name_index(name: String, index: usize) -> String {
            format!("{}({})", name, index)
        }

        let mut result = Vec::with_capacity(names.len());
        let mut map = HashMap::new();
        for name in names {
            if let Some(next) = map.get(&name) {
                let mut i = *next;
                while map.get(&create_name_index(name.clone(), i)).is_some() {
                    i += 1;
                }
                let new_name = create_name_index(name.clone(), i);
                result.push(new_name.clone());
                map.insert(name, i + 1);
                map.insert(new_name, 1);
            } else {
                map.insert(name.clone(), 1);
                result.push(name);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecString;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::get_folder_names(["pes", "fifa", "gta", "pes(2019)"].to_vec_string()),
            ["pes", "fifa", "gta", "pes(2019)"]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::get_folder_names(["gta", "gta(1)", "gta", "avalon"].to_vec_string()),
            ["gta", "gta(1)", "gta(2)", "avalon"]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::get_folder_names(
                [
                    "onepiece",
                    "onepiece(1)",
                    "onepiece(2)",
                    "onepiece(3)",
                    "onepiece"
                ]
                .to_vec_string()
            ),
            [
                "onepiece",
                "onepiece(1)",
                "onepiece(2)",
                "onepiece(3)",
                "onepiece(4)"
            ]
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::get_folder_names(["kaido", "kaido(1)", "kaido", "kaido(1)"].to_vec_string()),
            ["kaido", "kaido(1)", "kaido(2)", "kaido(1)(1)"]
        );
    }
}
