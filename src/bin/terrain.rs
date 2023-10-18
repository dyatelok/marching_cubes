use euler::*;
use plotter_3d::{Camera3, Line3, Runner};
use rand::Rng;
use raylib::color::Color;

fn noise(x: f32, y: f32, v: &[Vec<Vec2>]) -> f32 {
    let side = (v.len() - 1) as f32;
    let i = ((x * side).floor() as usize).min(v.len() - 2).max(0);
    let ii = i as f32 / side;
    let ii1 = (i + 1) as f32 / side;
    let j = ((y * side).floor() as usize).min(v.len() - 2).max(0);
    let jj = j as f32 / side;
    let jj1 = (j + 1) as f32 / side;
    let mut grid: [[f32; 2]; 2] = [[0.0; 2]; 2];
    grid[0][0] = vec2!(x - ii, y - jj).dot(v[i][j]);
    grid[0][1] = vec2!(x - ii, y - jj1).dot(v[i][j + 1]);
    grid[1][0] = vec2!(x - ii1, y - jj).dot(v[i + 1][j]);
    grid[1][1] = vec2!(x - ii1, y - jj1).dot(v[i + 1][j + 1]);
    let x = (x - ii) * side;
    let y = (y - jj) * side;

    interp(grid, x, y)
}

fn interp(grid: [[f32; 2]; 2], x: f32, y: f32) -> f32 {
    let x = fade(x);
    let y = fade(y);
    let a = lerp(grid[0][0], grid[1][0], x);
    let b = lerp(grid[0][1], grid[1][1], x);
    lerp(a, b, y)
}

fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0) // 6t^5 - 15t^4 + 10t^3
}

fn lerp(a: f32, b: f32, x: f32) -> f32 {
    a + (b - a) * x
}

fn setup() -> Vec<Line3> {
    let mut v: Vec<Vec<Vec2>> = Vec::new();
    let r: usize = 4 + 1;
    let mut rng = rand::thread_rng();
    v.resize(r, Vec::new());
    for i in 0..r {
        v[i].resize(r, vec2!());
        for j in 0..r {
            let mut x: f32 = rng.gen();
            x -= 0.5;
            let mut y: f32 = rng.gen();
            y -= 0.5;
            v[i][j] = vec2!(x, y).normalize();
        }
    }

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
            temp.push(Vec3 {
                x,
                y,
                z: noise((x + 1.0) / 2.0, (y + 1.0) / 2.0, &v) * 2.0,
            });
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
    for i in 1..xyz.len() {
        ans.push(Line3::from((xyz[i][0], xyz[i - 1][0], graph)));
    }
    for j in 1..xyz.len() {
        ans.push(Line3::from((xyz[0][j], xyz[0][j - 1], graph)));
    }

    ans
}

fn cam(t: f32) -> Camera3 {
    Camera3::from(
        vec3!(2.0 * t.sin(), 2.0 * t.cos(), 2.0),
        (vec3!() - vec3!(5.0 * t.sin(), 5.0 * t.cos(), 5.0)).normalize() * 1.5,
        (vec3!(0.0, 0.0, 10.0) - vec3!(5.0 * t.sin(), 5.0 * t.cos(), 5.0)).normalize(),
        (vec3!() - vec3!(5.0 * t.sin(), 5.0 * t.cos(), 5.0))
            .cross(vec3!(0.0, 0.0, 10.0) - vec3!(5.0 * t.sin(), 5.0 * t.cos(), 5.0))
            .normalize(),
    )
}

fn main() {
    let mut runner = Runner::new();
    runner.add_setup(Box::new(setup));
    runner.add_camera(Box::new(cam));
    let runner = runner.prepare();
    runner.run();
}
