# Linum
Linum is an Open-Source vector library made for Game Development, and general Physics.
The name comes from the Latin word 'linum', meaning thread. The English words 'line', and 'linear' both come from Latin 'linea' which comes from linum.

## Examples
Below is an example of some things linum can do:
```Rust
use linum::prelude::*;

fn main() {
    {
        let vec = vec2d(5.0, 0.0);
        let rotated = vec.rotate_deg(180.0);
        println!("{:?}", rotated) // Prints `Vec2D { x: -5.0, y: 0.0 }`
    }
    {
        let vec1 = Vec2D::from(vec![3,7]);
        let vec2 = vec2d(0u8, 5u8);
        println!("Result = {:?}", vec1 + vec2.cast().unwrap()); // Prints `Vec2D { x: 3, y: 12 }`
    }
    {
        let vec1 = vec2d(3u8, 6u8);
        let scalar = 5f64;
        println!("Result = {:?}", vec1.cast().unwrap() * scalar); // Prints `Vec2D { x: 15.0, y: 30.0 }
    }
}
```
