use tokio::time::{sleep, Duration};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let cancellation_token = CancellationToken::new();

     // Клонируем токен для первой задачи
    let token1 = cancellation_token.clone();
    let task1 = tokio::spawn(async move {
        loop {
            tokio::select! {
                // Ожидает отмены через токен
                _ = token1.cancelled() => {
                    println!("Таск 1 закрыт");
                    break; // Прерываем цикл при получении сигнала отмены
                }
               // Ожидание одной секунды, чтобы не блокировать выполнение
                _ = sleep(Duration::from_secs(1)) => {
                    println!("Таск 1 работает");
                }
            }
        }
    });

     // Клонируем токен для второй задачи
    let token2 = cancellation_token.clone();
    let task2 = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = token2.cancelled() => {
                    println!("Таск 2 закрыт");
                    break;
                }
                _ = sleep(Duration::from_secs(1)) => {
                    println!("Таск 2 работает");
                }
            }
        }
    });

    // Ждем 3 секунды перед отменой задач
    sleep(Duration::from_secs(3)).await;
    cancellation_token.cancel(); // Отправляем сигнал отмены всем задачам

    // Ждем завершения задач
    task1.await.unwrap();
    task2.await.unwrap();
}
