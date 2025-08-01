
#[allow(unused_assignments)]
#[allow(dead_code)]
#[allow(unused_variables)]

#[derive(Debug)]
struct Point<T> { x: T, y: T }

impl<T> Point<T> {
    fn xy(self) -> (T, T) {
        (self.x, self.y)
    }
}
/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

pub fn main() {

    println!("--------------------  generic");
    let point = Point::<f64>{x: 3., y: 5.};
    println!("{:#?}", point);
    println!("{:#?}", point.xy());



}

