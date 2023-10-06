use euler::*;
use itertools::Itertools;
use raylib::prelude::*;

#[derive(Clone)]
struct Pt {
    x: f32,
    y: f32,
}

impl From<(f32, f32)> for Pt {
    fn from(p: (f32, f32)) -> Pt {
        Pt { x: p.0, y: p.1 }
    }
}

impl From<Vec2> for Pt {
    fn from(v: Vec2) -> Pt {
        Pt { x: v.x, y: v.y }
    }
}

impl Pt {
    fn from(x: f32, y: f32) -> Pt {
        Pt { x, y }
    }
}

#[derive(Clone)]
struct Line {
    start: Pt,
    end: Pt,
    col: Color,
}

#[allow(dead_code)]
impl Line {
    fn from(start: Pt, end: Pt, col: Color) -> Line {
        Line { start, end, col }
    }
}

#[derive(Clone)]
struct Line3d {
    start: Vec3,
    end: Vec3,
    col: Color,
}

#[allow(dead_code)]
impl Line3d {
    fn from(start: Vec3, end: Vec3, col: Color) -> Line3d {
        Line3d { start, end, col }
    }
}

#[derive(Clone, Copy)]
struct Camera3d {
    pos: Vec3,
    dir: Vec3,
    base1: Vec3,
    base2: Vec3,
}

impl Camera3d {
    fn project_line(&self, line: &Line3d) -> Line {
        Line {
            start: self.project_point(line.start),
            end: self.project_point(line.end),
            col: line.col,
        }
    }
    fn project_point(&self, point: Vec3) -> Pt {
        let mut p = Pt::from(0.0, 0.0);
        let mut v = vec![0.0_f32; 3];

        let m = Mat3::new(
            self.base1.x,
            self.base2.x,
            self.dir.x,
            self.base1.y,
            self.base2.y,
            self.dir.y,
            self.base1.z,
            self.base2.z,
            self.dir.z,
        );

        v[0] = point.x - self.pos.x;
        v[1] = point.y - self.pos.y;
        v[2] = point.z - self.pos.z;

        let d = m.determinant();
        let mut mm = m;
        mm.m00 = v[0];
        mm.m10 = v[1];
        mm.m20 = v[2];
        let d0 = mm.determinant();
        let mut mm = m;
        mm.m01 = v[0];
        mm.m11 = v[1];
        mm.m21 = v[2];
        let d1 = mm.determinant();
        let mut mm = m;
        mm.m02 = v[0];
        mm.m12 = v[1];
        mm.m22 = v[2];
        let d2 = mm.determinant();
        let x0 = d0 / d;
        let x1 = d1 / d;
        let x2 = d2 / d;
        p.x = x0 / x2;
        p.y = x1 / x2;
        p
    }

    fn cam(t: f32) -> Self {
        Self {
            pos: vec3!(6.0 * t.sin(), 6.0 * t.cos(), 4.0),
            dir: (vec3!() - vec3!(6.0 * t.sin(), 6.0 * t.cos(), 4.0)).normalize() * 3.0,
            base1: (vec3!(0.0, 0.0, 13.0) - vec3!(6.0 * t.sin(), 6.0 * t.cos(), 4.0)).normalize(),
            base2: (vec3!() - vec3!(6.0 * t.sin(), 6.0 * t.cos(), 4.0))
                .cross(vec3!(0.0, 0.0, 13.0) - vec3!(6.0 * t.sin(), 6.0 * t.cos(), 4.0))
                .normalize(),
        }
    }
}

fn func(x: f32, y: f32, z: f32) -> f32 {
    (x.powi(2) + 9.0 / 4.0 * y.powi(2) + z.powi(2) - 1.0).powi(3)
        - x.powi(2) * z.powi(3)
        - 9.0 / 80.0 * y.powi(2) * z.powi(3)
}

fn main() {
    let origin = Pt { x: 500.0, y: 500.0 };
    let screen = Pt {
        x: 1000.0,
        y: 1000.0,
    };
    let (mut rl, thread) = raylib::init()
        .size(screen.x as i32, screen.y as i32)
        .title("3d_grapher")
        .build();
    let scale_x: f32 = 500.0;
    let scale_y: f32 = 500.0;
    let mut c: Camera3d;
    let mut P: Vec<Line>;
    let mut PP: Vec<Line3d> = Vec::new();

    let backgroung = Color::BLACK;
    let graph = Color::PURPLE;

    //Добавляем оси
    PP.push(Line3d {
        start: vec3!(0.0, 0.0, 0.0),
        end: vec3!(2.0, 0.0, 0.0),
        col: Color::RED,
    });
    PP.push(Line3d {
        start: vec3!(0.0, 0.0, 0.0),
        end: vec3!(-2.0, 0.0, 0.0),
        col: Color::WHITE,
    });
    PP.push(Line3d {
        start: vec3!(0.0, 0.0, 0.0),
        end: vec3!(0.0, 2.0, 0.0),
        col: Color::GREEN,
    });
    PP.push(Line3d {
        start: vec3!(0.0, 0.0, 0.0),
        end: vec3!(0.0, -2.0, 0.0),
        col: Color::WHITE,
    });
    PP.push(Line3d {
        start: vec3!(0.0, 0.0, 0.0),
        end: vec3!(0.0, 0.0, 2.0),
        col: Color::BLUE,
    });
    PP.push(Line3d {
        start: vec3!(0.0, 0.0, 0.0),
        end: vec3!(0.0, 0.0, -2.0),
        col: Color::WHITE,
    });
    //Проектируем линии на экран
    //Колличество ячеек по осям
    const X_CELLS: usize = 30;
    const Y_CELLS: usize = 30;
    const Z_CELLS: usize = 30;
    //Устанавливаем границы, в которых будет осуществлен просчет
    let x_up: f32 = 1.5;
    let x_down: f32 = -1.5;
    let y_up: f32 = 1.5;
    let y_down: f32 = -1.5;
    let z_up: f32 = 1.5;
    let z_down: f32 = -1.5;
    //Считаем шаги по осям
    let dx = (x_up - x_down) / (X_CELLS as f32);
    let dy = (y_up - y_down) / (Y_CELLS as f32);
    let dz = (z_up - z_down) / (Z_CELLS as f32);
    // Делаем сетку значений
    let mut grid = [[[0.0; X_CELLS + 1]; Y_CELLS + 1]; Z_CELLS + 1];
    for ((i, j), l) in (0..X_CELLS)
        .cartesian_product(0..Y_CELLS)
        .cartesian_product(0..Z_CELLS)
    {
        grid[i][j][l] = func(
            x_down + i as f32 * dx,
            y_down + j as f32 * dy,
            z_down + l as f32 * dz,
        );
    }
    let C: f32 = 0.0;
    let mut G: [[[bool; 2]; 2]; 2];
    let mut PT = [[[vec3!(); 2]; 2]; 2];
    let mut g = [[[0.0; 2]; 2]; 2];
    let mut V: Vec<Vec3>;
    for ((i, j), l) in (0..X_CELLS)
        .cartesian_product(0..Y_CELLS)
        .cartesian_product(0..Z_CELLS)
    {
        G = [[[false; 2]; 2]; 2];
        if grid[i][j][l] < C {
            G[0][0][0] = true;
        }
        if grid[i + 1][j][l] < C {
            G[1][0][0] = true;
        }
        if grid[i][j + 1][l] < C {
            G[0][1][0] = true;
        }
        if grid[i + 1][j + 1][l] < C {
            G[1][1][0] = true;
        }
        if grid[i][j][l + 1] < C {
            G[0][0][1] = true;
        }
        if grid[i + 1][j][l + 1] < C {
            G[1][0][1] = true;
        }
        if grid[i][j + 1][l + 1] < C {
            G[0][1][1] = true;
        }
        if grid[i + 1][j + 1][l + 1] < C {
            G[1][1][1] = true;
        }

        for I in 0..2 {
            for J in 0..2 {
                for L in 0..2 {
                    PT[I][J][L] = Vec3 {
                        x: x_down + (i + I) as f32 * dx,
                        y: y_down + (j + J) as f32 * dy,
                        z: z_down + (l + L) as f32 * dz,
                    };
                    g[I][J][L] = grid[i + I][j + J][l + L];
                }
            }
        }

        V = Vec::new();
        for I in 0..2 {
            for J in 0..2 {
                if G[I][J][0] ^ G[I][J][1] {
                    V.push(
                        (PT[I][J][1] * g[I][J][0] - PT[I][J][0] * g[I][J][1])
                            / (g[I][J][0] - g[I][J][1]),
                    );
                }
            }
        }
        for I in 0..2 {
            for J in 0..2 {
                if G[I][0][J] ^ G[I][1][J] {
                    V.push(
                        (PT[I][1][J] * g[I][0][J] - PT[I][0][J] * g[I][1][J])
                            / (g[I][0][J] - g[I][1][J]),
                    );
                }
            }
        }
        for I in 0..2 {
            for J in 0..2 {
                if G[0][I][J] ^ G[1][I][J] {
                    V.push(
                        (PT[1][I][J] * g[0][I][J] - PT[0][I][J] * g[1][I][J])
                            / (g[0][I][J] - g[1][I][J]),
                    );
                }
            }
        }
        for u in 0..V.len() {
            for v in u..V.len() {
                PP.push(Line3d {
                    start: V[u],
                    end: V[v],
                    col: graph,
                });
            }
        }
    }

    let mut tau: f32 = 0.0;
    let dtau: f32 = 0.01;

    //Отрисовка всего
    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(backgroung);
        //Проектируем линии на экран
        c = Camera3d::cam(tau);
        tau += dtau;
        P = Vec::new();
        for p in &PP {
            P.push(c.project_line(p));
        }
        //Рисуем линии
        for r in &P {
            d.draw_line(
                (origin.y + scale_y * r.start.y) as i32,
                (origin.x - scale_x * r.start.x) as i32,
                (origin.y + scale_y * r.end.y) as i32,
                (origin.x - scale_x * r.end.x) as i32,
                r.col,
            );
        }
    }
}
