// Задача

// Дана последовательность температурных колебаний (для примера: -25.4, -27.0 13.0, 19.0, 15.5, 24.5, -21.0, 32.5). 
// Объединить данные значения в интервалы с шагом в 10 градусов. Последовательность в подмножноствах не важна.

// Пример: [-30,-20):{-25.0, -27.0, -21.0}, [10, 20):{13.0, 19.0, 15.5}, [20, 30): {24.5}, etc.

use std::collections::HashMap;

fn main() {
    // Данная последовательность температурных колебаний
    let temperatures = vec![-25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5, -12.0, 16.0, 5.4, 28.1, -18.2, -11.0];

    // HashMap для хранения температур по диапазонам
    let mut intervals: HashMap<String, Vec<f64>> = HashMap::new();

    // Проход по каждому значению температуры
    for temp in temperatures {
        // Определение нижней границы интервала с явным указанием типа
        let lower_bound = (temp / 10.0_f64).floor() * 10.0;
        let upper_bound = lower_bound + 10.0;
        let interval_key = format!("[{:.0}, {:.0})", lower_bound, upper_bound);

        // Добавление температуры в соответствующий интервал
        intervals.entry(interval_key).or_insert(Vec::new()).push(temp);
    }

    // Сортировка ключей и вывод интервалов с температурами
    let mut keys: Vec<_> = intervals.keys().collect();
    keys.sort();
    for key in keys {
        println!("{}: {:?}", key, intervals.get(key).unwrap());
    }
}
