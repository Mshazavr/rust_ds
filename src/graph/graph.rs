use core::hash::Hash;
use std::cell::Cell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::mem::swap;
use std::slice::Iter;

#[non_exhaustive]
pub struct GraphCategoryBits;
impl GraphCategoryBits {
    const BIDIR_BIT: u8 = (1 << 0);
    const TREE_BIT: u8 = (1 << 1);
    const CONNECTED_BIT: u8 = (1 << 2);
    const FOREST_BIT: u8 = (1 << 3);

    const DAG_BIT: u8 = (1 << 4);

    fn name_from_bit(bit: u8) -> String {
        match bit {
            GraphCategoryBits::BIDIR_BIT => "Bidirectional",
            GraphCategoryBits::TREE_BIT => "Tree",
            GraphCategoryBits::CONNECTED_BIT => "Connected-bidirectional",
            GraphCategoryBits::FOREST_BIT => "Forest",
            GraphCategoryBits::DAG_BIT => "Directed-acyclic-graph",
            _ => panic!("Tried converting invalid bit to name"),
        }
        .to_string()
    }
}

#[derive(Clone, Debug)]
pub struct TreeNode {
    parent_id: usize,
    enter_time: usize,
    level: usize,
    pow2_ancestors: Option<Vec<usize>>,
    exit_time: Option<usize>,
    subtree_upnode_cnt: Option<usize>,
    subtree_sz: Option<usize>,
}

#[derive(Clone)]
pub struct Graph<N, E> {
    pub node_map: HashMap<N, usize>,
    node_map_rev: Vec<N>,
    nbs: Vec<Vec<(usize, E)>>,
    category: u8,
    pub rooted_tree_infos: Option<Box<Vec<TreeNode>>>,
}

impl<N, E> Graph<N, E>
where
    N: Eq + Hash + Clone + Debug,
    E: Clone + Debug,
{
    pub fn new<'a, NodeIterT, EdgeIterT>(nodes: &'a NodeIterT, edges: &'a EdgeIterT) -> Self
    where
        &'a NodeIterT: IntoIterator<Item = &'a N>,
        &'a EdgeIterT: IntoIterator<Item = &'a (N, N, E)>,
        N: 'a,
        E: 'a,
    {
        let mut node_map: HashMap<N, usize> = HashMap::new();
        let mut node_map_rev: Vec<N> = Vec::new();
        let mut node_count = 0;
        for node in nodes {
            if node_map.get(node).is_none() {
                node_map.insert(node.clone(), node_count);
                node_map_rev.push(node.clone());
                node_count += 1;
            }
        }

        let mut nbs: Vec<Vec<(usize, E)>> = vec![vec![]; node_count];
        for edge in edges {
            let node1 = node_map.get(&edge.0).unwrap();
            let node2 = node_map.get(&edge.1).unwrap();
            nbs[*node1].push((*node2, edge.2.clone()));
        }

        let graph = Self {
            node_map,
            node_map_rev,
            nbs,
            category: 0,
            rooted_tree_infos: None,
        };

        let mut category = 0;
        if !graph._has_cycle() {
            category += GraphCategoryBits::DAG_BIT;
        }

        Self { category, ..graph }
    }

    pub fn new_bidir<'a, NodeIterT, EdgeIterT>(nodes: &'a NodeIterT, edges: &'a EdgeIterT) -> Self
    where
        &'a NodeIterT: IntoIterator<Item = &'a N>,
        &'a EdgeIterT: IntoIterator<Item = &'a (N, N, E)>,
        N: 'a,
        E: 'a,
    {
        let mut node_map: HashMap<N, usize> = HashMap::new();
        let mut node_map_rev: Vec<N> = Vec::new();
        let mut node_count = 0;
        for node in nodes {
            if node_map.get(node).is_none() {
                node_map.insert(node.clone(), node_count);
                node_map_rev.push(node.clone());
                node_count += 1;
            }
        }

        let mut nbs: Vec<Vec<(usize, E)>> = vec![vec![]; node_count];
        for edge in edges {
            let node1 = node_map.get(&edge.0).unwrap();
            let node2 = node_map.get(&edge.1).unwrap();
            nbs[*node1].push((*node2, edge.2.clone()));
            nbs[*node2].push((*node1, edge.2.clone()));
        }

        let graph = Self {
            node_map,
            node_map_rev,
            nbs,
            category: 0,
            rooted_tree_infos: None,
        };

        let mut category = GraphCategoryBits::BIDIR_BIT;
        if graph._is_tree() {
            category += GraphCategoryBits::TREE_BIT
                + GraphCategoryBits::CONNECTED_BIT
                + GraphCategoryBits::FOREST_BIT;
        } else if !graph._has_cycle_bidir() {
            category += GraphCategoryBits::FOREST_BIT;
        }

        Self { category, ..graph }
    }

    fn _bfs<InfoT: Clone>(
        &self,
        start: usize,
        start_info: InfoT,
        info_fn: impl Fn(&InfoT, &E) -> InfoT,
    ) -> Vec<Option<InfoT>> {
        let mut infos: Vec<Option<InfoT>> = vec![None; self.node_map_rev.len()];

        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut visited: Vec<bool> = vec![false; self.node_map_rev.len()];

        let add_to_queue = |visited: &mut Vec<bool>,
                            infos: &mut Vec<Option<InfoT>>,
                            queue: &mut VecDeque<usize>,
                            node_id: usize,
                            info: InfoT| {
            visited[node_id] = true;
            infos[node_id] = Some(info);
            queue.push_back(node_id);
        };

        add_to_queue(&mut visited, &mut infos, &mut queue, start, start_info);

        while !queue.is_empty() {
            let node_id = queue.pop_front().unwrap();
            for (nb, e) in &self.nbs[node_id] {
                if !visited[*nb] {
                    let new_info = info_fn(infos[node_id].as_ref().unwrap(), e);
                    add_to_queue(&mut visited, &mut infos, &mut queue, *nb, new_info);
                }
            }
        }

        return infos;
    }

    fn _dfs<'a, InfoT: Clone>(
        &'a self,
        start: usize,
        start_info: &InfoT,
        enter_fn: impl Fn(usize, &Vec<Option<InfoT>>) -> InfoT, // parent_id -> parent_id -> infos -> current_info
        exit_fn: impl Fn(usize, &Vec<Option<InfoT>>) -> InfoT, //  node_id -> parent_id -> infos -> current_info
    ) -> Vec<Option<InfoT>> {
        struct StackFrame<'a, E> {
            node_id: usize,
            parent_id: usize,
            nb_iter: Iter<'a, (usize, E)>,
        }

        let mut infos: Vec<Option<InfoT>> = vec![None; self.node_map_rev.len()];
        let mut stack: Vec<StackFrame<'_, E>> = Vec::new();
        let mut visited: Vec<bool> = vec![false; self.node_map_rev.len()];

        stack.push(StackFrame {
            node_id: start,
            parent_id: start,
            nb_iter: self.nbs[start].iter(),
        });

        while !stack.is_empty() {
            let frame = stack.last_mut().unwrap();
            let node_id = frame.node_id;

            if !visited[frame.node_id] {
                visited[frame.node_id] = true;
                if frame.node_id == start {
                    infos[frame.node_id] = Some(start_info.clone());
                } else {
                    infos[frame.node_id] = Some(enter_fn(frame.parent_id, &infos));
                }
            }

            loop {
                match frame.nb_iter.next() {
                    Some((nb, _)) => {
                        if !visited[*nb] {
                            stack.push(StackFrame {
                                node_id: *nb,
                                parent_id: node_id,
                                nb_iter: self.nbs[*nb].iter(),
                            });
                            break;
                        }
                    }
                    None => {
                        infos[frame.node_id] = Some(exit_fn(frame.node_id, &infos));
                        stack.pop();
                        break;
                    }
                }
            }
        }

        return infos;
    }

    fn _has_cycle(&self) -> bool {
        return false;
    }
    fn _has_cycle_bidir(&self) -> bool {
        return false;
    }

    fn _is_tree(&self) -> bool {
        if self
            .nbs
            .iter()
            .map(|x| x.len())
            .reduce(|x, y| x + y)
            .unwrap()
            != (self.node_map_rev.len() - 1) * 2
        {
            return false;
        }

        let infos = self._bfs::<()>(0, (), |(), _| ());
        return infos.into_iter().filter(|x| x.is_some()).count() == self.node_map_rev.len();
    }

    pub fn is_bidir(&self) -> bool {
        self.category & GraphCategoryBits::BIDIR_BIT > 0
    }

    pub fn is_tree(&self) -> bool {
        self.category & GraphCategoryBits::TREE_BIT > 0
    }

    pub fn is_connected(&self) -> bool {
        self.category & GraphCategoryBits::CONNECTED_BIT > 0
    }

    pub fn is_forest(&self) -> bool {
        self.category & GraphCategoryBits::FOREST_BIT > 0
    }

    pub fn is_dag(&self) -> bool {
        self.category & GraphCategoryBits::DAG_BIT > 0
    }

    fn _assert_categories(&self, bits: Vec<u8>, fn_name: &str) {
        let satisfies = bits
            .iter()
            .map(|bit| self.category & bit > 0)
            .fold(true, |x, y| x && y);
        let bit_names: Vec<String> = bits
            .iter()
            .map(|bit| GraphCategoryBits::name_from_bit(*bit))
            .collect();
        assert!(
            satisfies,
            "The following categories are required to use the method {}: {:#?}",
            fn_name, bit_names
        );
    }

    pub fn compute_rooted_tree(&mut self, root: &N, include_pow2_ancestors: bool) {
        self._assert_categories(
            [GraphCategoryBits::TREE_BIT].to_vec(),
            "compute_rooted_tree",
        );
        let root_id = self.node_map[root];
        let timer = Cell::new(0);
        let start_info = TreeNode {
            parent_id: root_id,
            enter_time: timer.get(),
            level: 0,
            pow2_ancestors: Some(vec![root_id]),
            exit_time: None,
            subtree_upnode_cnt: None,
            subtree_sz: None,
        };
        let enter_fn = |parent_id: usize, infos: &Vec<Option<TreeNode>>| -> TreeNode {
            timer.set(timer.get() + 1);
            let mut pow2_ancestors: Option<Vec<usize>> = None;
            if include_pow2_ancestors {
                let level = infos[parent_id].as_ref().unwrap().level + 1;
                let max_pow =
                    1 + std::mem::size_of_val(&level) * 8 - level.leading_zeros() as usize;
                pow2_ancestors = Some(vec![root_id; max_pow]);
                pow2_ancestors.as_mut().unwrap().reserve(max_pow);
                let mut cur_ancestor = parent_id;
                for i in 0..max_pow {
                    pow2_ancestors.as_mut().unwrap()[i] = cur_ancestor;
                    match infos[cur_ancestor]
                        .as_ref()
                        .unwrap()
                        .pow2_ancestors
                        .as_ref()
                        .unwrap()
                        .get(i)
                    {
                        Some(x) => {
                            cur_ancestor = *x;
                        }
                        None => {
                            break;
                        }
                    }
                }
            }
            TreeNode {
                parent_id,
                enter_time: timer.get(),
                level: infos[parent_id].as_ref().unwrap().level + 1,
                pow2_ancestors,
                exit_time: None,
                subtree_upnode_cnt: None,
                subtree_sz: None,
            }
        };
        let exit_fn = |node_id: usize, infos: &Vec<Option<TreeNode>>| -> TreeNode {
            let mut subtree_upnode_cnt = 0;
            let mut subtree_sz = 1;
            for (nb_id, _) in &self.nbs[node_id] {
                let nb_info = infos[*nb_id].as_ref().unwrap();
                if nb_id == &infos[node_id].as_ref().unwrap().parent_id {
                    continue;
                } else if nb_info.exit_time.is_none() {
                    subtree_upnode_cnt += 1;
                } else if nb_info.level > nb_info.level + 1 {
                    subtree_upnode_cnt -= 1;
                } else {
                    subtree_upnode_cnt += nb_info.subtree_upnode_cnt.unwrap();
                    subtree_sz += nb_info.subtree_sz.unwrap();
                }
            }

            TreeNode {
                exit_time: Some(timer.get()),
                subtree_upnode_cnt: Some(subtree_upnode_cnt),
                subtree_sz: Some(subtree_sz),
                ..infos[node_id].as_ref().unwrap().clone()
            }
        };

        self.rooted_tree_infos = Some(Box::new(
            self._dfs(0, &start_info, enter_fn, exit_fn)
                .into_iter()
                .map(|x| x.unwrap())
                .collect(),
        ));
    }

    fn _is_ancestor(&self, node1_id: usize, node2_id: usize) -> bool {
        self.rooted_tree_infos.as_ref().unwrap()[node1_id].enter_time
            <= self.rooted_tree_infos.as_ref().unwrap()[node2_id].enter_time
            && self.rooted_tree_infos.as_ref().unwrap()[node1_id]
                .exit_time
                .unwrap()
                >= self.rooted_tree_infos.as_ref().unwrap()[node2_id]
                    .exit_time
                    .unwrap()
    }

    fn _common_ancestor_ids(&self, mut node1_id: usize, mut node2_id: usize) -> usize {
        let infos = self.rooted_tree_infos.as_ref().unwrap();

        if infos[node1_id].level > infos[node2_id].level {
            swap(&mut node1_id, &mut node2_id);
        }

        if self._is_ancestor(node1_id, node2_id) {
            return node1_id;
        }

        let mut common_ancestor = node1_id;
        let mut bit = (infos[node1_id].pow2_ancestors.as_ref().unwrap().len() as i32) - 1;
        while bit >= 0 {
            while bit
                >= infos[common_ancestor]
                    .pow2_ancestors
                    .as_ref()
                    .unwrap()
                    .len() as i32
            {
                bit -= 1;
            }
            let next_node = infos[common_ancestor].pow2_ancestors.as_ref().unwrap()[bit as usize];
            if !self._is_ancestor(next_node, node2_id) {
                common_ancestor = next_node
            }
            bit -= 1;
        }

        return infos[common_ancestor].parent_id;
    }

    pub fn common_ancestor(&self, node1: &N, node2: &N) -> &N {
        self._assert_categories([GraphCategoryBits::TREE_BIT].to_vec(), "common_ancestor");
        let node1_id = self.node_map[node1];
        let node2_id = self.node_map[node2];

        &self.node_map_rev[self._common_ancestor_ids(node1_id, node2_id)]
    }

    pub fn get_bridges(&self) -> Vec<(&N, &N)> {
        self._assert_categories([GraphCategoryBits::TREE_BIT].to_vec(), "get_bridges");
        let n = self.node_map_rev.len();
        let infos = self.rooted_tree_infos.as_ref().unwrap();

        (0..n)
            .into_iter()
            .filter(|v| infos[*v].parent_id != *v && infos[*v].subtree_upnode_cnt.unwrap() == 0)
            .map(|v| {
                (
                    &self.node_map_rev[v],
                    &self.node_map_rev[infos[v].parent_id],
                )
            })
            .collect()
    }

    /*
    pub fn get_connected_components() {
        for
    }*/

    pub fn node_iter<'a>(&'a self, node: &'a N) -> impl Iterator<Item = (&N, &E)> + 'a {
        self.nbs[self.node_map[node]]
            .iter()
            .map(|(node_id, distance)| (&self.node_map_rev[*node_id], distance))
    }
}
