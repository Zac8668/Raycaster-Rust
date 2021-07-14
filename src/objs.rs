pub mod objects {
    use std::usize;
    const PI: f64 = std::f64::consts::PI;

    use macroquad::color::Color;

    pub struct Player {
        pub x: f32,
        pub y: f32,
        pub angle: f64,
        pub delta_x: f64,
        pub delta_y: f64,
        pub render_distance: usize,
        pub fov: usize,
        pub rays: usize,
        pub size: f32,
        pub speed: f32,
        pub color: Color,
    }

    impl Player {
        pub fn update(&mut self) {
            self.delta_x = self.angle.cos() * (self.size * 2.5) as f64;
            self.delta_y = self.angle.sin() * (self.size * 2.5) as f64;

            if self.angle <= 0.0 {
                self.angle += 2.0 * PI;
            } else if self.angle >= 2.0 * PI {
                self.angle -= 2.0 * PI;
            }
        }
    }

    pub struct Map {
        pub width: usize,
        pub height: usize,
        pub size: f32,
        pub grid: Vec<Vec<i8>>,
    }

    impl Map {
        pub fn new(width: usize, height: usize, size: f32) -> Self {
            let grid = vec![
                /*[vec![1; width]; height];*/
                vec![1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1],
            ];

            Self {
                width,
                height,
                size,
                grid,
            }
        }
    }
}
