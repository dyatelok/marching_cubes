use euler::*;
use plotter_3d::{Line3, Runner};
use raylib::color::Color;

fn func(x: f32, y: f32) -> Vec3 {
    let mut v: Vec3 = vec3!();
    v.x = x;
    v.y = y;
    v.z = x + y.powi(2);
    v
}

fn setup() -> Vec<Line3> {
    let mut ans: Vec<Line3> = Vec::new();
    let graph = Color::PURPLE;

    let min_x: f32 = -1.0;
    let min_y: f32 = -1.0;
    let max_x: f32 = 1.0;
    let max_y: f32 = 1.0;

    let dx: f32 = 0.05;
    let dy: f32 = 0.05;

    let mut xyz: Vec<Vec<Vec3>> = Vec::new();

    let mut x;
    let mut y = min_y;

    while y < max_y {
        let mut temp = Vec::new();
        x = min_x;
        while x < max_x {
            temp.push(func(x, y));
            x += dx;
        }
        y += dy;
        xyz.push(temp);
    }

    for i in 1..xyz.len() {
        for j in 1..xyz[0].len() {
            ans.push(Line3::from((xyz[i][j], xyz[i][j - 1], graph)));
            ans.push(Line3::from((xyz[i][j], xyz[i - 1][j], graph)));
            ans.push(Line3::from((xyz[i][j], xyz[i - 1][j - 1], graph)));
        }
    }

    ans
}

fn main() {
    let mut runner = Runner::new();
    runner.add_axes(2.0);
    runner.add_setup(Box::new(setup));
    let runner = runner.prepare();
    runner.run();
}
