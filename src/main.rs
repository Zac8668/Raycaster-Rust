mod objs;

use macroquad::prelude::*;
use objs::objects::*;

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
        color: YELLOW,
    };

    let mut map = Map::new(8, 8, 64.0);

    loop {
        input(&mut player);
        player.update();
        map.update();

        draw_map(&map);
        draw_player(&player);
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

fn draw_player(player: &Player) {
    draw_circle(player.x, player.y, player.size, player.color);
    //draw pointer
    draw_line(
        player.x,
        player.y,
        player.x + player.delta_x as f32,
        player.y + player.delta_y as f32,
        1.0,
        RED,
    );

} 

fn input(player: &mut Player) {
    if is_key_down(KeyCode::W) {
        player.x += player.delta_x as f32 / 3.0;
        player.y += player.delta_y as f32 / 3.0;
    }
    if is_key_down(KeyCode::S) {
        player.x -= player.delta_x as f32 / 3.0;
        player.y -= player.delta_y as f32 / 3.0;
    }
    if is_key_down(KeyCode::A) {
        player.angle += 0.2;
    }
    if is_key_down(KeyCode::D) {
        player.angle -= 0.2;
    }
}
