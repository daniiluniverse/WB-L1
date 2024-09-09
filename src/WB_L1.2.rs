// Задача

// Написать программу, которая параллельно рассчитает квадраты чисел, взятых из массива (массив инициализировать 1..N), и выведет их в stdout.
// Числа могут быть выведены в произвольном порядке.
// Использовать только стандартную библиотеку.

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn main() {
    // Размер массива
    let n = 10;

    // Инициализация массива
    let numbers : Vec<u32> = (1..=n).collect();


    // Создание канала для передачи данных между потоками
    let (tx, rx) = mpsc::channel();
    let tx = Arc::new(Mutex::new(tx));


    // Создаем вектор для хранения дескрипторов потоков
    let mut handles:Vec<_> = vec![];

    for &num in &numbers{
        // Клонируем Arc для использования в потоке
        let tx = Arc::clone(&tx);
        let handle = thread::spawn(move || {
            let square = num * num;

            tx.lock().unwrap().send(square).unwrap();
        });
        handles.push(handle)
    }

    // Ждем завершения всех потоков
    for handle in handles{
        handle.join().unwrap();
    }


    // Получаем и выводим результаты
    for _ in 0..n{
        let result = rx.recv().unwrap();
        println!("{}", result)
    }

}
