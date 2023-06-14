use std::{io::Write, time::Duration, ops::Sub};

const PI: f32 = ::std::f32::consts::PI;
const DEG2RAD: f32 = PI / 180f32;
const DISP_OFFSET: (i32, i32) = (52, 24);

const RADIUS: i32 = 16;
const CAM_DIST: i32 = RADIUS * 3;
const PROJECTION_DIST: i32 = CAM_DIST - RADIUS;

fn main() {
    let cube_points_0 = initialize_points(RADIUS);
    let camera_pos = Point::from((0, 0, CAM_DIST));
    let display_plane = Point::from((0, 0, PROJECTION_DIST));

    let mut rot_alpha = 0.0;
    let mut rot_gamma = 0.0;
    let rot_alpha_speed = DEG2RAD * 2.0;
    let rot_gamma_speed = DEG2RAD * 16.0;

    let render_delay = Duration::from_millis(100);
    loop {
        ::std::thread::sleep(render_delay);

        rot_alpha += rot_alpha_speed;
        rot_gamma += rot_gamma_speed;
        let (sina, cosa) = rot_alpha.sin_cos();
        let (sing, cosg) = rot_gamma.sin_cos();
        if rot_alpha > 2.0 * PI { rot_alpha -= 2.0 * PI };
        if rot_gamma > 2.0 * PI { rot_gamma -= 2.0 * PI };

        print!("\u{001b}[H\u{001b}[2J");
        for point in &cube_points_0 {
            draw_point(point.clone(), camera_pos, display_plane, sina, cosa, sing, cosg);
        }

        ::std::io::stdout().flush().unwrap();
    }
}

fn draw_point(point: Point, camera: Point, iplane: Point, sina: f32, cosa: f32, sing: f32, cosg: f32) {
    let rel_cam = Point::from((
        point.x * cosg + point.y * sing,
        -point.x * cosa * sing + point.y * cosa * cosg + point.z * sina,
        point.x * sina * sing - point.y * sina * cosg + point.z * cosa,
    )) - camera;
    let (bx, by) = (iplane.z / rel_cam.z * rel_cam.x, iplane.z / rel_cam.z * rel_cam.y);
    print!(
        "\u{001b}[{};{}H",
        by.round() as i32 + DISP_OFFSET.1,
        bx.round() as i32 + DISP_OFFSET.0
    );
    print!("##");
}

fn initialize_points(radius: i32) -> Vec<Point> {
    let mut points = vec![];
    for i in -radius..radius {
        points.push(Point::from((i, radius, radius)));
        points.push(Point::from((i, radius, -radius)));
        points.push(Point::from((i, -radius, radius)));
        points.push(Point::from((i, -radius, -radius)));

        points.push(Point::from((radius, i, radius)));
        points.push(Point::from((-radius, i, radius)));
        points.push(Point::from((radius, i, -radius)));
        points.push(Point::from((-radius, i, -radius)));

        points.push(Point::from((radius, radius, i)));
        points.push(Point::from((-radius, radius, i)));
        points.push(Point::from((radius, -radius, i)));
        points.push(Point::from((-radius, -radius, i)));
    }
    points
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl From<(f32, f32, f32)> for Point {
    fn from(value: (f32, f32, f32)) -> Self {
        Point {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl From<(i32, i32, i32)> for Point {
    fn from(value: (i32, i32, i32)) -> Self {
        Point {
            x: value.0 as f32,
            y: value.1 as f32,
            z: value.2 as f32,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
