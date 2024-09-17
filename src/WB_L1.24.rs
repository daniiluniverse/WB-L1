// Задача
// Разработать программу, которая проверяет, что все символы в строке уникальные (true — если уникальные, false etc). Функция проверки должна быть регистронезависимой.
// Например:
// abcd — true abCdefAaf — false aabcd — false

use std::collections::HashSet;

fn uniq_char(s: &str){
    // Хешсет для хранения символов
    let mut hs = HashSet::new();
    // Приводим слово в нижнему регистру
    let s = s.to_ascii_lowercase();
    // добавляем символы  в хешсет
    for i in s.chars() {
        hs.insert(i);
    }
    // Сравниваем длину хэшсета и строки, если есть не уникальные значения, то длина хешсета будет меньше
    if hs.len() < s.len() {
        println!("{}", "false");
    } else {
        println!("{} ", "true");
    }
}

fn main() {
    let x = "aaaaabcd";
    uniq_char(x);
}
