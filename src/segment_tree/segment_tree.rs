use core::cmp;
use std::fmt::Display;

pub trait SegmentTreeState: Clone + Sized {
    type LeafT: Default + Copy + Display;
    type DeltaT;
    fn merge(a: Self::LeafT, b: Self::LeafT) -> Self::LeafT;
    fn new() -> Self;
    fn new_with_leaf(leaf_value: Self::LeafT, leaf_pos: usize) -> Self;
    fn new_with_children(left_child: &Self, right_child: &Self) -> Self;
    fn get_delta(&self) -> Self::DeltaT;
    fn update_delta(&mut self, delta: &Self::DeltaT);
    fn get_value(&self) -> Self::LeafT;
    fn push(&mut self, left_child: &mut Self, right_child: &mut Self);
    fn push_leaf(&mut self);
    fn compute(&mut self, left_child: &Self, right_child: &Self);
}

pub struct SegmentTree<NodeT: SegmentTreeState> {
    node_state: NodeT,
    left_node: Option<Box<SegmentTree<NodeT>>>,
    right_node: Option<Box<SegmentTree<NodeT>>>,
    range_left: usize,
    range_right: usize,
}

impl<NodeT: SegmentTreeState> SegmentTree<NodeT> {
    fn _new(arr: &Vec<NodeT::LeafT>, left: usize, right: usize) -> Self {
        if left == right {
            return Self {
                node_state: NodeT::new_with_leaf(arr[left], left),
                left_node: None,
                right_node: None,
                range_left: left,
                range_right: right,
            };
        }
        let middle = (left + right) / 2;
        let left_node = Self::_new(arr, left, middle);
        let right_node = Self::_new(arr, middle + 1, right);
        return Self {
            node_state: NodeT::new_with_children(&left_node.node_state, &right_node.node_state),
            left_node: Some(Box::new(left_node)),
            right_node: Some(Box::new(right_node)),
            range_left: left,
            range_right: right,
        };
    }

    pub fn new(arr: &Vec<NodeT::LeafT>) -> Self {
        Self::_new(arr, 0, ((arr.len() as i32) - 1) as usize)
    }

    fn _push_node(&mut self) {
        if self.range_left == self.range_right {
            self.node_state.push_leaf();
        } else {
            self.node_state.push(
                &mut (*self.left_node.as_mut().unwrap()).node_state,
                &mut (*self.right_node.as_mut().unwrap()).node_state,
            );
        }
    }

    pub fn update(&mut self, left: usize, right: usize, delta: &NodeT::DeltaT) {
        let left = cmp::max(left, self.range_left);
        let right = cmp::min(right, self.range_right);
        if left > right {
            self._push_node();
            return;
        }

        if left == self.range_left && right == self.range_right {
            self.node_state.update_delta(delta);
            self._push_node();
            return;
        }

        self._push_node();
        (*self.left_node.as_mut().unwrap()).update(left, right, delta);
        (*self.right_node.as_mut().unwrap()).update(left, right, delta);

        self.node_state.compute(
            &(*self.left_node.as_ref().unwrap()).node_state,
            &(*self.right_node.as_ref().unwrap()).node_state,
        );
    }

    pub fn query(&mut self, left: usize, right: usize) -> NodeT::LeafT {
        let left = cmp::max(left, self.range_left);
        let right = cmp::min(right, self.range_right);

        if left > right {
            return NodeT::LeafT::default();
        }

        self._push_node();
        if left == self.range_left && right == self.range_right {
            return self.node_state.get_value();
        }

        return NodeT::merge(
            (*self.left_node.as_mut().unwrap()).query(left, right),
            (*self.right_node.as_mut().unwrap()).query(left, right),
        );
    }
}
