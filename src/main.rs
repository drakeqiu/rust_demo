fn main() {
    let mut s1 = String::from("hello");
    calculate_length(&mut s1)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
