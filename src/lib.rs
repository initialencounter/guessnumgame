use std::cmp::Ordering;
use std::io;
use rand::Rng;

pub fn start() {
    println!("欢迎来到猜数字小游戏！\n");
    println!("请输入一个整数数字 1-100：输入q退出");
    let secret_number = rand::thread_rng().gen_range(1..101);
    guess_num(secret_number);
    println!("按任意键退出");
    let mut tmp = String::new();
    let _ = io::stdin().read_line(&mut tmp);
}
pub fn guess_num(secret_number: i32) {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    if guess.trim() == "q" {
        return;
    }

    let back_int = guess.trim().parse::<i32>().expect("非法输入，请输入一个整数！");

    match back_int.cmp(&secret_number) {
        Ordering::Less => {
            println!("猜小了!，再试一次吧！");
            return guess_num(secret_number);
        }
        Ordering::Greater => {
            println!("猜大了，再试一次吧！");
            return guess_num(secret_number);
        }
        Ordering::Equal => {
            println!("你赢了！");
        }
    }
}