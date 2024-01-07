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
        //("EEE", "AAA", 2),
    ]
    .map(|(v, w, e)| (v.to_string(), w.to_string(), e));

    let mut g: Graph<String, i32> = Graph::new_bidir(&nodes, &edges);
    println!("The neighbours of AAA:");
    for nb in g.node_iter(&"AAA".to_string()) {
        println!("{:?}", nb);
    }

    println!("{}", g.is_tree());

    g.compute_rooted_tree(&"AAA".to_string(), true);

    println!(
        "CCC & BBB ca: {}",
        g.common_ancestor(&"CCC".to_string(), &"BBB".to_string())
    );
    println!(
        "DDD & FFF ca: {}",
        g.common_ancestor(&"DDD".to_string(), &"FFF".to_string())
    );
    println!(
        "EEE & BBB ca: {}",
        g.common_ancestor(&"EEE".to_string(), &"BBB".to_string())
    );

    println!("{:#?}", g.get_bridges());
}
