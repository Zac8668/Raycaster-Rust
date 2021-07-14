mod objs;

use macroquad::prelude::*;
use objs::objects::*;
use std::f64::consts::PI;
const DEGREE: f64 = 0.0174533;

#[macroquad::main("RayCaster")]
async fn main() {
    let mut player = Player {
        x: 20.0,
        y: 20.0,
        size: 10.0,
        speed: 5.0,
        angle: 0.0,
        delta_x: 0.0,
        delta_y: 0.0,
        fov: 100,
        rays: 100,
        render_distance: 8,
        color: YELLOW,
    };

    let mut map = Map::new(8, 8, 64.0);

    loop {
        input(&mut player, &mut map);
        player.update();

        draw_map(&map);
        draw_player(&player);
        for i in 0..player.rays {
            draw_3d(get_ray(&player, &map, i), &map, &player, i);
        }
        next_frame().await
    }
}

fn draw_map(map: &Map) {
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

    for y in 0..map.height {
        draw_line(
            y as f32 * map.size - 1.0,
            0.0,
            y as f32 * map.size - 1.0,
            map.size * map.height as f32,
            2.0,
            DARKGRAY,
        );
    }
}

fn horizontal(player: &Player, map: &Map, angle: f64) -> Option<[f32; 5]> {
    let mut y: f32;
    let mut x: f32;
    let y_off: f32;
    let x_off: f32;
    let inv_tan = (-1.0 / angle.tan()) as f32;

    if angle > PI {
        //looking up
        y = ((player.y / map.size) as i32 * map.size as i32) as f32 - 1.0;
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
        let map_x = (x / map.size) as usize;
        let map_y = (y / map.size) as usize;

        if (map_x < map.width && map_y < map.height) && map.grid[map_y][map_x] != 0 {
            return Some([x, y, line_lenght([player.x, player.y], [x, y]), angle as f32, 0.0]);
        }

        x += x_off;
        y += y_off;
    }
    None
}

fn vertical(player: &Player, map: &Map, angle: f64) -> Option<[f32; 5]> {
    let mut y: f32;
    let mut x: f32;
    let y_off: f32;
    let x_off: f32;
    let neg_tan = -angle.tan() as f32;
    if angle > PI * 0.5 && angle < PI * 1.5 {
        //looking left
        x = ((player.x / map.size) as i32 * map.size as i32) as f32 -1.0;
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
        let map_x = (x / map.size) as usize;
        let map_y = (y / map.size) as usize;

        if (map_x < map.width && map_y < map.height) && map.grid[map_y][map_x] != 0 {
            return Some([x, y, line_lenght([player.x, player.y], [x, y]), angle as f32, 1.0]);
        }

        x += x_off;
        y += y_off;
    }
    None
}

fn draw_player(player: &Player) {
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
        player.angle -= 0.2;
    }
    if is_key_down(KeyCode::D) {
        player.angle += 0.2;
    }

    if is_mouse_button_pressed(MouseButton::Left) {
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

fn get_ray(player: &Player, map: &Map, i: usize) -> [f32; 5]{
    let ver_pos = vertical(
        &player,
        &map,
        valid_ray(
            &(player.angle - (player.fov as f64 * DEGREE / 2.0)
                + ((i) as f64 + 0.5) * (player.fov / player.rays) as f64 * DEGREE),
        ),
    );

    let hor_pos = horizontal(
        &player,
        &map,
        valid_ray(
            &(player.angle - (player.fov as f64 * DEGREE / 2.0)
                + ((i) as f64 + 0.5) * (player.fov / player.rays) as f64 * DEGREE),
        ),
    );
    if hor_pos.is_some() && ver_pos.is_some() {
        let hor_pos = hor_pos.unwrap();
        let ver_pos = ver_pos.unwrap();

        if hor_pos[2] < ver_pos[2] {
            draw_line(
                player.x,
                player.y,
                hor_pos[0],
                hor_pos[1],
                2.0,
                GREEN,
            );
            hor_pos
        } else {
            draw_line(
                player.x,
                player.y,
                ver_pos[0],
                ver_pos[1],
                2.0,
                DARKGREEN,
            );
            ver_pos
        }

    } else if ver_pos.is_some() {
        let ver_pos = ver_pos.unwrap();

        draw_line(
            player.x,
            player.y,
            ver_pos[0],
            ver_pos[1],
            2.0,
            DARKGREEN,
        );
        ver_pos
    } else {
        let hor_pos = hor_pos.unwrap();

        draw_line(
            player.x,
            player.y,
            hor_pos[0],
            hor_pos[1],
            2.0,
            GREEN
        );
        hor_pos
    } 
}

fn draw_3d(ray: [f32;5], map: &Map, player: &Player, i: usize) {
    let mut height = ray[2] * valid_ray(&(player.angle - ray[3] as f64)).cos() as f32;
    height = (map.size * screen_height()) / height;
    if height > screen_height() {height = screen_height()}
    let off = screen_height() / 2.0 - height / 2.0;
    let y2 = height + off;
    let x = (i as f32 + 0.5) * (screen_width() / player.rays as f32) + map.size * map.width as f32;
    let color: Color;
    if ray[4] == 0.0 {
        color = GREEN;
    } else {
        color = DARKGREEN;
    }

    draw_line(x, off, x, y2, screen_width() / player.rays as f32, color);

}

fn line_lenght(pos1: [f32; 2], pos2: [f32; 2]) -> f32 {
    ((pos2[0] - pos1[0]).powi(2) + (pos2[1] - pos1[1]).powi(2)).sqrt()
}

fn valid_ray(angle: &f64) -> f64 {
    let mut new_angle = *angle;

    while new_angle < 0.0 {
        new_angle += 2.0 * PI;
    }
    while new_angle > 2.0 * PI {
        new_angle -= 2.0 * PI;
    }

    new_angle
}
