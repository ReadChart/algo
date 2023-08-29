use crate::solution::Solution;

mod solution;
mod leetcode;

/// This is a template for future algo practice
impl Solution {
    pub fn resolve() -> i32 {
        2
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::resolve(), 2);
    }
}
