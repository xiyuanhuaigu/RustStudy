// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

// fn main() {
//     println!("猜数字游戏");

//     let secret_number = rand::thread_rng().gen_range(1, 100);

//     loop {
//         println!("猜测一个数");
//         let mut guess: String = String::new();
//         io::stdin().read_line(&mut guess).expect("无法读取行");

//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         println!("你猜测的数是：{}", guess);

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//     }
// }
// fn main(){
//     let tup:(i32,f64,u8) = (500,6.4,1);
//     let (x,y,z) = tup;
//     println!("{},{},{}",x,y,z);

// }
// fn main(){
//     println!("hello world!");
//     another_function(5); 
// }
// fn another_function(x:i32){
//     println!("hello! {x}");
// }
fn five() -> i32{
    5
}
fn main(){
    let x = five();

    println!("is {x}");
}