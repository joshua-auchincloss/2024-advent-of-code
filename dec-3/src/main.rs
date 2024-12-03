fn find_all_mul(src: String) -> i32 {
    let re = regex::Regex::new("mul\\(([0-9]*),([0-9]*)\\)").unwrap();

    let mut sum = 0_i32;
    for the in re.find_iter(&src) {
        let start = &src[the.start()..the.end()];
        let start = start.trim_start_matches("mul(");
        let start = start.trim_end_matches(")");
        let mut the = start.split(",");
        let first: i32 = the.next().unwrap().parse().unwrap();
        let second: i32 = the.next().unwrap().parse().unwrap();

        sum += first * second;
    }

    sum
}

#[derive(Clone, PartialEq)]
enum Op {
    Do,
    Dont,
}

fn find_all_mul_with_do_dont(src: String) -> i32 {
    let re = regex::Regex::new("mul\\(([0-9]*),([0-9]*)\\)").unwrap();
    let dos = regex::Regex::new("do\\(\\)").unwrap();
    let donts = regex::Regex::new("don't\\(\\)").unwrap();

    let mut ops: Vec<_> = dos
        .find_iter(&src)
        .map(|the| the.start())
        .map(|it| (it, Op::Do))
        .collect();

    ops.append(
        &mut donts
            .find_iter(&src)
            .map(|the| the.start())
            .map(|it| (it, Op::Dont))
            .collect(),
    );

    ops.sort_by(|(idx, _), (idx2, _)| idx.cmp(idx2));

    let mut sum = 0_i32;
    let mut last_op = Op::Do;
    for the in re.find_iter(&src) {
        let found = ops
            .iter()
            .enumerate()
            .find(|(_, (idx, _))| *idx < the.start());

        if let Some((pos, (_, op))) = found {
            last_op = op.clone();
            ops.remove(pos);
        }

        if last_op == Op::Dont {
            continue;
        }

        let start = &src[the.start()..the.end()];
        let start = start.trim_start_matches("mul(");
        let start = start.trim_end_matches(")");

        let mut the = start.split(",");
        let first: i32 = the.next().unwrap().parse().unwrap();
        let second: i32 = the.next().unwrap().parse().unwrap();

        sum += first * second;
    }

    sum
}

fn main() {
    let args = common::args();
    let buf = args.file();
    let with_do = args.boolean_flag("--ops");
    let sum = if with_do {
        find_all_mul_with_do_dont(buf)
    } else {
        find_all_mul(buf)
    };
    println!("sum: {sum}")
}
