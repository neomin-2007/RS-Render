use macroquad::prelude::*;
use render::Menu;
use render::Geometry;

use std::fs::File;
use std::io::prelude::*;

mod render;

#[macroquad::main("RSRender")]
async fn main() {
    let mut geometric = Geometry {
        vertex: [
            [0.0, 100.0, 0.0], [100.0, 100.0, 0.0], [100.0, 0.0, 0.0], [0.0, 0.0, 0.0],
            [0.0, 100.0, 100.0], [100.0, 100.0, 100.0], [100.0, 0.0, 100.0], [0.0, 0.0, 100.0]
        ].to_vec(),
        edges: [
            [0, 1], [1, 2], [2, 3], [3, 0],
            [7, 6], [6, 5], [5, 4], [4, 7],
            [0, 4], [1, 5], [2, 6], [3, 7]
          ].to_vec(),
          user_angle_x: 0.0,
          user_angle_y: 0.0,
          user_scale: 500.0,
          user_distance: 500.0,
          user_x: 0.0,
          user_y: 0.0,
    };

    let mut menu = Menu::new();
    let mut apply_render = false;
    let mut loading_message: Option<String> = None;

    loop {
        clear_background(RED);

        if !apply_render {
            menu.handle_click();
            menu.draw();

            if let Some(selected_file) = &menu.selected_file {
                println!("Selected file: {}", selected_file);

                loading_message = Some("Loading file...".to_string());

                if let Ok(mut file) = File::open(selected_file) {
                    let mut contents = String::new();
                    if file.read_to_string(&mut contents).is_ok() {
                        match serde_json::from_str::<Geometry>(&contents) {
                            Ok(new_geometry) => {
                                geometric = new_geometry;
                                loading_message = None;
                                apply_render = true;
                                menu.opened_file_menu = false;
                                menu.selected_file = None;
                            }
                            Err(e) => {
                                loading_message = Some(format!("Error to deserialize JSON: {}", e));
                            }
                        }
                    } else {
                        loading_message = Some("Error to read.".to_string());
                    }
                } else {
                    loading_message = Some("Erro to open the file.".to_string());
                }
            }

            if let Some(message) = &loading_message {
                draw_text(
                    message,
                    screen_width() / 2.0 - 100.0,
                    screen_height() / 2.0,
                    30.0,
                    WHITE,
                );
            }
        }

        if apply_render {
            Geometry::draw_use();
            geometric.draw();
            geometric.movement();

            if is_key_pressed(KeyCode::Escape) {
                apply_render = false;
            }
        }

        next_frame().await;
    }
}