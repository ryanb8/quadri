pub fn ryan_test() -> () {
    let num : usize = 265;
    let rem : usize = num % 26;
    let div : usize = num / 26;

    println!("Number: {:?}", num);
    println!("Remainder: {:?}", rem);
    println!("Division Result: {:?}", div);
}