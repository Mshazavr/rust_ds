use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use rust_ds::SegmentTree;
use rust_ds::SegmentTreeBmp;
use rust_ds::SumNode;
use std::time::Duration;
use std::time::Instant;

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

struct TestInput {
    arr: Vec<i64>,
    operations: Vec<Operation>,
}

fn gen_test_input(seed: u64, n: usize, m: usize) -> TestInput {
    let range_min = -1000;
    let range_max = 1000;
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);

    fn gen_arr_elem(rng: &mut StdRng, range_min: i64, range_max: i64) -> i64 {
        return rng.gen_range(range_min..=range_max);
    }

    fn gen_operation(rng: &mut StdRng, n: usize, range_min: i64, range_max: i64) -> Operation {
        let left: usize = rng.gen_range(0..n) as usize;
        let right: usize = rng.gen_range(left..n) as usize;
        let tp = rng.gen_range(0..=1);

        if tp == 0 {
            Operation::Update {
                left,
                right,
                delta: rng.gen_range(range_min..=range_max),
            }
        } else {
            Operation::Query { left, right }
        }
    }

    TestInput {
        arr: (0..n)
            .map(|_| gen_arr_elem(&mut rng, range_min, range_max))
            .collect(),
        operations: (0..m)
            .map(|_| gen_operation(&mut rng, n, range_min, range_max))
            .collect(),
    }
}

fn process_input(input: &TestInput) {
    let mut st: SegmentTree<SumNode> = SegmentTree::new(&input.arr);
    for operation in &input.operations {
        match operation {
            Operation::Update { left, right, delta } => {
                st.update(*left, *right, &delta);
            }
            Operation::Query { left, right } => {
                let _ = st.query(*left, *right);
            }
        }
    }
}

fn process_input_bump(input: &TestInput) {
    let mut st: SegmentTreeBmp<SumNode> = SegmentTreeBmp::new(&input.arr);
    for operation in &input.operations {
        match operation {
            Operation::Update { left, right, delta } => {
                st.update(*left, *right, &delta);
            }
            Operation::Query { left, right } => {
                let _ = st.query(*left, *right);
            }
        }
    }
}

#[test]
fn segment_tree_regular() {
    let n = 100000;
    let m = 100000;
    let num_iterations: usize = 1000;

    let mut elapsed_times: Vec<Duration> = Vec::new();
    elapsed_times.reserve((num_iterations+1).try_into().unwrap());

    for seed in 0..num_iterations {
        let input = gen_test_input(seed.try_into().unwrap(), n, m);

        let timer = Instant::now();
        process_input(&input);
        let elapsed = timer.elapsed();
        elapsed_times.push(elapsed);
    }
    elapsed_times.sort();

    println!(
        "Median Elapsed Regular: {:.2?}",
        elapsed_times[(num_iterations/2) as usize],
    );
}

#[test]
fn segment_tree_bump() {
    let n = 100000;
    let m = 100000;
    let num_iterations: usize = 1000;

    let mut elapsed_times: Vec<Duration> = Vec::new();
    elapsed_times.reserve((num_iterations+1).try_into().unwrap());

    for seed in 0..num_iterations {
        let input = gen_test_input(seed.try_into().unwrap(), n, m);

        let timer = Instant::now();
        process_input_bump(&input);
        let elapsed = timer.elapsed();
        elapsed_times.push(elapsed);
    }
    elapsed_times.sort();

    println!(
        "Median Elapsed Bumped: {:.2?}",
        elapsed_times[(num_iterations/2) as usize]
    );
}
