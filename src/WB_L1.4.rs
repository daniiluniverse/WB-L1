use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::{io, thread};
use std::time::Duration;

fn main() {
    // Определяем количество воркеров
    let mut input = String::new();
    println!("Введите число воркеров:");

    io::stdin().read_line(&mut input).expect("Не удалось прочитать строку");
    let num_workers: usize = match input.trim().parse() {
        Ok(num) if num > 0 => num,
        _ => {
            eprintln!("Недопустимое количество воркеров, введите целое положительное число");
            std::process::exit(1);
        }
    };

    // Создаем канал
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    // Создаем и запускаем воркеры
    let mut handles = vec![];
    for i in 0..num_workers {
        let rx = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            loop {
                let message = rx.lock().unwrap().recv();
                match message {
                    Ok(data) => println!("Воркер {} выводит: {}", i, data),
                    Err(_) => {
                        // Канал закрыт
                        println!("Воркер {}: выходит, канал закрыт", i);
                        break;
                    }
                }
            }
        });
        handles.push(handle);
    }

    // Главный поток: записываем данные в канал
    for i in 0..15 {
        tx.send(format!("число {}", i)).unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    // Закрываем канал
    drop(tx);

    // Ждем завершения всех воркеров
    for handle in handles {
        handle.join().unwrap();
    }
}
