#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
pub mod nonui;

use std::fs::read_dir;
use std::path::{absolute, Path, PathBuf};
use raylib::drawing::RaylibDrawHandle;
use raylib::{init, RaylibHandle, RaylibThread};
use raylib::prelude::{Color, RaylibDraw, Rectangle, Texture2D, Vector2};
use raylib::prelude::MouseButton::{MOUSE_BUTTON_LEFT, MOUSE_BUTTON_RIGHT};
use vea_shared::helper::Vec2;
use crate::nonui::{build, run, Panel};

fn rectangle_from_points(x1: f32, y1: f32, x2: f32, y2: f32) -> Rectangle {
    Rectangle::new(x1, y1, x2 - x1, y2 - y1)
}

fn draw_text_centered(d: &mut RaylibDrawHandle, text: &str, font_size: i32, rec: Rectangle, color: Color) {
    let text_width = d.measure_text(text, font_size);
    let text_height = font_size; // in Raylib, font_size ≈ glyph height in pixels

    let x = rec.x + (rec.width - text_width as f32) / 2.0;
    let y = rec.y + (rec.height - text_height as f32) / 2.0;

    d.draw_text(text, x as i32, y as i32, font_size, color);
}

fn next_button(d: &mut RaylibDrawHandle, x: f32, y: &mut f32, label: &str, mouse: Vector2, is_disabled: bool) -> bool {
    let rect = Rectangle::new(x, *y, BUTTON_WIDTH, BUTTON_HEIGHT);
    d.draw_rectangle_rounded(
        rect.clone(),
        0.4,
        8,
        if is_disabled {
            BUTTON_DISABLED
        } else if rect.check_collision_point_rec(mouse) {
            BUTTON_HOVER
        } else {
            BUTTON_NORMAL
        }
    );
    draw_text_centered(d, label, 24, rect, Color::WHITE);

    *y += BUTTON_HEIGHT + BETWEEN_BUTTON_GAP;  // ERROR here
    !is_disabled && rect.check_collision_point_rec(mouse)
}

fn next_game_panel(d: &mut RaylibDrawHandle, index: u32, x: f32, y: &mut f32, name: &str, path: PathBuf, version: &str, hover: &mut Option<u32>, mouse: Vector2, selected: &Option<u32>, logo: &Option<Texture2D>) {
    let rect = rectangle_from_points(
        x,
        *y,
        1280.0 - PROJECTS_AREA_GAP - BUTTONS_AREA_GAP - BUTTONS_AREA_WIDTH - BUTTONS_AREA_GAP,  // Also update resolution here if you ever update resolution in the main part (prob will never will happen)
        *y + GAME_PANEL_HEIGHT
    );
    d.draw_rectangle_rounded_lines(
        rect.clone(),
        0.1,
        8,
        if selected.is_some_and(|x| x == index) {
            Color::WHITE
        } else {
            BUTTON_NORMAL
        }
    );

    // Logo
    const LOGO_GAP: f32 = 10.0;
    let logo_rect = Rectangle::new(
        rect.x + LOGO_GAP, rect.y + LOGO_GAP,
        rect.height - 2.0 * LOGO_GAP, rect.height - 2.0 * LOGO_GAP,
    );
    match logo {
        Some(texture) => {
            d.draw_texture_pro(
                &texture,
                Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32),
                logo_rect,
                Vector2::zero(),
                0.0,
                Color::WHITE,
            )
        }
        None => {
            d.draw_rectangle_rounded(
                logo_rect,
                0.2,
                8,
                BUTTON_NORMAL
            );
        }
    };

    // Name
    d.draw_text(
        name,
        (rect.x + 2.0 * LOGO_GAP + rect.height - 2.0 * LOGO_GAP) as i32,
        (rect.y + LOGO_GAP) as i32,
        (rect.height / 4.0).floor() as i32,
        Color::WHITE,
    );

    // Path
    d.draw_text(
        absolute(path).unwrap().to_str().unwrap(),
        (rect.x + 2.0 * LOGO_GAP + rect.height - 2.0 * LOGO_GAP) as i32,
        (rect.y + rect.height - LOGO_GAP - (rect.height / 5.0).floor()) as i32,
        (rect.height / 5.0).floor() as i32,
        Color::GRAY,
    );

    // version
    d.draw_text(
        version,
        (rect.x + rect.width - LOGO_GAP - d.measure_text(version, (rect.height / 5.0).floor() as i32) as f32) as i32,
        (rect.y + LOGO_GAP) as i32,
        (rect.height / 5.0).floor() as i32,
        Color::GRAY,
    );

    // TODO: actual logo texture

    if rect.check_collision_point_rec(mouse) {
        *hover = Some(index);
    }
    *y += GAME_PANEL_HEIGHT + GAME_PANEL_GAP;
}

const BASE_COLOR: Color = Color::new(41, 41, 41, 255);
const VERTICAL_GAP: f32 = 4.0;

const PROJECTS_AREA_COLOR: Color = Color::new(31, 31, 31, 255);
const PROJECTS_AREA_GAP: f32 = VERTICAL_GAP;

const BUTTONS_AREA_WIDTH: f32 = 200.0;
const BUTTONS_AREA_GAP: f32 = 2.0;

const BUTTON_WIDTH: f32 = BUTTONS_AREA_WIDTH - 2.0 * BUTTONS_AREA_GAP;
const BUTTON_HEIGHT: f32 = 40.0;
const BETWEEN_BUTTON_GAP: f32 = 5.0;

const BUTTON_NORMAL: Color = Color::new(95, 155, 199, 255);
const BUTTON_HOVER: Color = Color::new(37, 150, 190, 255);
const BUTTON_DISABLED: Color = Color::new(47, 88, 124, 255);

const GAME_PANEL_HEIGHT: f32 = 100.0;
const GAME_PANEL_GAP: f32 = 10.0;

fn get_panels(raylib_handle: &mut RaylibHandle, raylib_thread: &mut RaylibThread) -> Vec<Panel> {
    let mut panels = vec![];

    for f in read_dir("dist").unwrap() {
        if f.is_err() {
            continue;
        }
        let path = f.unwrap().path();

        if path.is_file() {
            continue;
        }

        let panel = Panel::from_path(path, raylib_handle, raylib_thread);
        if panel.is_none() {
            continue;
        }

        panels.push(panel.unwrap());
    }

    panels
}

fn draw_all_panels(d: &mut RaylibDrawHandle, panels: &Vec<Panel>, mouse: Vector2, selected_game: &Option<u32>) -> Option<u32> {
    let mut y = VERTICAL_GAP;
    let mut hovered_game: Option<u32> = None;

    for (ind, panel) in panels.iter().enumerate() {
        next_game_panel(d, ind as u32, PROJECTS_AREA_GAP, &mut y, panel.name.as_str(),
                        panel.path.clone(), panel.version.as_str(), &mut hovered_game,
                        mouse, &selected_game, &panel.logo);
    }

    hovered_game
}

fn is_built(selected_game: &Option<u32>, panels: &Vec<Panel>) -> bool {
    let Some(index) = selected_game else { return false; };
    panels[*index as usize].path.with_extension("vec").exists()
}

fn main() {
    #[allow(nonstandard_style)]
    let SCREEN_RESOLUTION: Vec2<i32> = Vec2::new(1280, 720);

    let mut selected_game: Option<u32> = None;

    #[allow(unused_mut)]
    let (mut rl, mut thread) = init()
        .size(SCREEN_RESOLUTION.x, SCREEN_RESOLUTION.y)
        .title("VEA Launcher")
        .build();

    rl.set_target_fps(60);

    let games = get_panels(&mut rl, &mut thread);

    while !rl.window_should_close() {
        let mouse = rl.get_mouse_position();

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(BASE_COLOR);

        d.draw_rectangle_rounded(
            rectangle_from_points(
                PROJECTS_AREA_GAP,
                VERTICAL_GAP,
                SCREEN_RESOLUTION.x as f32 - PROJECTS_AREA_GAP - BUTTONS_AREA_GAP - BUTTONS_AREA_WIDTH - BUTTONS_AREA_GAP,
                SCREEN_RESOLUTION.y as f32 - VERTICAL_GAP,
            ),
            0.02,
            8,
            PROJECTS_AREA_COLOR
        );

        let mut y = VERTICAL_GAP;

        let is_build_hovered = next_button(&mut d, SCREEN_RESOLUTION.x as f32 - BUTTONS_AREA_WIDTH - BUTTONS_AREA_GAP, &mut y, "Build", mouse, selected_game == None);
        let is_play_hovered = next_button(&mut d, SCREEN_RESOLUTION.x as f32 - BUTTONS_AREA_WIDTH - BUTTONS_AREA_GAP, &mut y, "Play", mouse, selected_game == None || !is_built(&selected_game, &games));

        let hovered_game: Option<u32> = draw_all_panels(&mut d, &games, mouse, &selected_game);

        drop(d);  // rl is freeeee!

        if rl.is_mouse_button_pressed(MOUSE_BUTTON_LEFT) {
            if is_build_hovered {
                build(games[selected_game.unwrap() as usize].path.clone())
            } else if is_play_hovered {
                run(games[selected_game.unwrap() as usize].path.with_extension("vec"))
            } else if hovered_game.is_some() {
                selected_game = hovered_game;
            }
        }
        if rl.is_mouse_button_down(MOUSE_BUTTON_RIGHT) {
            selected_game = None;
        }
    };
}