use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Создание двух каналов
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    // Запуск потока для генерации чисел
    let producer_thread = thread::spawn(move || {
        for i in 1..=10 { // Инициализируем числа от 1 до 10
            tx1.send(i).expect("Failed to send data");
            thread::sleep(Duration::from_millis(500)); // Добавляем паузу
        }
    });

    // Запуск потока для обработки чисел (возведение в квадрат)
    let processor_thread = thread::spawn(move || {
        for num in rx1 {
            let squared = num * num;
            tx2.send(squared).expect("Failed to send data");
        }
    });

    // Запуск потока для вывода результатов
    let consumer_thread = thread::spawn(move || {
        for squared in rx2 {
            println!("{}", squared);
        }
    });

    // Ожидание завершения всех потоков
    producer_thread.join().expect("Producer thread panicked");
    processor_thread.join().expect("Processor thread panicked");
    consumer_thread.join().expect("Consumer thread panicked");
}
