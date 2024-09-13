// Задача

// Реализовать пересечение двух неупорядоченных множеств.



use std::collections::HashSet;

fn main() {

    let x = HashSet::from([5, 8, 32 , 2, 1, 90, 4, 71]);
    let y = HashSet::from([4, 5, 6, 7, 8, 71, 18, 42, 32]);

    let q = HashSet::from([5, 8, 32 , 2, 1, 90, 4, 71]);
    let z = HashSet::from([4, 5, 6, 7, 8, 71, 18, 42, 32]);

    sets(x, y);

    sets_2(q, z);

}

//Первый способ
 fn sets(x: HashSet<i32>, y: HashSet<i32>) {

        let mut xy = vec![];

    // Используем циклы
        for q in x {
            for w in &y {
                if q == *w {
                    xy.push(q);
                }
            }
        }
        println!("{:?}", xy);
    }

// Второй способ
fn sets_2(x: HashSet<i32>, y: HashSet<i32>){

    // Используем встроенный метод intersection
    let intersection: Vec<i32> = x.intersection(&y).cloned().collect();

    println!("{:?}", intersection)
}
