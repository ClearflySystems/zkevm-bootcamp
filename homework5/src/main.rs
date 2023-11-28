fn main() {
    println!("Hello, world!");

    fizz_buzz(301);
}


fn fizz_buzz(num: u32){
    let mut count = 0;
    for i in 0..num {
        if i % 3 == 0 && i % 5 == 0 {
            println!("fizz buzz");
            count+=1;
        } else if i % 5 == 0 {
            println!("buzz");
        } else if i % 3 == 0 {
            println!("fizz");
        }
    }
    println!("fizz buzz occured {} times", count);
}