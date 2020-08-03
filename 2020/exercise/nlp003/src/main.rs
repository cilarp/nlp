fn main() {
    let s = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
    let v: Vec<&str> = s.split(' ').collect();
    let v: Vec<&str> = v.iter().map(|s| s.trim_matches(',')).collect();
    let v: Vec<&str> = v.iter().map(|s| s.trim_matches('.')).collect();
    let res: Vec<usize> = v.iter().map(|s| s.len()).collect();
    println!("{:?}", res);
}
