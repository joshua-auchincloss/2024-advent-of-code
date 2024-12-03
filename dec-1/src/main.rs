use std::collections::HashMap;

fn nth_even(src: Vec<i32>, even: bool) -> Vec<i32> {
    src.iter()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == if even { 0 } else { 1 })
        .map(|(_, the)| *the)
        .collect()
}

fn parse_lists(buf: String) -> (Vec<i32>, Vec<i32>) {
    let parsed: Vec<_> = buf
        .replace("\n", " ")
        .split(" ")
        .filter(|the| !the.is_empty())
        .map(|this| this.parse::<i32>().unwrap())
        .collect();

    let (mut first, mut second) = (
        nth_even(parsed.clone(), true),
        nth_even(parsed.clone(), false),
    );

    first.sort();
    second.sort();

    (first, second)
}

fn pairwise_dist(a: Vec<i32>, b: Vec<i32>) -> Vec<u32> {
    let mut dists = vec![];

    for (i, a) in a.into_iter().enumerate() {
        let b = b.get(i).expect("equidistant sources");
        dists.push(a.abs_diff(*b))
    }

    dists
}

fn frequencies(src: Vec<i32>) -> HashMap<i32, usize> {
    let mut freq_ct: HashMap<i32, usize> = HashMap::default();
    for it in src {
        if let Some(ct) = freq_ct.get_mut(&it) {
            *ct += 1;
        } else {
            freq_ct.insert(it, 1);
        }
    }
    freq_ct
}

fn freq_diff(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let fb = frequencies(b);

    let mut ft = 0;

    for v in a {
        let fb = fb.get(&v);
        if let Some(fb) = fb {
            ft += (*fb as i32) * v
        } else {
            continue;
        }
    }

    ft
}

fn main() {
    let args = common::args();
    let data = args.file();

    let (left, right) = parse_lists(data);

    let total: u32 = pairwise_dist(left.clone(), right.clone()).iter().sum();
    println!("total distance: {total}");

    let freq_t = freq_diff(left, right);
    println!("distance by frequency: {freq_t}");
}
