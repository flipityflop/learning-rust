fn main() {
    let s1 = String::from("string 1");
    let s2 = String::from("string 2");
    let s3 = s1 + &s2;

    println!("s3 is {s3}");

    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");

    let s7 = format!("{s4} {s5} {s6}");

    println!("{s7}");

    for c in s7.chars() {
        println!("{c}");
    }
}
