// Задача
// Реализовать структуру-счетчик, которая будет инкрементироваться в конкурентной среде. 
// По завершению программа должна выводить итоговое значение счетчика.


use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

struct Counter{
    count: Arc<AtomicUsize>
}

impl Counter{

    // Конструктор
    fn new() -> Self{
        Counter{
            count: Arc::new(AtomicUsize::new(0))
        }
    }

     // Метод для инкрементации
    fn increment(&self){
        self.count.fetch_add(1, Ordering::SeqCst);
    }

     // Метод для получения текущего значения счетчика
    fn get_count(&self) -> usize{
        self.count.load(Ordering::SeqCst)
    }
}

fn main() {

     // Создаем экземпляр структуры
    let count = Arc::new(Counter::new());

    let mut handles = vec![];

    // Запускаем несколько потоков
    for _ in 0..10{

        let count = Arc::clone(&count);

        let handle = thread::spawn(move || {
            for _ in 0..100{
                // Инкрементируем счетчик
                count.increment();
            }
        });

        handles.push(handle);
    }

    // Ждем завершения всех потоков
    for handle in handles{
        handle.join().unwrap();
    }


    println!("Итоговое значение счетчика: {}", count.get_count())



}
