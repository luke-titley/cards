

fn main() {
    let mut pairs = std::collections::HashMap::<u32, Vec<(u32, u32)>>::new();

    let mut add_pair = |i, j| {
        let key = i*j;

        pairs.entry(key).or_insert(Vec::<(u32, u32)>::new()).push((i, j))
    };

    // Build the mapping of results to pairs
    for i in (2..10).into_iter().chain((12..13).into_iter()) {
        for j in (i..10).into_iter().chain((12..13).into_iter()) {
            add_pair(i, j);
        }
    }
    add_pair(11, 11);

    let mut flat : Vec<(u32, Vec<(u32, u32)>)> = pairs.drain().collect();

    flat.sort();

    for (k, p) in flat.iter() {
        print!("{} -> ", k);
        for (a,b) in p {
            print!("[{0} {1}] ", a, b);
        }
        println!();
    }


    println!("length = {}", flat.len());
}
