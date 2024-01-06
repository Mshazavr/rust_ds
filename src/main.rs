pub mod graph;
pub mod segment_tree;

use crate::graph::Graph;

fn main() {
    let nodes: [String; 6] = ["AAA", "BBB", "CCC", "DDD", "EEE", "FFF"].map(|s| s.to_string());
    let edges: [(String, String, i32); 5] = [
        ("AAA", "BBB", 1),
        ("AAA", "CCC", 2),
        ("CCC", "DDD", 2),
        ("CCC", "EEE", 2),
        ("CCC", "FFF", 2),
        //("AAA1", "BBB1", 1),
        //("AAA1", "CCC1", 2)
    ]
    .map(|(v, w, e)| (v.to_string(), w.to_string(), e));

    let mut g: Graph<String, i32> = Graph::new_bidir(&nodes, &edges);
    println!("The neighbours of AAA:");
    for nb in g.node_iter(&"AAA".to_string()) {
        println!("{:?}", nb);
    }

    println!("{}", g.is_tree());

    g.compute_rooted_tree();
    for node in g.rooted_tree_infos.unwrap().iter() {
        println!("{:?}", node);
    }
}
