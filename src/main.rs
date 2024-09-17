use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Создание каналов
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    // Обертка для блокировки
    let rx1 = Arc::new(Mutex::new(rx1));
    let rx2 = Arc::new(Mutex::new(rx2));

    // Поток для генерации чисел с паузами
    let tx1_clone = tx1.clone();
    thread::spawn(move || {
        for i in 1..=10 {
            tx1_clone.send(i).unwrap();
            thread::sleep(Duration::from_millis(500)); // Пауза 500 мс
        }
    });

    // Поток для вычисления квадратов чисел
    let tx2_clone = tx2.clone();
    let rx1_clone = Arc::clone(&rx1);
    thread::spawn(move || {
        while let Ok(num) = rx1_clone.lock().unwrap().recv() {
            let square = num * num;
            tx2_clone.send(square).unwrap();
        }
    });

    // Поток для вывода квадратов чисел
    let rx2_clone = Arc::clone(&rx2);
    thread::spawn(move || {
        while let Ok(square) = rx2_clone.lock().unwrap().recv() {
            println!("Квадрат числа: {}", square);
        }
    });

    // Основной поток ожидает завершения работы всех потоков
    // Поскольку у нас нет механизма для ожидания завершения потоков,
    // основной поток просто будет спать, чтобы дать время другим потокам завершиться.
    thread::sleep(Duration::from_secs(10));
}
