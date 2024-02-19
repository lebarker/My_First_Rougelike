mod player;
mod visibility_system;
use rltk::RandomNumberGenerator;
use visibility_system::VisibilitySystem;
mod components;
use components::Viewshed;
pub mod map;
use map::{draw_map, xy_idx, Map, TileType};
use player::player_input;
use rltk::{GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};
pub struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
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

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems(); // Check if it is defined in the book
 
        let map = self.ecs.fetch::<Map>();
        draw_map(&self.ecs, ctx);

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
    let mut gs = State {
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    let map : Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    gs.ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(Viewshed{ visible_tiles : Vec::new(), range: 8, dirty: true })
        .build();

        for room in map.rooms.iter().skip(1) {
            let (x,y) = room.center();
            gs.ecs.create_entity()
                .with(Position{ x, y })
                .with(Renderable{
                    glyph: rltk::to_cp437('g'),
                    fg: RGB::named(rltk::RED),
                    bg: RGB::named(rltk::BLACK),
                })
                .with(Viewshed{ visible_tiles : Vec::new(), range: 8, dirty: true })
                .build();
        }
        
        gs.ecs.insert(map);

    rltk::main_loop(context, gs)
    
}