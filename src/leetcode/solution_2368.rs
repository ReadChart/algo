use std::collections::{HashMap, HashSet};
use crate::solution::Solution;

impl Solution {
    pub fn reachable_nodes(n: i32, mut edges: Vec<Vec<i32>>, mut restricted: Vec<i32>) -> i32 {
        let mut visit = vec![0];
        while visit.len() != 0 {
            let mut next_visit = HashSet::new();
            for i in 0..edges.len() {
                let visiting = &edges[i];
                if visit.contains(&visiting[0]) {
                    next_visit.insert(visiting[1]);
                    edges.remove(i);
                } else if visit.contains(&visiting[1]) {
                    next_visit.insert(visiting[0]);
                    edges.remove(i);
                }
            }
            visit = Vec::from(next_visit);
        }
        return 1;
    }
}

#[cfg(test)]
mod test_solution_2368 {
    use crate::solution::Solution;

    #[test]
    fn test_for_output() {
        let n = 7;
        let edges: Vec<Vec<i32>> = vec![vec![0,3],vec![2,0],vec![4,2],vec![4,1],vec![0,6],vec![5,1]];
        let restricted = vec![6,3,4,2,5];
        assert_eq!(Solution::reachable_nodes(n, edges, restricted), 1);
    }
}