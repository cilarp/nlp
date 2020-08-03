fn main() {
    let s = "パタトクカシーー";
    let res: String = s.chars().step_by(2).collect();
    println!("{}", res);
}
