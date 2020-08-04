use itertools::interleave;

fn main() {
    let s1 = "パトカー";
    let s2 = "タクシー";
    let res: String = combine(s1, s2);
    println!("{}", res);
}

fn combine(s1: &str, s2: &str) -> String {
    interleave(s1.chars(), s2.chars()).collect()
}
