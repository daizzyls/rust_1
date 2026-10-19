fn sum_arr(arr: [u16; 5]) -> u16 {
    let mut sum: u16 = 0;
    for item in arr {
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
        venom = "cm"
    );

    println!("base 10 : {}", 52);
    println!("base 16(hexademical) : {:x}", 52);
    println!("base 2(binary) : {:b}", 52);
    println!("base 8(octal) : {:o}", 52);

    let num: i8 = 5;
    let width: usize = 3; //не дает сделать дургой инт даже Юинт старнно!!!
    println!("{num:<width$}", num = num, width = width);
    println!("{0:<1$}", num, width);

    //#[allow(dead_code)] // disable `dead_code` which warn against unused module
    //struct Structure(i32);
    //let s = Structure(5);
    //println!("{}" ,s); не скомпилируется

    let num: f32 = 52.001;
    let width: usize = 6;
    println!("{num:<width$}");

    println!("{}", sum_arr([1, 2, 3, 4, 5]));
}
