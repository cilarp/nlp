use std::collections::HashMap;

fn main() {
    let s = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
    let v: Vec<&str> = s.split(' ').collect();
    let v: Vec<&str> = v.iter().map(|s| s.trim_matches(',')).collect();
    let v: Vec<&str> = v.iter().map(|s| s.trim_matches('.')).collect();

    let mut res = HashMap::new();
    for i in 0..v.len() {
        let si = v[i].to_string();
        match i {
            0 | 4 | 5 | 6 | 7 | 8 | 14 | 15 | 18 => {
                res.insert(si[..1].to_owned(), i + 1);
            }
            _ => {
                res.insert(si[..2].to_owned(), i + 1);
            }
        }
    }
    println!("{:?}", res);
}
