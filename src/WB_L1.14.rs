// Задача

// Подготовьте пример программы, которая в рантайме способна определить тип переменной, используйте std::any::*.


use std::any::{type_name_of_val, Any};

fn main() {

    type_val(2.0);
    type_val_2('s');
}

// Первый способ
fn type_val <T: Any + 'static> (value: T) {
  
// Приводим ссылку &value к типу &dyn std::any::Any для того, чтобы можно было использовать метод is::<T>() для проверки типа.
    match &value as &dyn std::any::Any {
        value if value.is::<&str>() => println!("String"),
        value if value.is::<i32>() => println!("i32"),
        value if value.is::<f64>() => println!("f64"),
        value if value.is::<bool>() => println!("bool"),
        value if value.is::<char>() => println!("char"),
        _ => println!("Unknown type"),
    }
}

// Второй способ
fn type_val_2 <T> (value: T) {
    if type_name_of_val(&value).contains("str") {
        println!("string")

    } else if type_name_of_val(&value).contains("i32") {
        println!("i32")

    } else if type_name_of_val(&value).contains("f64") {
        println!("f64")

    } else if type_name_of_val(&value).contains("bool") {
        println!("bool")

    } else if type_name_of_val(&value).contains("char") {
        println!("char")
    }else {
        println!("Unknown type")
    }
}
