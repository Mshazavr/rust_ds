//use crate::segment_tree::SegmentTreeBmp as SegmentTree;
use rust_ds::SegmentTree;
use rust_ds::SumNode;

enum Operation {
    Update {
        left: usize,
        right: usize,
        delta: i64,
    },
    Query {
        left: usize,
        right: usize,
    },
}

#[test]
fn segment_tree() {
    let arr: Vec<i64> = vec![4, 5, 2, 1, 0, 13, 2, 4, 4];
    let operations: Vec<(Operation, Option<i64>)> = vec![
        (Operation::Query { left: 0, right: 4 }, Some(12)),
        (Operation::Query { left: 1, right: 3 }, Some(8)),
        (
            Operation::Update {
                left: 1,
                right: 3,
                delta: 3,
            },
            None,
        ),
        (Operation::Query { left: 0, right: 4 }, Some(21)),
        (Operation::Query { left: 1, right: 3 }, Some(17)),
    ];

    let mut st: SegmentTree<SumNode> = SegmentTree::new(&arr);

    for operation in operations {
        match operation.0 {
            Operation::Update { left, right, delta } => {
                st.update(left, right, &delta);
            }
            Operation::Query { left, right } => {
                let query_res = st.query(left, right);
                assert_eq!(query_res, operation.1.unwrap());
            }
        }
    }
}
