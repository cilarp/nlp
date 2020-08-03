use itertools::interleave;

fn main() {
    let s1 = "パトカー";
    let s2 = "タクシー";
    let res: String = interleave(s1.chars(), s2.chars()).collect();
    println!("{}", res);
}
