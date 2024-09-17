struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // Конструктор для создания новой точки
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    // Метод для вычисления евклидова расстояния между двумя точками
    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        // Вычисляем евклидово расстояние по формуле sqrt(dx^2 + dy^2)
        (dx.powi(2) + dy.powi(2)).sqrt()
    }

    // Методы для получения значений x и y
    fn get_x(&self) -> f64 {
        self.x
    }

    fn get_y(&self) -> f64 {
        self.y
    }
}

fn main() {
    // Создаем две точки
    let p1 = Point::new(3.0, 4.0);
    let p2 = Point::new(7.0, 12.0);

    // Вычисляем расстояние между точками
    let distance = p1.distance(&p2);

    
    println!(
        "Расстояние между точкой 1 ({}, {}) и точкой 2 ({}, {}) равно {:.2}",
        p1.get_x(),
        p1.get_y(),
        p2.get_x(),
        p2.get_y(),
        distance
    );
}
