fn sum_arr(arr : [u16 ; 5]) -> u16 {
    let mut sum : u16 = 0;
    for item in arr  {
        sum += item;
    }
    
    sum
}

fn main() {
    let x = 5 + /* 90 */ 5;
    println!("Is `x` 10 or 100 ? x = {}", x);

    println!("{} days ago...", 31);

    println!("{0} metted {1} about 6-7 hours ago...", "Bob", "Jessy");

    println!(
        "{sub} {bebra}{venom}",
        sub = "KIKIMARA",
        bebra = 52,
        venom = "cm");

    println!("{}" , sum_arr([1 , 2 ,3 ,4 ,5]));
}
