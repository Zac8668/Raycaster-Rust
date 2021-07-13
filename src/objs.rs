pub mod objects {
    use std::usize;

    use macroquad::color::Color;
    use macroquad::prelude::screen_height;

    pub struct Player {
        pub x: f32,
        pub y: f32,
        pub rot: f64,
        pub size: f32,
        pub speed: f32,
        pub color: Color,
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

        pub fn update(&mut self) {
            self.size = screen_height() / self.height as f32;
        }
    }
}
