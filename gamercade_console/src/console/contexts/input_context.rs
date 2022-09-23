use paste::paste;

use gamercade_core::{ButtonCode, InputState};

use crate::api::InputApi;
use crate::console::PlayerInputEntry;

#[derive(Clone)]
pub struct InputContext {
    pub(crate) input_entries: Box<[PlayerInputEntry]>,
}

impl InputContext {
    pub fn new(num_players: usize) -> Self {
        Self {
            input_entries: (0..num_players)
                .map(|_| PlayerInputEntry::default())
                .collect(),
        }
    }
}

/// This file automatically derives the various "get input" or "check input"
/// types of functions based on the macro at the bottom. This would otherwise be a
/// long and error prone process.

macro_rules! derive_generate_input_api {
    (
        Buttons { $($btn_name:ident: $btn_code:ident,)* },
        Analogs { $($anlg_name:ident,)* },
        Triggers { $($trg_name:ident,)* },
        Mouse {
            Buttons { $($mbtn_name:ident,)* },
            Axis { $($maxis_name:ident,)* },
            Wheel { $($mwheel_name:ident,)* },
         },
    ) => {
        paste! {
            impl InputApi for InputContext {
                $(
                    fn [<button_ $btn_name _pressed>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            let prev = player_input.previous.get_button_state(ButtonCode::$btn_code);
                            let curr = player_input.current.buttons.get_button_state(ButtonCode::$btn_code);
                            (prev == false && curr == true) as i32
                        } else {
                            -1
                        }
                    }

                    fn [<button_ $btn_name _released>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            let prev = player_input.previous.get_button_state(ButtonCode::$btn_code);
                            let curr = player_input.current.buttons.get_button_state(ButtonCode::$btn_code);
                            (prev == true && curr == false) as i32
                        } else {
                            -1
                        }
                    }

                    fn [<button_ $btn_name _held>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            player_input.current.buttons.get_button_state(ButtonCode::$btn_code) as i32
                        } else {
                            -1
                        }
                    }
                )*

                $(
                    fn [<analog_ $anlg_name _x>](&self, player_id: i32) -> f32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            player_input.current.[<$anlg_name _stick>].get_x_axis()
                        } else {
                            f32::NAN
                        }
                    }

                    fn [<analog_ $anlg_name _y>](&self, player_id: i32) -> f32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            player_input.current.[<$anlg_name _stick>].get_y_axis()
                        } else {
                            f32::NAN
                        }
                    }
                )*

                $(
                    fn [<trigger_ $trg_name>](&self, player_id: i32) -> f32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            player_input.current.[<$trg_name _trigger>].get_value()
                        } else {
                            f32::NAN
                        }
                    }
                )*

                $(
                    fn [<mouse_ $mbtn_name _pressed>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            let prev = player_input.previous_mouse.[<get_ $mbtn_name _button_down>]();
                            let curr = player_input.current_mouse.[<get_ $mbtn_name _button_down>]();
                            (prev == false && curr == true) as i32
                        } else {
                            -1
                        }
                    }

                    fn [<mouse_ $mbtn_name _released>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            let prev = player_input.previous_mouse.[<get_ $mbtn_name _button_down>]();
                            let curr = player_input.current_mouse.[<get_ $mbtn_name _button_down>]();
                            (prev == true && curr == false) as i32
                        } else {
                            -1
                        }
                    }

                    fn [<mouse_ $mbtn_name _held>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            player_input.current_mouse.[<get_ $mbtn_name _button_down>]() as i32
                        } else {
                            -1
                        }
                    }
                )*

                $(
                    fn [<mouse_ $maxis_name _pos>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            player_input.current_mouse.[<get_ $maxis_name _pos>]() as i32
                        } else {
                            -1
                        }
                    }

                    fn [<mouse_ $maxis_name _delta>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            player_input.current_mouse.[<get_ $maxis_name _delta>]() as i32
                        } else {
                            i32::MIN
                        }
                    }
                )*

                $(
                    fn [<mouse_wheel_ $mwheel_name>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            player_input.current_mouse.[<get_wheel_ $mwheel_name>]() as i32
                        } else {
                            -1
                        }
                    }
                )*

                fn raw_mouse_state(&self, player_id: i32) -> i64 {
                    if let Some(player_input) = self.input_entries.get(player_id as usize) {
                        player_input.current_mouse.0 as i64
                    } else {
                        1 << gamercade_core::MOUSE_INVALID_BIT
                    }
                }

                fn raw_input_state(&self, player_id: i32) -> i64 {
                    let state = if let Some(player_input) = self.input_entries.get(player_id as usize) {
                        player_input.current
                    } else {
                        InputState::INVALID_STATE
                    };

                    unsafe { std::mem::transmute(state) }
                }
            }
        }
    }
}

derive_generate_input_api! {
    Buttons {
        a: A,
        b: B,
        c: C,
        d: D,
        up: Up,
        down: Down,
        left: Left,
        right: Right,
        start: Start,
        select: Select,
        left_shoulder: LeftShoulder,
        right_shoulder: RightShoulder,
        left_stick: LeftStick,
        right_stick: RightStick,
        left_trigger: LeftTrigger,
        right_trigger: RightTrigger,
    },
    Analogs {
        left,
        right,
    },
    Triggers {
        left,
        right,
    },
    Mouse {
        Buttons {
            left,
            right,
            middle,
        },
        Axis {
            x,
            y,
        },
        Wheel {
            up,
            down,
            left,
            right,
        },
    },
}
