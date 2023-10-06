# marching_cubes
Marching cubes implementation written in rust.

Marching cubes is a computer graphics algorithm for extracting a polygonal mesh of an isosurface from a three-dimensional discrete scalar field.

To run use ```cargo run --release```

To use your own function edit
```
fn func(x: f32, y: f32, z: f32) -> f32 {
    (x.powi(2) + 9.0 / 4.0 * y.powi(2) + z.powi(2) - 1.0).powi(3)
        - x.powi(2) * z.powi(3)
        - 9.0 / 80.0 * y.powi(2) * z.powi(3)
}
```
in src/main.rs
