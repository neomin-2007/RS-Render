use macroquad::prelude::*;
use std::path::Path;
use std::fs;

use std::f32::consts::PI;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

const FONT_SIZE: f32 = 23.4;


#[derive(Serialize, Deserialize, Debug)]
pub struct Geometry {
    pub vertex: Vec<[f32;3]>,
    pub edges: Vec<[usize;2]>,

    pub user_angle_x: f32,
    pub user_angle_y: f32,
    
    pub user_scale: f32,
    pub user_distance: f32,
    pub user_x: f32,
    pub user_y: f32,
}

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * (PI / 180.0)
}

pub fn rotate(x: f32, y: f32, z: f32, angleX: f32, angleY: f32) -> [f32;3] {

    let radX = degrees_to_radians(angleX);
    let radY = degrees_to_radians(angleY);

    let cosX = radX.cos();
    let sinX = radX.sin();

    let cosY = radY.cos();
    let sinY = radY.sin();

    let xRotY = x * cosY - z * sinY;
    let zRotY = x * sinY + z * cosY;

    let yRotX = y * cosX - zRotY * sinX;
    let zRotX = y * sinX + zRotY * cosX;

    return [xRotY, yRotX, zRotX];
}

pub fn project(x: f32, y: f32, z: f32, scale: f32, distance: f32) -> [f32;2] {

    let x2d = (x * scale)/(z + distance) + (screen_width() / 2 as f32);
    let y2d = (y * scale)/(z + distance) + (screen_height() / 2 as f32);

    return [x2d, y2d];
}

impl Geometry {



    pub fn draw(&self) {

        for edge in &self.edges {

            let vertex1 = rotate(
                self.vertex[edge[0]][0],
                self.vertex[edge[0]][1],
                self.vertex[edge[0]][2],
                self.user_angle_x,
                self.user_angle_y,
            );

            let vertex2 = rotate(
                self.vertex[edge[1]][0],
                self.vertex[edge[1]][1],
                self.vertex[edge[1]][2],
                self.user_angle_x,
                self.user_angle_y,
            );

            let p1 = project(
                vertex1[0],
                vertex1[1],
                vertex1[2],
                self.user_scale,
                self.user_distance,
            );

            let p2 = project(
                vertex2[0],
                vertex2[1],
                vertex2[2],
                self.user_scale,
                self.user_distance,
            );


            draw_line(p1[0] + self.user_x, p1[1] + self.user_y, p2[0] + self.user_x, p2[1] + self.user_y, 2.0, WHITE);
        }

    }

    pub fn movement(&mut self) {

        if is_key_down(KeyCode::O) {
            if self.user_angle_x <= 360.0 {
                self.user_angle_x += 1.0;
            } else {
                self.user_angle_x = 0.0;
            }
        }

        if is_key_down(KeyCode::L) {
            if self.user_angle_y <= 360.0 {
                self.user_angle_y += 1.0;
            } else {
                self.user_angle_y = 0.0;
            }
        }

        if is_key_down(KeyCode::Equal) {
            self.user_scale += 1.0;
        }
        if is_key_down(KeyCode::Minus) {
            self.user_scale -= 1.0;
        }


        if is_key_down(KeyCode::Up) {
            self.user_y -= 1.0;
        }
        if is_key_down(KeyCode::Down) {
            self.user_y += 1.0;
        }
        if is_key_down(KeyCode::Left) {
            self.user_x -= 1.0;
        }

        if is_key_down(KeyCode::Right) {
            self.user_x += 1.0;
        }

    }

    pub fn draw_use() {

        draw_text("(^) To up", 675.0 , 500.0, FONT_SIZE, WHITE);
        draw_text("(v) To Down", 675.0, 525.0, FONT_SIZE, WHITE);
        draw_text("(>) To Right", 675.0, 550.0, FONT_SIZE, WHITE);
        draw_text("(<) To Left", 675.0, 575.0, FONT_SIZE, WHITE);

        draw_text("(O >-) Rotate X", 520.0, 500.0, FONT_SIZE, WHITE);
        draw_text("(L -<) Rotate Y", 520.0, 525.0, FONT_SIZE, WHITE);
        draw_text("(U -) Zoom out", 520.0, 550.0, FONT_SIZE, WHITE);
        draw_text("(H +) Zoom in", 520.0, 575.0, FONT_SIZE, WHITE);
    }

}

pub struct Menu {
    pub opened_file_menu: bool,
    pub current_path: String,
    pub files: Vec<String>,
    pub selected_file: Option<String>,
    pub scroll_offset: f32,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            opened_file_menu: false,
            current_path: String::from("."),
            files: Vec::new(),
            selected_file: None,
            scroll_offset: 0.0,
        }
    }

    pub fn new_geometry() -> Geometry {

        let mut cube = Geometry {
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
        
        return cube;
    }

    pub fn list_files(&mut self) {
        self.files.clear();
        if let Ok(entries) = fs::read_dir(&self.current_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let path_str = path.to_string_lossy().to_string();
                    self.files.push(path_str);
                }
            }
        }
    }

    pub fn handle_click(&mut self) {
        if !self.opened_file_menu {
            let mouse_x = mouse_position().0;
            let mouse_y = mouse_position().1;

            let clicked = mouse_x >= 275.0 && mouse_x <= (275.0 + 270.0)
                && mouse_y >= 475.0 && mouse_y <= (475.0 + 45.0);

            if clicked {
                self.opened_file_menu = !self.opened_file_menu;
                self.list_files();
            }
        } else {
            let mut y_offset = 160.0 - self.scroll_offset;
            let mut action: Option<Action> = None;

            for file in &self.files {
                if is_mouse_button_pressed(MouseButton::Left) {
                    let mouse_x = mouse_position().0;
                    let mouse_y = mouse_position().1;

                    if mouse_x >= 275.0 && mouse_x <= (275.0 + 270.0)
                        && mouse_y >= y_offset - 25.0 && mouse_y <= y_offset
                    {
                        let path = Path::new(file);
                        if path.is_dir() {
                            action = Some(Action::ChangeDirectory(file.to_string()));
                        } else {
                            action = Some(Action::SelectFile(file.to_string()));
                        }
                    }
                }
                y_offset += 25.0;
            }

            if self.current_path != "." {
                if is_mouse_button_pressed(MouseButton::Left) {
                    let mouse_x = mouse_position().0;
                    let mouse_y = mouse_position().1;

                    if mouse_x >= 275.0 && mouse_x <= (275.0 + 270.0)
                        && mouse_y >= 500.0 && mouse_y <= (500.0 + 45.0)
                    {
                        action = Some(Action::GoBack);
                    }
                }
            }

            match action {
                Some(Action::ChangeDirectory(path)) => {
                    self.current_path = path;
                    self.list_files();
                    self.scroll_offset = 0.0;
                }
                Some(Action::SelectFile(file)) => {
                    self.selected_file = Some(file);
                }
                Some(Action::GoBack) => {
                    self.current_path = String::from("..");
                    self.list_files();
                    self.scroll_offset = 0.0;
                }
                None => {}
            }

            let (_, mouse_wheel_y) = mouse_wheel();
            self.scroll_offset -= mouse_wheel_y * 10.0;
            let max_scroll = (self.files.len() as f32 * 25.0) - 340.0;
            self.scroll_offset = self.scroll_offset.max(0.0).min(max_scroll);
        }
    }

    pub fn draw(&mut self) {
        if !self.opened_file_menu {
            draw_text("RSRender", 315.0, 200.0, 45.0, WHITE);
            draw_text("by Neomin", 305.0, 245.0, 45.0, WHITE);

            let button_color = if mouse_position().0 >= 275.0
                && mouse_position().0 <= (275.0 + 270.0)
                && mouse_position().1 >= 475.0
                && mouse_position().1 <= (475.0 + 45.0)
            {
                Color::from_rgba(200, 50, 50, 255) 
            } else {
                Color::from_rgba(250, 50, 50, 255) 
            };

            draw_rectangle(275.0, 475.0, 270.0, 45.0, button_color);
            draw_text("Click to Import json", 295.0, 500.0, 25.0, WHITE);
        } else {
            let max_width = self
                .files
                .iter()
                .map(|file| measure_text(file, None, 20, 1.0).width)
                .fold(0.0, f32::max);

            let rect_width = max_width + 40.0;

            draw_rectangle(275.0, 150.0, rect_width, 340.0, Color::from_rgba(50, 50, 50, 255));

            let mut y_offset = 160.0 - self.scroll_offset;
            for file in &self.files {
                draw_text(&file, 280.0, y_offset, 20.0, WHITE);
                y_offset += 25.0;
            }

            if self.current_path != "." {
                draw_rectangle(275.0, 500.0, rect_width, 45.0, Color::from_rgba(100, 100, 250, 255));
                draw_text("Back", 380.0, 525.0, 25.0, WHITE);
            }
        }
    }
}

enum Action {
    ChangeDirectory(String),
    SelectFile(String),
    GoBack,
}