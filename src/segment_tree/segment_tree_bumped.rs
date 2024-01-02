use super::segment_tree::SegmentTreeState;
use core::cmp;

pub struct SegmentTreeBmp<NodeT: SegmentTreeState> {
    nodes: Vec<NodeT>,
    arr_size: usize,
}

impl<NodeT: SegmentTreeState> SegmentTreeBmp<NodeT> {
    fn _populate(
        arr: &Vec<NodeT::LeafT>,
        nodes: &mut Vec<NodeT>,
        node_id: usize,
        left: usize,
        right: usize,
    ) {
        if left == right {
            nodes[node_id] = NodeT::new_with_leaf(arr[left], left);
            return;
        }
        let middle = (left + right) / 2;
        Self::_populate(arr, nodes, node_id * 2 + 1, left, middle);
        Self::_populate(arr, nodes, node_id * 2 + 2, middle + 1, right);
        nodes[node_id] = NodeT::new_with_children(&nodes[node_id * 2 + 1], &nodes[node_id * 2 + 2]);
    }

    pub fn new(arr: &Vec<NodeT::LeafT>) -> Self {
        let mut nodes = vec![NodeT::new(); arr.len() * 5];
        Self::_populate(arr, &mut nodes, 0, 0, ((arr.len() as i32) - 1) as usize);
        Self {
            nodes,
            arr_size: arr.len(),
        }
    }

    fn _push_node(&mut self, node_id: usize, left: usize, right: usize) {
        if left == right {
            self.nodes[node_id].push_leaf();
        } else {
            let (root_portion, rest) = self.nodes.split_at_mut(node_id + 1);
            let (l_nodes, r_nodes) = rest.split_at_mut(node_id * 2 + 2 - node_id - 1);

            let root_element = &mut root_portion[node_id];
            root_element.push(&mut l_nodes[node_id * 2 + 1 - node_id - 1], &mut r_nodes[0]);
        }
    }

    fn _update_subtree(
        &mut self,
        delta: &NodeT::DeltaT,
        node_id: usize,
        left: usize,
        right: usize,
        left_update: usize,
        right_update: usize,
    ) {
        if left_update > right_update {
            self._push_node(node_id, left, right);
            return;
        }

        if left_update == left && right_update == right {
            self.nodes[node_id].update_delta(delta);
            self._push_node(node_id, left, right);
            return;
        }

        let middle = (left + right) / 2;

        self._push_node(node_id, left, right);
        self._update_subtree(
            delta,
            node_id * 2 + 1,
            left,
            middle,
            left_update,
            cmp::min(right_update, middle),
        );
        self._update_subtree(
            delta,
            node_id * 2 + 2,
            middle + 1,
            right,
            cmp::max(left_update, middle + 1),
            right_update,
        );
        let (root_portion, rest) = self.nodes.split_at_mut(node_id + 1);
        let root_element = &mut root_portion[node_id];
        root_element.compute(
            &rest[node_id * 2 + 1 - node_id - 1],
            &rest[node_id * 2 + 2 - node_id - 1],
        );
    }

    pub fn update(&mut self, left: usize, right: usize, delta: &NodeT::DeltaT) {
        self._update_subtree(
            delta,
            0,
            0,
            ((self.arr_size as i32) - 1) as usize,
            left,
            right,
        );
    }

    fn _query_subtree(
        &mut self,
        node_id: usize,
        left: usize,
        right: usize,
        left_query: usize,
        right_query: usize,
    ) -> NodeT::LeafT {
        if left_query > right_query {
            return NodeT::LeafT::default();
        }

        self._push_node(node_id, left, right);
        if left_query == left && right_query == right {
            return self.nodes[node_id].get_value();
        }

        let middle = (left + right) / 2;

        return NodeT::merge(
            self._query_subtree(
                node_id * 2 + 1,
                left,
                middle,
                left_query,
                cmp::min(right_query, middle),
            ),
            self._query_subtree(
                node_id * 2 + 2,
                middle + 1,
                right,
                cmp::max(left_query, middle + 1),
                right_query,
            ),
        );
    }

    pub fn query(&mut self, left: usize, right: usize) -> NodeT::LeafT {
        return self._query_subtree(0, 0, ((self.arr_size as i32) - 1) as usize, left, right);
    }
}
