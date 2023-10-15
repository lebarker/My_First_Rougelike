mod visibility_system;
use rltk::RandomNumberGenerator;
use visibility_system::VisibilitySystem;
pub mod cmp;
use cmp::Viewshed;
pub mod map;
use map::{draw_map, xy_idx, Map, TileType};
use rltk::{GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};
pub struct State {
    ecs: World,
}

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
pub struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::Left => try_move_player(-5, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(5, 0, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, -5, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, 5, &mut gs.ecs),
            _ => {}
        },
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
        ctx.print(1, 1, "Hello Rust World");
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    let map = Map::new_map_rooms_and_corridors();
    let rooms = map.rooms;
    if !rooms.is_empty() {
        let (player_x, player_y) = rooms[0].center();
        gs.ecs.insert(map);
        gs.ecs
            .create_entity()
            .with(Position {
                x: player_x,
                y: player_y,
            })
            .with(Renderable {
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Player {})
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
            })
            .build();

        rltk::main_loop(context, gs)
    } else {
        Ok(())
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}
