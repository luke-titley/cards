
fn make_card(index : usize, number : &u32, pairs : &Vec<(u32, u32)>)
{
    if index % 2 == 0 {
        print!("*");
    }
    print!("{} -> ", number);
    for (a,b) in pairs {
        print!("[{0} {1}] ", a, b);
    }
    println!();

    use svg::Document;
    use svg::node::element::Path;
    use svg::node::element::path::Data;

    let data = Data::new()
        .move_to((10, 10))
        .line_by((0, 50))
        .line_by((50, 0))
        .line_by((0, -50))
        .close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", data);

    let document = Document::new()
        .set("viewBox", (0, 0, 70, 70))
        .add(path);

    let file_path = format!("card_{:02}.svg", index);
    svg::save(&file_path, &document).unwrap();
}

fn main() {
    // Build the pair groupings
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

    for (i, (k, p)) in flat.iter().enumerate() {
        make_card(i, k, &p);
    }

    println!("length = {}", flat.len());

    // Now build the svg cards
}
