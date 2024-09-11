Задача

Программа должна аккуратно завершаться по нажатию Ctrl+C. Выбрать и обосновать способ завершения работы всех воркеров.
Использовать tokio и flume (или другую аналогичную библиотеку для spmc/mpmc-каналов).



use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::{io, thread};
use std::time::Duration;
use tokio::{signal, task};

#[tokio::main]
async fn main() {
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
        let handle = thread::spawn( move || {
            loop {
                let message = rx.lock().unwrap().recv();
                match message {
                    Ok(data) => println!("Воркер {} выводит: {}", i, data),
                    Err(_) => {
                        // Канал закрыт
                        println!("Канал воркер {}: закрыт", i);
                        break;
                    }
                }
            }
        });
        handles.push(handle);
    }

    // Главный поток: записываем данные в канал
    let tx_clone = tx.clone();
    task::spawn(async move {
        for i in 0..10{
            if let Err(e) = tx_clone.send(format!("число {}", i)) {
                eprintln!("Ошибка при отправке сообщения: {}", e);
                break;
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });



     // Обрабатываем сигнал Ctrl+C
    signal::ctrl_c().await.expect("Не удалось установить обработчик сигнала");


    println!("Получен сигнал завершения, ждем завершения всех воркеров...");
    tokio::time::sleep(Duration::from_secs(1)).await;

     drop(tx);

    // Ждем завершения всех воркеров
    for handle in handles {
        handle.join().expect("Воркер завершился с ошибкой");
    }

    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Все воркеры завершены.");


}
