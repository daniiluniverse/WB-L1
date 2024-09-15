// Задача
// Разработать программу, которая переворачивает слова в строке.

// Пример: «snow dog sun — sun dog snow»..


fn reverse_word(str: &str)  {

    // Преобразуем строку в вектор символов
    let mut v: Vec<&str> = str.split_whitespace().collect();
     // Переворачиваем вектор
    v.reverse();

    // Собираем перевернутое предложение обратно 
    for i in v{
        print!("{} ", i)
    }
}

fn main() {

    reverse_word("snow dog sun");

}
