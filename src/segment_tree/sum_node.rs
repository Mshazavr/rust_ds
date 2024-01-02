use super::segment_tree::SegmentTreeState;

#[derive(Debug, Clone)]
pub struct SumNode {
    sum: i64,
    delta: i64,
    left: usize,  // could be optimized away
    right: usize, // could be optimized away
}

impl SegmentTreeState for SumNode {
    type LeafT = i64;
    type DeltaT = i64;

    fn merge(a: Self::LeafT, b: Self::LeafT) -> Self::LeafT {
        a + b
    }

    fn new() -> Self {
        Self {
            sum: 0,
            delta: 0,
            left: 0,
            right: 0,
        }
    }

    fn new_with_leaf(leaf_value: Self::LeafT, leaf_pos: usize) -> Self {
        Self {
            sum: leaf_value,
            delta: 0,
            left: leaf_pos,
            right: leaf_pos,
        }
    }

    fn new_with_children(left_child: &Self, right_child: &Self) -> Self {
        Self {
            sum: left_child.sum + right_child.sum,
            delta: 0,
            left: left_child.left,
            right: right_child.right,
        }
    }

    fn get_delta(&self) -> Self::DeltaT {
        self.delta
    }

    fn update_delta(&mut self, delta: &Self::DeltaT) {
        self.delta += delta;
    }

    fn get_value(&self) -> Self::LeafT {
        self.sum
    }

    fn push(&mut self, left_child: &mut Self, right_child: &mut Self) {
        self.sum += ((self.right as i64) - (self.left as i64) + 1) * self.delta;
        left_child.delta += self.delta;
        right_child.delta += self.delta;
        self.delta = 0;
    }

    fn push_leaf(&mut self) {
        self.sum += self.delta;
        self.delta = 0;
    }

    fn compute(&mut self, left_child: &Self, right_child: &Self) {
        // The assert should be removed for efficiency
        assert!(self.delta == 0 && left_child.delta == 0 && right_child.delta == 0);

        self.sum = left_child.sum + right_child.sum;
    }
}
