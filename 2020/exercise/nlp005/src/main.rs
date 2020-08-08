fn main() {
    let s = "I am an NLPer";
    println!("{:?}", n_gram_chars(s, 2));
    println!("{:?}", n_gram_words(s, 2));
}

fn n_gram_chars(s: &str, n: usize) -> Vec<String> {
    let mut res = Vec::new();
    for i in 0..=s.len() - n {
        res.push(s.get(i..i + n).unwrap().to_string());
    }
    res
}

fn n_gram_words(s: &str, n: usize) -> Vec<Vec<String>> {
    let mut res = Vec::new();
    let v: Vec<&str> = s.split_whitespace().collect();
    let v: Vec<String> = v.iter().map(|s| s.to_string()).collect();
    for i in 0..=v.len() - n {
        res.push(v.get(i..i + n).unwrap().to_owned());
    }
    res
}
