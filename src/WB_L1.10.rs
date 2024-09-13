
// Задача

// Разработать конвейер чисел с помощью каналов. В первый канал с паузами пишутся числа из массива, проинициализированного 1..N. 
// Первый thread или task читает из этого канала и пишет квадрат полученного числа во второй канал. 
// Второй thread или task читает из второго канала и выводит в stdout.


use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let n = 10; // Задаем значение N
    let array: Vec<i32> = (1..=n).collect();


    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
    for number in array{
        tx1.send(number).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

    let thread1 = thread::spawn(move || {
        for number in rx1{
            let square = number * number;
            tx2.send(square).unwrap()
        }
        drop(tx2);
    });

    let mut count = 0;

    let thread2 = thread::spawn(move || {
        for square in rx2 {
            count +=1;
            println!("Квадрат числа {} равен : {}", count,  square);
        }
    });

        thread1.join().unwrap();
        thread2.join().unwrap();
