use std::{io, process::exit};
mod step1_calculatetick;
use step1_calculatetick::calculate_tick;
mod step2_calculateAmount;
use step2_calculateAmount::generate_token;

fn main() {
    loop {
        println!("输入需求功能（q键退出）: ");

        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).expect("读取有误");

        let condition = user_input.trim();
        match condition {
            "1" => calculate_tick(),
            "2" => generate_token(),
            "q" => {
                println!("退出程序。");
                exit(0); // 退出循环
            }
            "" => println!("请输入命令。"),          // 处理空输入
            _ => println!("未知命令，请重新输入。"), // 处理未知输入
        }
    }
}
