use std::collections::HashMap;

fn main() {
    let s = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
    let res = atom(s);
    println!("{:?}", res);
}

fn atom(s: &str) -> HashMap<String, usize> {
    let mut res = HashMap::new();
    s.replace(".", "")
        .split_whitespace()
        .enumerate()
        .for_each(|(i, si)| {
            let t = match i {
                0 | 4 | 5 | 6 | 7 | 8 | 14 | 15 | 18 => si[..1].to_owned(),
                _ => si[0..2].to_owned(),
            };
            res.insert(t, i + 1);
        });
    res
}
