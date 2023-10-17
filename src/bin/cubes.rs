use euler::*;
use itertools::Itertools;
use plotter_3d::{Camera3, Line3, Runner};
use raylib::color::Color;

fn func(x: f32, y: f32, z: f32) -> f32 {
    (x.powi(2) + 9.0 / 4.0 * y.powi(2) + z.powi(2) - 1.0).powi(3)
        - x.powi(2) * z.powi(3)
        - 9.0 / 80.0 * y.powi(2) * z.powi(3)
}

fn setup() -> Vec<Line3> {
    let mut ans: Vec<Line3> = Vec::new();
    let graph = Color::PURPLE;

    const X_CELLS: usize = 30;
    const Y_CELLS: usize = 30;
    const Z_CELLS: usize = 30;

    const X_UP: f32 = 1.5;
    const X_DOWN: f32 = -1.5;
    const Y_UP: f32 = 1.5;
    const Y_DOWN: f32 = -1.5;
    const Z_UP: f32 = 1.5;
    const Z_DOWN: f32 = -1.5;

    const DX: f32 = (X_UP - X_DOWN) / (X_CELLS as f32);
    const DY: f32 = (Y_UP - Y_DOWN) / (Y_CELLS as f32);
    const DZ: f32 = (Z_UP - Z_DOWN) / (Z_CELLS as f32);

    let mut grid = [[[0.0; X_CELLS + 1]; Y_CELLS + 1]; Z_CELLS + 1];
    for ((i, j), l) in (0..X_CELLS)
        .cartesian_product(0..Y_CELLS)
        .cartesian_product(0..Z_CELLS)
    {
        grid[i][j][l] = func(
            X_DOWN + i as f32 * DX,
            Y_DOWN + j as f32 * DY,
            Z_DOWN + l as f32 * DZ,
        );
    }

    let bound: f32 = 0.0;

    for ((i, j), l) in (0..X_CELLS)
        .cartesian_product(0..Y_CELLS)
        .cartesian_product(0..Z_CELLS)
    {
        let mut window = [[[false; 2]; 2]; 2];
        window[0][0][0] = grid[i][j][l] < bound;
        window[1][0][0] = grid[i + 1][j][l] < bound;
        window[0][1][0] = grid[i][j + 1][l] < bound;
        window[1][1][0] = grid[i + 1][j + 1][l] < bound;
        window[0][0][1] = grid[i][j][l + 1] < bound;
        window[1][0][1] = grid[i + 1][j][l + 1] < bound;
        window[0][1][1] = grid[i][j + 1][l + 1] < bound;
        window[1][1][1] = grid[i + 1][j + 1][l + 1] < bound;

        let mut pt = [[[vec3!(); 2]; 2]; 2];
        let mut g = [[[0.0; 2]; 2]; 2];

        for ii in 0..=1 {
            for jj in 0..=1 {
                for ll in 0..=1 {
                    pt[ii][jj][ll] = Vec3 {
                        x: X_DOWN + (i + ii) as f32 * DX,
                        y: Y_DOWN + (j + jj) as f32 * DY,
                        z: Z_DOWN + (l + ll) as f32 * DZ,
                    };
                    g[ii][jj][ll] = grid[i + ii][j + jj][l + ll];
                }
            }
        }

        let mut vert = Vec::new();
        for ii in 0..2 {
            for jj in 0..2 {
                if window[ii][jj][0] ^ window[ii][jj][1] {
                    vert.push(
                        (pt[ii][jj][1] * g[ii][jj][0] - pt[ii][jj][0] * g[ii][jj][1])
                            / (g[ii][jj][0] - g[ii][jj][1]),
                    );
                }
            }
        }
        for ii in 0..2 {
            for jj in 0..2 {
                if window[ii][0][jj] ^ window[ii][1][jj] {
                    vert.push(
                        (pt[ii][1][jj] * g[ii][0][jj] - pt[ii][0][jj] * g[ii][1][jj])
                            / (g[ii][0][jj] - g[ii][1][jj]),
                    );
                }
            }
        }
        for ii in 0..2 {
            for jj in 0..2 {
                if window[0][ii][jj] ^ window[1][ii][jj] {
                    vert.push(
                        (pt[1][ii][jj] * g[0][ii][jj] - pt[0][ii][jj] * g[1][ii][jj])
                            / (g[0][ii][jj] - g[1][ii][jj]),
                    );
                }
            }
        }
        for u in 0..vert.len() {
            for v in u..vert.len() {
                ans.push(Line3::from((vert[u], vert[v], graph)));
            }
        }
    }
    ans
}

fn cam(t: f32) -> Camera3 {
    Camera3::from(
        vec3!(6.0 * t.sin(), 6.0 * t.cos(), 4.0),
        (vec3!() - vec3!(6.0 * t.sin(), 6.0 * t.cos(), 4.0)).normalize() * 3.0,
        (vec3!(0.0, 0.0, 13.0) - vec3!(6.0 * t.sin(), 6.0 * t.cos(), 4.0)).normalize(),
        (vec3!() - vec3!(6.0 * t.sin(), 6.0 * t.cos(), 4.0))
            .cross(vec3!(0.0, 0.0, 13.0) - vec3!(6.0 * t.sin(), 6.0 * t.cos(), 4.0))
            .normalize(),
    )
}

fn main() {
    let mut runner = Runner::new();
    runner.add_axes(2.0);
    runner.add_setup(Box::new(setup));
    runner.add_camera(Box::new(cam));
    let runner = runner.prepare();
    runner.run();
}
