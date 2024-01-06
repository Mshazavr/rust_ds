use core::hash::Hash;
use std::cell::Cell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::slice::Iter;

#[derive(Clone, Debug)]
pub struct TreeNode {
    parent_id: usize,
    enter_time: usize,
    level: usize,
    exit_time: Option<usize>,
    up_node_cnt: Option<usize>,
    down_node_cnt: Option<usize>,
    subtree_upnode_cnt: Option<usize>,
    subtree_sz: Option<usize>,
}
pub struct Graph<N, E> {
    pub node_map: HashMap<N, usize>,
    node_map_rev: Vec<N>,
    nbs: Vec<Vec<(usize, E)>>,
    bidir: bool,
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

        Self {
            node_map,
            node_map_rev,
            nbs,
            bidir: false,
            rooted_tree_infos: None,
        }
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

        Self {
            node_map,
            node_map_rev,
            nbs,
            bidir: true,
            rooted_tree_infos: None,
        }
    }

    pub fn node_iter<'a>(&'a self, node: &'a N) -> impl Iterator<Item = (&N, &E)> + 'a {
        self.nbs[self.node_map[node]]
            .iter()
            .map(|(node_id, distance)| (&self.node_map_rev[*node_id], distance))
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
        enter_fn: impl Fn(usize, &InfoT) -> InfoT, // parent_id -> &parent_info -> current_info
        exit_fn: impl Fn(Vec<(&InfoT, &E)>, &InfoT) -> InfoT, // (&nb_info, &e) -> &current_info -> final_current_info
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

            if !visited[node_id] {
                visited[node_id] = true;

                println!("Enter: {}", node_id);

                if node_id == start {
                    infos[node_id] = Some(start_info.clone());
                }
                else {
                    infos[node_id] = Some(enter_fn(
                        frame.parent_id,
                        infos[frame.parent_id].as_ref().unwrap(),
                    ));
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
                        infos[node_id] = Some(exit_fn(
                            self.nbs[frame.node_id]
                                .iter()
                                .filter(|(nb, _)| nb != &frame.parent_id)
                                .map(|(nb, e)| (infos[*nb].as_ref().unwrap(), e))
                                .collect(),
                            infos[node_id].as_ref().unwrap(),
                        ));

                        
                        println!("Exit: {}", node_id);
                        stack.pop();
                        break;
                    }
                }
            }
        }

        return infos;
    }

    pub fn is_tree(&self) -> bool {
        if !self.bidir {
            return false;
        }

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

    pub fn compute_rooted_tree(&mut self) {
        let timer = Cell::new(0);
        let start_info = TreeNode {
            parent_id: 0,
            enter_time: timer.get(),
            level: 0,
            exit_time: None,
            up_node_cnt: None,
            down_node_cnt: None,
            subtree_upnode_cnt: None,
            subtree_sz: None,
        };
        let enter_fn = |parent_id: usize, parent: &TreeNode| -> TreeNode {
            timer.set(timer.get() + 1);
            TreeNode {
                parent_id,
                enter_time: timer.get(),
                level: parent.level + 1,
                exit_time: None,
                up_node_cnt: None,
                down_node_cnt: None,
                subtree_upnode_cnt: None,
                subtree_sz: None,
            }
        };
        let exit_fn = |nbs: Vec<(&TreeNode, &E)>, node: &TreeNode| -> TreeNode {
            let mut up_node_cnt = 0;
            let mut down_node_cnt = 0;
            let mut subtree_upnode_cnt = 0;
            let mut subtree_sz = 1;
            for (nb, _) in nbs {
                if nb.exit_time.is_none() {
                    up_node_cnt += 1;
                    subtree_upnode_cnt += 1;
                } else if nb.level > node.level + 1 {
                    down_node_cnt += 1;
                    subtree_upnode_cnt -= 1;
                } else {
                    subtree_upnode_cnt += nb.subtree_upnode_cnt.unwrap();
                    subtree_sz += nb.subtree_sz.unwrap();
                }
            }

            TreeNode {
                exit_time: Some(timer.get()),
                up_node_cnt: Some(up_node_cnt),
                down_node_cnt: Some(down_node_cnt),
                subtree_upnode_cnt: Some(subtree_upnode_cnt),
                subtree_sz: Some(subtree_sz),
                ..node.clone()
            }
        };

        self.rooted_tree_infos = Some(Box::new(
            self._dfs(0, &start_info ,enter_fn, exit_fn)
                .into_iter()
                .map(|x| x.unwrap())
                .collect(),
        ));
    }
}
