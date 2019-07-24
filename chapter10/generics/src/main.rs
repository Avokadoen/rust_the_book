
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&mut self) -> &mut T {
        &mut self.x
    }
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let mut p = Point { x: 5, y: 10 };
    {
        let x = p.x();
        println!("p.x = {}", x);
        *x += 6;
    }

    println!("p.x = {}", p.x());
    let x = p.x();
    *x += 6;
    println!("p.x = {}", x);

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

}
