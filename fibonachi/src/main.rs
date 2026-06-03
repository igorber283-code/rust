use std::io;

fn input() -> u32 {
    let mut input_string = String::new();

    io::stdin()
        .read_line(&mut input_string)
        .expect("Не удалось прочитать строку");

    input_string.trim().parse().expect("Пожалуйста, введите число")
}

fn fibonachi() {
    let input = input();
    if input == 0 {
        println!("fibonachi: 0");
        return; 
    }

    let mut x: u32 = 0;
    let mut y: u32 = 1;
    
    for _ in 2..=input {
        (x, y) = (y, x + y);
    }
    
    println!("fibonachi: {}", y);
}

fn main() {
    println!("Введите порядковый номер числа Фибоначчи:");
    fibonachi();
}
