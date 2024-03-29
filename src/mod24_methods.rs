use crate::mod07_structs;

impl mod07_structs::Line // name of struct for which methods should be implemented
{
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.y;
        let dy = self.start.y - self.end.y;
        (dx * dy + dx * dy).sqrt()
    }
}

fn methods() {
    let p1 = mod07_structs::Point { x: 1.0, y: 2.0 };
    let p2 = mod07_structs::Point { x: 3.0, y: 4.0 };
    let myline = mod07_structs::Line { start: p1, end: p2 };

    println!("myline length = {}", myline.len());
}

pub fn main() {
    methods()
}
