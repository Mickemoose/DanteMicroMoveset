#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_imports,
	unused_macros,
	unused_variables,
	unused_assignments,
	unused_unsafe,
	non_upper_case_globals,
	non_snake_case,
    clippy::borrow_interior_mutable_const
)]

use std::collections::HashMap;
use smash::hash40;
use param_config::*;
use smash::lib::lua_const::{FIGHTER_KIND_CLOUD};
use csk::*;

mod cloud;

pub static mut MARKED_COLORS: [bool; 256] = [false; 256];
pub static mut LAST_COLOR: i32 = -1;

extern "C" fn mods_mounted(_ev: arcropolis_api::Event) {
    const FIGHTER_NAME: &str = "cloud";
    const MARKER_FILE: &str = "dante.marker";
    let mut lowest_color: i32 = -1;
    let mut marked_slots: Vec<i32> = vec![];
    for x in 0..256 {
        if let Ok(_) = std::fs::read(format!(
            "mods:/fighter/{}/model/body/c{:02}/{}",
            FIGHTER_NAME, x, MARKER_FILE
        )) {
            unsafe {
                marked_slots.push(x as _);
                MARKED_COLORS[x as usize] = true;
                if lowest_color == -1 {
                    lowest_color = x as _;
                }
            }
        }
    }


    if lowest_color == -1 {
        // if no marker exist, leave
        return;
    }

    let color_num = {
        unsafe {
            let mut index = lowest_color;
            while index < 256 && MARKED_COLORS[index as usize] {
                index += 1;
            }
            LAST_COLOR = index - 1;
            index - lowest_color
        }
    };

    add_series_db_entry_info(SeriesDatabaseEntry {
        ui_series_id: hash40("ui_series_devil_may_cry"),
        name_id: StringType::Overwrite(CStrCSK::new("devil_may_cry")),
        disp_order: SignedByteType::Overwrite(0),
        disp_order_sound: SignedByteType::Overwrite(0),
        save_no: SignedByteType::Overwrite(0),
        shown_as_series_in_directory: BoolType::Overwrite(false),
        is_dlc: BoolType::Overwrite(false),
        is_patch: BoolType::Overwrite(false),
        dlc_chara_id: Hash40Type::Overwrite(0),
        is_use_amiibo_bg: BoolType::Overwrite(false),
        ..Default::default()
    });

}

#[skyline::main(name = "dante_micromoveset")]
pub fn main() {
    unsafe {
        extern "C" {
            fn arcrop_register_event_callback(
                ty: arcropolis_api::Event,
                callback: arcropolis_api::EventCallbackFn,
            );
        }
        arcrop_register_event_callback(arcropolis_api::Event::ModFilesystemMounted, mods_mounted);
    }
	println!("Dante Micro Moveset loaded!");
    cloud::install();
}