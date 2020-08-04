fn main() {
    let s = "パタトクカシーー";
    let res: String = odd_ch(s);
    println!("{}", res);
}

fn odd_ch(s: &str) -> String {
    s.chars().step_by(2).collect()
}
