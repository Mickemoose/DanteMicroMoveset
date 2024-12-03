use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::{*, Priority::*}
};

mod Aerials;
mod Tilts;
mod Jabs;

use crate::MARKED_COLORS;

//const FIGHTER_DANTE_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID : i32 = 0x20000220;
//const FIGHTER_DANTE_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP : i32 = 0x20000221;
//const FIGHTER_DANTE_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS : i32 = 0x20000222;
//const FIGHTER_DANTE_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N : i32 = 0x20000223;
//const FIGHTER_DANTE_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON : i32 = 0x20000224;
//const FIGHTER_DANTE_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY : i32 = 0x20000225;
//const FIGHTER_DANTE_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER : i32 = 0x20000226;

const FIGHTER_INSTANCE_WORK_ID_FLAG_STYLE_EFFECT_SPAWNED : i32 = 0x20000226;
const FIGHTER_INSTANCE_WORK_ID_INT_STYLE_TIMER : i32 = 0x1000022A;
const FIGHTER_INSTANCE_WORK_ID_INT_STYLE_COUNTER : i32 = 0x1000022B;
const FIGHTER_INSTANCE_WORK_ID_FLAG_STYLE_AURA_ACTIVE : i32 = 0x20000227;
static mut _EFFECT_COUNTER: u32 = 0;

unsafe extern "C" fn dante_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = fighter.module_accessor;
        let status_kind = StatusModule::status_kind(boma);
        let status_frame = fighter.global_table[0xe].get_f32();
        let motion_kind = MotionModule::motion_kind(boma);
        let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        
        if crate::MARKED_COLORS[color as usize] {
            let scale = ModelModule::scale(fighter.module_accessor);
            let default_scale = WorkModule::get_param_float(fighter.module_accessor, hash40("scale"), 0);
    
            // Apply scaling effect if the scale matches the default
            if scale == default_scale {
                ModelModule::set_scale(fighter.module_accessor, 1.07);
                AttackModule::set_attack_scale(fighter.module_accessor, 1.07, true);
                GrabModule::set_size_mul(fighter.module_accessor, 1.07);
            }
            // Decrease combo timer every frame
            if ![
                *FIGHTER_STATUS_KIND_CATCH_ATTACK,
                *FIGHTER_STATUS_KIND_CATCH_DASH,
                *FIGHTER_STATUS_KIND_CATCH_DASH_PULL,
                *FIGHTER_STATUS_KIND_CATCH_PULL,
                *FIGHTER_STATUS_KIND_CATCH_TURN,
                *FIGHTER_STATUS_KIND_CATCH_WAIT,
            ].contains(&status_kind) {
                WorkModule::dec_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_STYLE_TIMER);
            }
            if WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_STYLE_TIMER) <= 0 {
                WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_STYLE_COUNTER);
            }

            // Spawn combo effect & sound
            if !WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_STYLE_EFFECT_SPAWNED) {
                if ([
                    *FIGHTER_STATUS_KIND_APPEAL,
                    *FIGHTER_STATUS_KIND_ATTACK,
                    *FIGHTER_STATUS_KIND_ATTACK_100,
                    *FIGHTER_STATUS_KIND_ATTACK_AIR,
                    *FIGHTER_STATUS_KIND_ATTACK_DASH,
                    *FIGHTER_STATUS_KIND_ATTACK_HI3,
                    *FIGHTER_STATUS_KIND_ATTACK_HI4,
                    *FIGHTER_STATUS_KIND_ATTACK_LW3,
                    *FIGHTER_STATUS_KIND_ATTACK_LW4,
                    *FIGHTER_STATUS_KIND_ATTACK_S3,
                    *FIGHTER_STATUS_KIND_ATTACK_S4,
                    *FIGHTER_STATUS_KIND_CLIFF_ATTACK,
                    *FIGHTER_STATUS_KIND_LADDER_ATTACK,
                    *FIGHTER_STATUS_KIND_SPECIAL_HI,
                    *FIGHTER_STATUS_KIND_SPECIAL_LW,
                    *FIGHTER_STATUS_KIND_SPECIAL_N,
                    *FIGHTER_STATUS_KIND_SPECIAL_S,
                ].contains(&status_kind)
                && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT))
                || (status_kind == *FIGHTER_STATUS_KIND_THROW 
                && !CatchModule::is_catch(boma)) {
                    WorkModule::on_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_STYLE_EFFECT_SPAWNED);
                    WorkModule::inc_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_STYLE_COUNTER);
                    WorkModule::set_int(boma, 100, FIGHTER_INSTANCE_WORK_ID_INT_STYLE_TIMER);

                    let combo_counter = WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_STYLE_COUNTER);
                    if combo_counter == 3 {
                        macros::PLAY_SE(fighter, Hash40::new("se_dmc_rank_d"));
                        macros::EFFECT(fighter, Hash40::new("dmc_rank_d"), Hash40::new("head"), 0, 10, 0, 0, 0, 0, 3.0, 0, 0, 0, 0, 0, 0, false);
                    } else if combo_counter == 6 {
                        macros::PLAY_SE(fighter, Hash40::new("se_dmc_rank_c"));
                        macros::EFFECT(fighter, Hash40::new("dmc_rank_c"), Hash40::new("head"), 0, 10, 0, 0, 0, 0, 3.0, 0, 0, 0, 0, 0, 0, false);
                    } else if combo_counter == 9 {
                        macros::PLAY_SE(fighter, Hash40::new("se_dmc_rank_b"));
                        macros::EFFECT(fighter, Hash40::new("dmc_rank_b"), Hash40::new("head"), 0, 10, 0, 0, 0, 0, 3.0, 0, 0, 0, 0, 0, 0, false);
                    } else if combo_counter == 12 {
                        macros::PLAY_SE(fighter, Hash40::new("se_dmc_rank_a"));
                        macros::EFFECT(fighter, Hash40::new("dmc_rank_a"), Hash40::new("head"), 0, 10, 0, 0, 0, 0, 3.0, 0, 0, 0, 0, 0, 0, false);
                    } else if combo_counter == 15 {
                        WorkModule::on_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_STYLE_AURA_ACTIVE);
                        macros::PLAY_SE(fighter, Hash40::new("se_dmc_rank_s"));
                        macros::EFFECT(fighter, Hash40::new("dmc_rank_s"), Hash40::new("head"), 0, 10, 0, 0, 0, 0, 3.0, 0, 0, 0, 0, 0, 0, false);
                    } else if combo_counter == 18 {
                        macros::PLAY_SE(fighter, Hash40::new("se_dmc_rank_ss"));
                        macros::EFFECT(fighter, Hash40::new("dmc_rank_ss"), Hash40::new("head"), 0, 10, 0, 0, 0, 0, 3.0, 0, 0, 0, 0, 0, 0, false);
                    } else if combo_counter == 21 {
                        macros::PLAY_SE(fighter, Hash40::new("se_dmc_rank_sss"));
                        macros::EFFECT(fighter, Hash40::new("dmc_rank_sss"), Hash40::new("head"), 0, 10, 0, 0, 0, 0, 3.0, 0, 0, 0, 0, 0, 0, false);
                    }
                }
                
            }
            
            // Reset effect spawned flag
            if status_frame <= 1.0 {
                WorkModule::off_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_STYLE_AURA_ACTIVE);
                WorkModule::off_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_STYLE_EFFECT_SPAWNED);
            }

            if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK
            || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_AIR
            {
                // Show gun
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("dante_gun"), true);
            } else {
                // Hide gun
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("dante_gun"), false);
            }
        }
        if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_STYLE_AURA_ACTIVE)  {
            AttackModule::set_power_up(fighter.module_accessor, 1.25);
            DamageModule::set_damage_mul(fighter.module_accessor, 0.8);
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
    }
}

pub fn install() {
    let agent = &mut smashline::Agent::new("cloud");
    //install aerials
    Aerials::install();
    Tilts::install();
    Jabs::install();
    Agent::new("cloud")
        .on_line(Main, dante_frame)
        .install();
}
