use std::sync::mpsc;
use std::thread;


fn main() {

    // Создание канала
    let (tx, rx) = mpsc::channel();
    // Создание потока
    let handle = thread::spawn(move || {
        // Если сообщение "stop", выходим из цикла
        while let Ok(message) = rx.recv() {
            if message == "stop" {
                break;
            }
            println!("Получено сообщение: {}", message);
        }
        println!("Поток завершен");
    });

    // Отправка сообщений в канал
    tx.send("Hello".to_string()).unwrap();
    tx.send("stop".to_string()).unwrap();

    handle.join().unwrap();
}

