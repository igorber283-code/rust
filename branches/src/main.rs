use std::io;

fn input() -> f64 {
    let mut input_string = String::new();

    io::stdin()
        .read_line(&mut input_string)
        .expect("Не удалось прочитать строку");

    input_string.trim().parse().expect("Ошибка: введено не число!")
}

fn main() {
    println!("Введите температуру в Цельсиях: ");
    
    let temperature = input(); 
    
    let result = temperature * 1.8 + 32.0;
    
    println!("Текущая температура: {:.1} градусов по Фаренгейту", result);
}