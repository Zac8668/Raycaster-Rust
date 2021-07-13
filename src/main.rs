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
        rot: 0.0,
        color: YELLOW,
    };

    let mut map = Map::new(8, 8, 64.0);

    loop {
        input(&mut player);
        map.update();

        draw_map(&map);
        draw_circle(player.x, player.y, player.size, player.color);
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
    }
}

fn input(player: &mut Player) {
    if is_key_down(KeyCode::W) {
        player.y -= player.speed;
    }
    if is_key_down(KeyCode::S) {
        player.y += player.speed;
    }
    if is_key_down(KeyCode::A) {
        player.x -= player.speed;
    }
    if is_key_down(KeyCode::D) {
        player.x += player.speed;
    }
}
