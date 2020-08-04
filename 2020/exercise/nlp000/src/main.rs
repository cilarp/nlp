fn main() {
    let s = "stressed";
    let res = rev(s);
    println!("{}", res);
}

fn rev(s: &str) -> String {
    s.chars().rev().collect()
}
