// Задача
// Реализовать бинарный поиск встроенными методами языка.


fn binary_search<T: PartialOrd + Copy + std::fmt::Debug>(arr: &[T], item: T){

    // Инициализируем нижнюю границу (low) и верхнюю границу (high)
    let mut low = 0;
    let mut high = arr.len() - 1;

    // Вычисляем средний индекс
    while low <= high {
        let mid = (low + high) / 2;
        let guess = arr[mid];


         // Если элемент равен искомому значению, выводим его и выходим из функции
        if guess == item {
            println!("Значение {:?} найдено", guess);
            return;
          // Если элемент больше искомого значения, перемещаем верхнюю границу
        } else if guess > item {
            high = mid - 1;
          // Если элемент меньше искомого значения, перемещаем нижнюю границу
        } else {
            low = mid + 1;
        }
    }
      println!("Значение не найдено");
}
fn main() {

    let arr_numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    binary_search(&arr_numbers, 4); // Поиск числа

    // Пример с символами
    let arr_chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    binary_search(&arr_chars, 'd'); // Поиск символа

    // Пример с числами с плавающей точкой
    let arr_floats = [1.1, 2.2, 3.3, 4.4, 5.5];
    binary_search(&arr_floats, 3.3); // Поиск числа с плавающей точкой
    
}


}

