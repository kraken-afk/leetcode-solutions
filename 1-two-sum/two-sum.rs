impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            let mut indexes: Vec<i32> = vec![0, 1];
            let len: usize = nums.len();

            while indexes[0] < len as i32 {
                let n1: &i32 = nums.get(indexes[0] as usize).unwrap();
                let n2: &i32 = nums.get(indexes[1] as usize).unwrap();

                if indexes[1] == indexes[0] {
                    indexes[1] += 1;
                    continue;
                }
                if n1 + n2 == target {
                    return indexes;
                }

                indexes[1] += 1;

                if indexes[1] >= len as i32 {
                    indexes[1] = 0;
                    indexes[0] += 1;

                    continue;
                }
            }
            panic!("No value that fulfill the target");
    }
}