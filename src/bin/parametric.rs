use euler::*;
use plotter_3d::{Line3, Runner};
use raylib::color::Color;

fn func(t: f32) -> Vec3 {
    let mut v: Vec3 = vec3!();
    v.x = (2.0 + (8.0 * t).cos()) * t.cos();
    v.y = (2.0 + (8.0 * t).cos()) * t.sin();
    v.z = (8.0 * t).sin();
    v
    //(2 + Cos[8 u]) Cos[u], (2 + Cos[8 u]) Sin[u], Sin[8 u]
}

fn setup() -> Vec<Line3> {
    let mut ans: Vec<Line3> = Vec::new();
    let graph = Color::PURPLE;

    let t_min: f32 = 0.0;
    let t_max: f32 = 2.0 * std::f32::consts::PI;
    let dt: f32 = 0.01;

    let mut p1;
    let mut p2;

    let mut t = t_min;

    p2 = func(t);
    t += dt;
    while t < t_max {
        p1 = p2;
        p2 = func(t);
        ans.push(Line3::from((p1, p2, graph)));
        t += dt;
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
