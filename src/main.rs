/* Bugs
    Make code more readable and clean
    solved but i dont know how and maybe i can solve it better - vertical/horizontal where isnt suposed to be and light passing trought two diagonal blocks, when looking down right
    walls missing when getting close on top left off the map
    add render distance and not check distance
*/

use macroquad::prelude::*;
use std::f64::consts::PI;

mod objs;
use objs::objects::*;

const DEG: f64 = 0.0174533;
const HOR_COLOR: Color = BLUE;
const VER_COLOR: Color = DARKBLUE;
const OFF: f32 = -0.0001;

struct Ray {
    x: f32,
    y: f32,
    lenght: f32,
    angle: f64,
    is_hor: bool,
}

#[macroquad::main("RayCaster")]
async fn main() {
    let mut player = Player {
        x: 400.0,
        y: 400.0,
        size: 10.0,
        speed: 5.0,
        delta_x: 0.0,
        delta_y: 0.0,
        angle: PI,
        fov: 60,
        rays: 5000,
        render_distance: 12,
        color: YELLOW,
        show_map: false
    };

    let grid: Vec<Vec<i8>> = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let mut map = Map::new(8, 8, 64.0, Some(grid));

    loop {
        input(&mut player, &mut map);
        player.rays = (screen_width() - map.width as f32 * map.size) as usize; //one ray per pixel
        player.update();

        if player.show_map {
            draw_map(&map, &player);
        }

        for i in 0..player.rays {
            draw_3d(cast_ray(&player, &map, i), &map, &player, i);
        }

        draw_text(&get_fps().to_string(), 10.0, 60.0, 60.0, WHITE);
        next_frame().await
    }
}

fn draw_map(map: &Map, player: &Player) {
    for (y, row) in map.grid.iter().enumerate() {
        for (x, item) in row.iter().enumerate() {
            if *item == 1 {
                draw_rectangle(
                    x as f32 * map.size,
                    y as f32 * map.size,
                    map.size,
                    map.size,
                    GRAY,
                );
            }
        }

        draw_line(
            0.0,
            y as f32 * map.size - 1.0,
            map.size * map.width as f32,
            y as f32 * map.size - 1.0,
            2.0,
            DARKGRAY,
        );
    }

    for y in 0..map.height + 1 {
        draw_line(
            y as f32 * map.size - 1.0,
            0.0,
            y as f32 * map.size - 1.0,
            map.size * map.height as f32,
            2.0,
            DARKGRAY,
        );
    }

    draw_line(
        0.0,
        map.height as f32 * map.size - 1.0,
        map.size * map.width as f32,
        map.height as f32 * map.size - 1.0,
        2.0,
        DARKGRAY,
    );

    draw_circle(player.x, player.y, player.size, player.color);
    //draw pointer
    draw_line(
        player.x,
        player.y,
        player.x + player.delta_x as f32,
        player.y + player.delta_y as f32,
        8.0,
        YELLOW,
    );
}

fn horizontal(player: &Player, map: &Map, angle: f64) -> Option<Ray> {
    let mut y: f32;
    let mut x: f32;
    let y_off: f32;
    let x_off: f32;
    let inv_tan = (-1.0 / angle.tan()) as f32;

    if angle > PI {
        //looking up
        y = ((player.y / map.size) as i32 * map.size as i32) as f32 + OFF;
        x = (player.y - y) * inv_tan + player.x;
        y_off = -map.size;
        x_off = -y_off * inv_tan;
    } else if angle < PI {
        //looking down
        y = ((player.y / map.size) as i32 as f32 * map.size + map.size) as f32;
        x = (player.y - y) * inv_tan + player.x;
        y_off = map.size;
        x_off = -y_off * inv_tan;
    } else {
        return None;
    }

    for _ in 0..player.render_distance {
        let map_x = x / map.size;
        let map_y = y / map.size;

        if map_x >= 0.0
            && map_y >= 0.0
            && ((map_x as usize) < map.width && (map_y as usize) < map.height)
            && map.grid[map_y as usize][map_x as usize] != 0
        {
            return Some(Ray {
                x,
                y,
                lenght: line_lenght([player.x, player.y], [x, y]),
                angle,
                is_hor: true,
            });
        }

        x += x_off;
        y += y_off;
    }
    None
}

fn vertical(player: &Player, map: &Map, angle: f64) -> Option<Ray> {
    let mut y: f32;
    let mut x: f32;
    let y_off: f32;
    let x_off: f32;
    let neg_tan = -angle.tan() as f32;
    if angle > PI * 0.5 && angle < PI * 1.5 {
        //looking left
        x = ((player.x / map.size) as i32 * map.size as i32) as f32 + OFF;
        y = (player.x - x) * neg_tan + player.y;
        x_off = -map.size;
        y_off = -x_off * neg_tan;
    } else if !(PI * 0.5..=PI * 1.5).contains(&angle) {
        //looking rigth
        x = ((player.x / map.size) as i32 as f32 * map.size + map.size) as f32;
        y = (player.x - x) * neg_tan + player.y;
        x_off = map.size;
        y_off = -x_off * neg_tan;
    } else {
        return None;
    }

    for _ in 0..player.render_distance {
        let map_x = x / map.size;
        let map_y = y / map.size;

        if map_x >= 0.0
            && map_y >= 0.0
            && ((map_x as usize) < map.width && (map_y as usize) < map.height)
            && map.grid[map_y as usize][map_x as usize] != 0
        {
            return Some(Ray {
                x,
                y,
                lenght: line_lenght([player.x, player.y], [x, y]),
                angle,
                is_hor: false,
            });
        }
        x += x_off;
        y += y_off;
    }
    None
}

fn input(player: &mut Player, map: &mut Map) {
    if is_key_down(KeyCode::W) {
        player.x += player.delta_x as f32 / 3.0;
        player.y += player.delta_y as f32 / 3.0;
    }
    if is_key_down(KeyCode::S) {
        player.x -= player.delta_x as f32 / 3.0;
        player.y -= player.delta_y as f32 / 3.0;
    }
    if is_key_down(KeyCode::A) {
        player.angle -= 0.1;
    }
    if is_key_down(KeyCode::D) {
        player.angle += 0.1;
    }

    if is_key_pressed(KeyCode::M) {
        player.show_map = !player.show_map;
    }

    if is_mouse_button_pressed(MouseButton::Left) && player.show_map{
        let (x, y) = mouse_position();

        let x = (x / map.size as f32) as usize;
        let y = (y / map.size as f32) as usize;

        if x < map.width && y < map.height {
            if map.grid[y][x] == 0 {
                map.grid[y][x] = 1;
            } else {
                map.grid[y][x] = 0;
            }
        }
    }
}

//needs changes
fn cast_ray(player: &Player, map: &Map, i: usize) -> Option<Ray> {
    let ver = vertical(
        &player,
        &map,
        valid_angle(&get_angle(player.angle, player.fov, player.rays, i)),
    );

    let hor = horizontal(
        &player,
        &map,
        valid_angle(&get_angle(player.angle, player.fov, player.rays, i)),
    );
    if hor.is_some() && ver.is_some() {
        let hor = hor.unwrap();
        let ver = ver.unwrap();

        if hor.lenght < ver.lenght {
            if player.show_map {
                draw_line(player.x, player.y, hor.x, hor.y, 2.0, HOR_COLOR);
            }
            Some(hor)
        } else {
            if player.show_map {
                draw_line(player.x, player.y, ver.x, ver.y, 2.0, VER_COLOR);
            }
            Some(ver)
        }
    } else if let Some(ver) = ver {
        if player.show_map {
            draw_line(player.x, player.y, ver.x, ver.y, 2.0, VER_COLOR);
        }
        Some(ver)
    } else if let Some(hor) = hor {
        if player.show_map {
            draw_line(player.x, player.y, hor.x, hor.y, 2.0, HOR_COLOR);
        }
        Some(hor)
    } else {
        None
    }
}

//needs changes
fn draw_3d(ray: Option<Ray>, map: &Map, player: &Player, i: usize) {
    if let Some(ray) = ray {
        //fix fisheye
        let mut height = ray.lenght * valid_angle(&(player.angle - ray.angle as f64)).cos() as f32;
        height = (map.size * screen_height()) / height;

        //limit height
        if height > screen_height() {
            height = screen_height()
        }

        let x_off: f32 = if player.show_map {screen_width() - map.size as f32 * map.width as f32} else {screen_width()};

        //center ray
        let off = screen_height() / 2.0 - height / 2.0;
        let y2 = height + off;
        let x = (i as f32 + 0.5)
            * (x_off / player.rays as f32)
            + if player.show_map{map.size * map.width as f32} else {0.0};

        //get color
        let color: Color;
        if ray.is_hor {
            color = HOR_COLOR;
        } else {
            color = VER_COLOR;
        }

        draw_line(
            x,
            off,
            x,
            y2,
            (x_off / player.rays as f32) as f32,
            color,
        );
    }
}

//get angle based on fov and how much rays will be cast
fn get_angle(angle: f64, fov: usize, rays: usize, i: usize) -> f64 {
    let mut angle = valid_angle(&(angle - fov as f64 / 2.0 * DEG));
    angle += (i as f64 * (fov as f64 / rays as f64)) as f64 * DEG;
    valid_angle(&angle)
}

//returns line lenght
fn line_lenght(pos1: [f32; 2], pos2: [f32; 2]) -> f32 {
    ((pos2[0] - pos1[0]).powi(2) + (pos2[1] - pos1[1]).powi(2)).sqrt()
}

//returns a valid angle
fn valid_angle(angle: &f64) -> f64 {
    let mut new_angle = *angle;

    while new_angle < 0.0 {
        new_angle += 2.0 * PI;
    }
    while new_angle > 2.0 * PI {
        new_angle -= 2.0 * PI;
    }

    new_angle
}
