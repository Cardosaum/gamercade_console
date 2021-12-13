use crate::core::Rom;
use rlua::{Context, Function, Lua};

use super::Console;
use crate::api::*;

pub struct LuaConsole {
    rom: Rom,
    lua: Lua,
    player_count: usize,
}

impl Console for LuaConsole {
    fn call_handle_input(&self) {
        // Call the roms handle_input function for each player
        self.lua.context(|ctx| {
            let handle_input: Function = ctx.globals().get("handle_input").unwrap();
            (0..self.player_count)
                .for_each(|player_id| handle_input.call::<usize, ()>(player_id + 1).unwrap());
        });
    }

    fn call_update(&self) {
        //Call the rom's update function
        self.lua.context(|ctx| {
            let update: Function = ctx.globals().get("update").unwrap();
            update.call::<_, ()>(()).unwrap();
        });
    }

    fn call_draw(&self) {
        // Call the rom's draw function
        self.lua.context(|ctx| {
            let draw: Function = ctx.globals().get("draw").unwrap();
            draw.call::<_, ()>(()).unwrap();
        });
    }

    fn rom(&self) -> &Rom {
        &self.rom
    }
}

impl LuaConsole {
    pub fn new(rom: Rom, player_count: usize, code: &str) -> Self {
        let lua = Lua::new();
        lua.context(|ctx| {
            // Load the user lua scripts
            ctx.load(code).exec().unwrap();

            //TODO: Figure out how to call this
            //let clear_screen = ctx.create_function(|_, clear_screen|);
            //let set_pixel = ctx.create_function(|_, set_pixel);
        });

        Self {
            rom,
            lua,
            player_count,
        }
    }
}
