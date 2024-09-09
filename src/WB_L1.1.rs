// Задача

// Сделать трейт Action с методом say, который должен выводить сообщение в консоль.
// Сделать структуру Person, которая содержит строковое имя.
// Сделать реализацию трейта Action для структуры Person, в котором метод say будет выводить в консоль текст “Hello, NAME” (где NAME - имя, хранимое в структуре).


// Определяем трейд Action
trait Actions{
    fn say(self);
}

//Определяем структуру Person
struct Person{
    name: String
}

// Реализация трейда Action для структуры
impl Actions for Person{
    fn say(self) {
        println!("Hello, {}", self.name)
    }
}

fn main() {
    let x = Person {
        name: "John".to_string()
    };

    x.say();
}
