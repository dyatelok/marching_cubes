use euler::*;
use raylib::prelude::*;

#[derive(Clone, Copy)]
struct Pt2 {
    x: f32,
    y: f32,
}

impl From<(f32, f32)> for Pt2 {
    fn from(p: (f32, f32)) -> Pt2 {
        Pt2 { x: p.0, y: p.1 }
    }
}

impl From<Vec2> for Pt2 {
    fn from(v: Vec2) -> Pt2 {
        Pt2 { x: v.x, y: v.y }
    }
}

impl Pt2 {
    fn from(x: f32, y: f32) -> Pt2 {
        Pt2 { x, y }
    }
}

#[derive(Clone, Copy)]
struct Line2 {
    start: Pt2,
    end: Pt2,
    col: Color,
}

impl Line2 {
    fn from(start: Pt2, end: Pt2, col: Color) -> Line2 {
        Line2 { start, end, col }
    }
}

#[derive(Clone, Copy)]
struct Pt3 {
    x: f32,
    y: f32,
    z: f32,
}

impl From<(f32, f32, f32)> for Pt3 {
    fn from(p: (f32, f32, f32)) -> Pt3 {
        Pt3 {
            x: p.0,
            y: p.1,
            z: p.2,
        }
    }
}

impl From<Vec3> for Pt3 {
    fn from(v: Vec3) -> Pt3 {
        Pt3 {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl Pt3 {
    fn from(x: f32, y: f32, z: f32) -> Pt3 {
        Pt3 { x, y, z }
    }
}

#[derive(Clone, Copy)]
pub struct Line3 {
    start: Pt3,
    end: Pt3,
    col: Color,
}

impl Line3 {
    fn from(start: Pt3, end: Pt3, col: Color) -> Line3 {
        Line3 { start, end, col }
    }
}

impl From<(Vec3, Vec3, Color)> for Line3 {
    fn from(tup: (Vec3, Vec3, Color)) -> Line3 {
        Line3 {
            start: tup.0.into(),
            end: tup.1.into(),
            col: tup.2,
        }
    }
}

#[derive(Clone, Copy, Default)]
struct Camera3 {
    pos: Vec3,
    dir: Vec3,
    base1: Vec3,
    base2: Vec3,
}

impl Camera3 {
    fn project_line(&self, line: &Line3) -> Line2 {
        Line2 {
            start: self.project_point(line.start),
            end: self.project_point(line.end),
            col: line.col,
        }
    }
    fn project_point(&self, point: Pt3) -> Pt2 {
        let mut p = Pt2::from(0.0, 0.0);
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

    fn set(&mut self, t: f32) {
        *self = Self {
            pos: vec3!(6.0 * t.sin(), 6.0 * t.cos(), 4.0),
            dir: (vec3!() - vec3!(6.0 * t.sin(), 6.0 * t.cos(), 4.0)).normalize() * 3.0,
            base1: (vec3!(0.0, 0.0, 13.0) - vec3!(6.0 * t.sin(), 6.0 * t.cos(), 4.0)).normalize(),
            base2: (vec3!() - vec3!(6.0 * t.sin(), 6.0 * t.cos(), 4.0))
                .cross(vec3!(0.0, 0.0, 13.0) - vec3!(6.0 * t.sin(), 6.0 * t.cos(), 4.0))
                .normalize(),
        };
    }
}

pub struct Building;
pub struct Ready;

pub struct Runner<State = Building> {
    name: String,
    origin: (f32, f32),
    screen: (f32, f32),
    scale: (f32, f32),
    lines: Vec<Line3>,
    camera: Camera3,
    setup: Box<dyn Fn() -> Vec<Line3>>,
    state: std::marker::PhantomData<State>,
}

impl Runner {
    pub fn new() -> Self {
        Runner {
            name: String::from("3d_plotter"),
            origin: (500., 500.),
            screen: (1000., 1000.),
            scale: (500., 500.),
            lines: Vec::new(),
            camera: Camera3::default(),
            setup: Box::new(|| Vec::new()),
            state: Default::default(),
        }
    }
}

impl Runner<Building> {
    pub fn add_axes(&mut self, length: f32) {
        self.lines.push(Line3 {
            start: Pt3::from(0.0, 0.0, 0.0),
            end: Pt3::from(length, 0.0, 0.0),
            col: Color::RED,
        });
        self.lines.push(Line3 {
            start: Pt3::from(0.0, 0.0, 0.0),
            end: Pt3::from(-length, 0.0, 0.0),
            col: Color::WHITE,
        });
        self.lines.push(Line3 {
            start: Pt3::from(0.0, 0.0, 0.0),
            end: Pt3::from(0.0, length, 0.0),
            col: Color::GREEN,
        });
        self.lines.push(Line3 {
            start: Pt3::from(0.0, 0.0, 0.0),
            end: Pt3::from(0.0, -length, 0.0),
            col: Color::WHITE,
        });
        self.lines.push(Line3 {
            start: Pt3::from(0.0, 0.0, 0.0),
            end: Pt3::from(0.0, 0.0, length),
            col: Color::BLUE,
        });
        self.lines.push(Line3 {
            start: Pt3::from(0.0, 0.0, 0.0),
            end: Pt3::from(0.0, 0.0, -length),
            col: Color::WHITE,
        });
    }
    pub fn add_setup(&mut self, fun: Box<dyn Fn() -> Vec<Line3>>) {
        self.setup = fun;
    }
    pub fn add_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn prepare(mut self) -> Runner<Ready> {
        self.lines.extend((*self.setup)());

        Runner {
            name: self.name,
            origin: self.origin,
            screen: self.screen,
            scale: self.scale,
            lines: self.lines,
            camera: self.camera,
            setup: self.setup,
            state: std::marker::PhantomData::<Ready>,
        }
    }
}

impl Runner<Ready> {
    pub fn run(mut self) {
        let backgroung = Color::BLACK;

        let mut tau: f32 = 0.0;
        let dtau: f32 = 0.01;

        let (mut rl, thread) = raylib::init()
            .size(self.screen.0 as i32, self.screen.1 as i32)
            .title(&self.name)
            .build();

        rl.set_target_fps(60);
        while !rl.window_should_close() {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(backgroung);

            self.camera.set(tau);
            tau += dtau;

            for line2 in self
                .lines
                .iter()
                .map(|line3| self.camera.project_line(line3))
            {
                d.draw_line(
                    (self.origin.1 + self.scale.1 * line2.start.y) as i32,
                    (self.origin.0 - self.scale.0 * line2.start.x) as i32,
                    (self.origin.1 + self.scale.1 * line2.end.y) as i32,
                    (self.origin.0 - self.scale.0 * line2.end.x) as i32,
                    line2.col,
                );
            }
        }
    }
}
