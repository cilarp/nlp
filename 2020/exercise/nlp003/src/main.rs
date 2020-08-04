fn main() {
    let s = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
    let res: Vec<usize> = pi(s);
    println!("{:?}", res);
}

fn pi(s: &str) -> Vec<usize> {
    s.replace(",", " ")
        .replace(".", "")
        .split_whitespace()
        .map(|s| s.len())
        .collect()
}
