// Задача
// Реализовать паттерн «адаптер» на любом примере.



// Клиентский интерфейс
trait StringPrint{
    fn string_print(&self) -> String;
}

// Сервис
struct NumberPrint{
    number: i32
}

impl NumberPrint{

    fn new(number: i32) -> Self{
        Self{number}
    }

    fn number_print(&self) -> i32{
        self.number
    }
}

// Адаптер
struct NumberToStringAdapter{
    adapter: NumberPrint
}

impl NumberToStringAdapter{

    fn new(adapter: NumberPrint) -> Self{
        Self{adapter}
    }
}

impl StringPrint for NumberToStringAdapter{

    fn string_print(&self) -> String{
        format!("Число: {}", self.adapter.number_print())
    }
}

fn main() {

    let nb = NumberPrint::new(45);

    let adapter = NumberToStringAdapter::new(nb);

    println!("{}", adapter.string_print())
}
