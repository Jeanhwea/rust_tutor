// Category: algorithms
// Level: Medium
// Percent: 41.02461%

// You are given two non-empty linked lists representing two non-negative
// integers. The digits are stored in reverse order, and each of their nodes
// contains a single digit. Add the two numbers and return the sum as a linked
// list.
//
// You may assume the two numbers do not contain any leading zero, except the
// number 0 itself.
//
//
// Example 1:
//
// Input: l1 = [2,4,3], l2 = [5,6,4]
// Output: [7,0,8]
// Explanation: 342 + 465 = 807.
//
//
// Example 2:
//
// Input: l1 = [0], l2 = [0]
// Output: [0]
//
//
// Example 3:
//
// Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
// Output: [8,9,9,9,0,0,0,1]
//
//
//
// Constraints:
//
//   1. The number of nodes in each linked list is in the range [1, 100].
//   2. 0 <= Node.val <= 9
//   3. It is guaranteed that the list represents a number that does not have
//      leading zeros.
//

mod list;

use list::ListNode;

// BEG_SUBMIT
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut tail = &mut result;
        let mut state = (l1, l2, 0, 0); // (list1, list2, sum, carry)
        loop {
            state = match state {
                (None, None, _, 0) => break,
                (None, None, _, carry) => (None, None, carry, 0),
                (Some(a), None, _, carry) | (None, Some(a), _, carry) if a.val + carry >= 10 => {
                    (a.next, None, a.val + carry - 10, 1)
                }
                (Some(a), None, _, carry) | (None, Some(a), _, carry) => {
                    (a.next, None, a.val + carry, 0)
                }
                (Some(a), Some(b), _, carry) if a.val + b.val + carry >= 10 => {
                    (a.next, b.next, a.val + b.val + carry - 10, 1)
                }
                (Some(a), Some(b), _, carry) => (a.next, b.next, a.val + b.val + carry, 0),
            };
            *tail = Some(Box::new(ListNode::new(state.2)));
            tail = &mut tail.as_mut().unwrap().next;
        }
        result
    }
}
// END_SUBMIT

struct Solution;

fn main() {
    let l1 = Some(Box::new(ListNode::new(1)));
    let l2 = Some(Box::new(ListNode::new(2)));
    let ans = Solution::add_two_numbers(l1, l2);
    println!("ans = {:?}", ans);
}
