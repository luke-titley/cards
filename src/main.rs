
fn make_card(index : usize, number : &u32, pairs : &Vec<(u32, u32)>)
{
    let mut bg_col = "#218d67";
    if index % 2 == 0 {
        bg_col = "#bed0b5";
    }
    print!("{} -> ", number);
    for (a,b) in pairs {
        print!("[{0} {1}] ", a, b);
    }
    println!();

    use svg::Document;
    use svg::node::element;
    use svg::node::Text;

    let back = format!("{}", number);

    let white = element::Rectangle::new()
        .set("width", "100%")
        .set("height", "100%")
        .set("fill", "white");

    let background = element::Rectangle::new()
        .set("width", "37mm")
        .set("height", "59mm")
        .set("x", "2mm")
        .set("y", "2mm")
        .set("rx", "4mm")
        .set("ry", "4mm")
        .set("fill", bg_col);

    let icon_path = format!("cards/icon_{:02}.png", index);
    let img = element::Image::new()
        .set("href", icon_path.as_str())
        .set("x", "2mm")
        .set("y", "2mm");

    let front = element::Text::new()
        .set("text-anchor", "middle")
        .set("dy", ".4em")
        .set("x", "50%")
        .set("y", "50%")
        .set("font-size", 64.0)
        .set("font-weight", "bold")
        .set("font-family", "kalimati")
        .set("fill", "#0e465b")
        .add(Text::new(&back));

    let document = Document::new()
        .set("style", "background: red")
        .set("width", "41mm")
        .set("height", "63mm")
        .add(white)
        .add(background)
        .add(img)
        .add(front);

    let icon = identicon_rs::Identicon::new(back)
        .size(4).unwrap()
        .scale(128).unwrap();
    icon.save_image(&icon_path);

    let file_path = format!("cards/card_{:02}.svg", index);
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
        break;
    }

    println!("length = {}", flat.len());

    // Now build the svg cards
}
