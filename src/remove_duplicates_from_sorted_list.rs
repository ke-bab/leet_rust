#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        return Option::None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: Vec<i32>,
        want: Vec<i32>,
    }

    #[test]
    fn tests() {
        let cases = vec![
            TestCase {
                input: vec![1, 1, 2],
                want: vec![1, 2],
            },
            TestCase {
                input: vec![1, 1, 2, 3, 3],
                want: vec![1, 2, 3],
            },
            TestCase {
                input: vec![],
                want: vec![],
            },
        ];

        for case in cases {
            let got = Solution::delete_duplicates(vec_to_node(&case.input));
            assert!(check_result(&vec_to_node(&case.want), &got));
        }
    }

    fn vec_to_node(case: &Vec<i32>) -> Option<Box<ListNode>> {

    }

    fn check_result(want: &Option<Box<ListNode>>, got: &Option<Box<ListNode>>) -> bool {
        let mut next_got = got;
        let mut next_want = want;
        loop {
            if next_want.is_some() {
                if next_got.is_some() {
                    if next_want.unwrap().val != next_got.unwrap().val {
                        return false
                    } else {
                        next_want = &next_want.unwrap().next;
                        next_got = &next_got.unwrap().next;
                    }
                } else {
                    return false
                }
            } else {
                break;
            }
        }

        true
    }
}

