Задача
Разработать программу, которая будет последовательно отправлять значения в канал, а с другой стороны канала — читать. 
По истечению N секунд программа должна завершаться.
  

use std::sync::mpsc;
use std::thread;
use std::time::{Duration, };
use rand::{Rng};


fn main() {

    //Время до завершения
    let  end_time = 10;
    println!("Программа будет генерировать случайные числа и остановится через: {} секунд", end_time);

    let mut counter:u32 = end_time;

    // Создание канала
    let (sender, receiver) = mpsc::channel();

    // Создание потока с циклом, который выводит случайное число каждую секунду
    let handle = thread::spawn(move || {
        let mut rng = rand::thread_rng();
        loop {
            let x: i32 = rng.gen_range(0..100);
             if let Err(_) = sender.send(x) {
                 // Если ошибка отправки, завершаем поток
                 break;
             }
            thread::sleep(Duration::from_secs(1))
        }
    });
    
    // Основной поток получается случайные числа
    for res in receiver {

        // Если counter = 0, то программа завершается
        if counter == 0{
            println!("Программа завершена ");
            break;
        }else {
            counter -=1;
        }
        println!("Случайное число: {}, программа завершится через: {} сек", res, counter);
    }

    // Дожидаемся завершения потока
    handle.join().unwrap();
}
