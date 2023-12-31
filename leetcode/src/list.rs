// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        vec_to_list(vec)
    }

    pub fn into_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut rest = list;
        std::iter::from_fn(move || {
            if let Some(node) = rest.as_mut() {
                let val = Some(node.val);
                rest = node.next.take();
                val
            } else {
                None
            }
        })
        .collect()
    }
}

pub fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    vec.into_iter()
        .rev()
        .fold(None, |next, val| Box::new(ListNode { val, next }).into())
}

#[macro_export]
macro_rules! list {
    () => {None};
    ($elem:expr; $n:expr) => {
        $crate::ListNode::from_vec(vec![$elem; $n])
    };
    ($($x:expr),+ $(,)?) => {
        $crate::ListNode::from_vec(vec![$($x),+])
    };
}

fn main() {
    let a = ListNode::from_vec(vec![1, 2, 3]);
    println!("{:?}", a);
    let b = ListNode::into_vec(a);
    println!("{:?}", b);
}
