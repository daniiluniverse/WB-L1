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
