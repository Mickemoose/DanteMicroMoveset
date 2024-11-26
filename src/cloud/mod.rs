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

use crate::MARKED_COLORS;

//const FIGHTER_DANTE_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID : i32 = 0x20000220;
//const FIGHTER_DANTE_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP : i32 = 0x20000221;
//const FIGHTER_DANTE_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS : i32 = 0x20000222;
//const FIGHTER_DANTE_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N : i32 = 0x20000223;
//const FIGHTER_DANTE_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON : i32 = 0x20000224;
//const FIGHTER_DANTE_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY : i32 = 0x20000225;
//const FIGHTER_DANTE_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER : i32 = 0x20000226;

unsafe extern "C" fn dante_specialn1_ex(agent: &mut L2CAgentBase) {

    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 0, 0, 0, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -3, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 0, 0, 0, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("dante_gun_hit2"), Hash40::new("se_dante_special_n02_01"));
    }
    
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 0, 0, 0, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -3, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 0, 0, 0, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("dante_gun_hit2"), Hash40::new("se_dante_special_n02_01"));
    }
    
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 0, 0, 0, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -3, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 0, 0, 0, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("dante_gun_hit2"), Hash40::new("se_dante_special_n02_01"));
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn dante_effect_specialn1_ex(agent: &mut L2CAgentBase) {
    let is_facing_right = PostureModule::lr(agent.module_accessor) == 1.0;
    let mut y_position = 0.0;
    if is_facing_right {
        y_position = -5.0;
    } else {
        y_position = 5.0;
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 4, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("dante_gun_muzzle"), Hash40::new("havel"), 0, y_position, 0, 0, 0, 0, 1, 2, 1, 0.5, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 4, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("dante_gun_muzzle"), Hash40::new("havel"), 01, y_position, 0, 0, 0, 0, 1, 2, 1, 0.5, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 4, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("dante_gun_muzzle"), Hash40::new("havel"), 0, y_position, 0, 0, 0, 0, 1, 2, 1, 0.5, 0, 0, 0, true);
    }
}

unsafe extern "C" fn dante_sound_specialn1_ex(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_jack_rnd_special_n02_01"));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dante_special_n06_01"));
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dante_special_n06_02"));
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dante_special_n06_03"));
    }
}

unsafe extern "C" fn dante_expression_specialn1_ex(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_63_bullet"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_63_bullet"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_63_bullet"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//JAB2///
unsafe extern "C" fn game_specialn2_ex(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 1.0);
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 0.0, y: 0.3, z: 0.0});
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 361, 0, 0, 0, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -3, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("dante_gun_hit2"), Hash40::new("se_dante_special_n02_03"));
    }
    
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.5, 361, 0, 0, 0, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -3, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.5, 361, 0, 0, 0, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("dante_gun_hit2"), Hash40::new("se_dante_special_n02_02"));
    }
    
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -3, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.5, 361, 0, 0, 0, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("dante_gun_hit2"), Hash40::new("se_dante_special_n02_01"));
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_specialn2_ex(agent: &mut L2CAgentBase) {
    let is_facing_right = PostureModule::lr(agent.module_accessor) == 1.0;
    let mut y_position = 0.0;
    if is_facing_right {
        y_position = -5.0;
    } else {
        y_position = 5.0;
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 4, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("dante_gun_muzzle"), Hash40::new("havel"), 0, y_position, 0, 0, 0, 0, 1, 2, 1, 0.5, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 4, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("dante_gun_muzzle"), Hash40::new("havel"), 0, y_position, 0, 0, 0, 0, 1, 2, 1, 0.5, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 4, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("dante_gun_muzzle"), Hash40::new("havel"), 0, y_position, 0, 0, 0, 0, 1, 2, 1, 0.5, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_specialn2_ex(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dante_special_n06_01"));
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dante_special_n06_02"));
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dante_special_n06_03"));
    }
}

unsafe extern "C" fn expression_specialn2_ex(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_63_bullet"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_63_bullet"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_63_bullet"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}
///END JAB 2///
/// JAB 3
unsafe extern "C" fn game_specialn3_ex(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 1.0);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 361, 0, 0, 0, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(9.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(9.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("dante_gun_hit2"), Hash40::new("se_dante_special_n02_01"));
    }

    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 361, 0, 0, 0, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(9.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(9.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("dante_gun_hit2"), Hash40::new("se_dante_special_n02_01"));
    }
    
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 361, 0, 0, 0, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("dante_gun_hit2"), Hash40::new("se_dante_special_n02_01"));
    }
    
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_specialn3_ex(agent: &mut L2CAgentBase) {
    let is_facing_right = PostureModule::lr(agent.module_accessor) == 1.0;
    let mut y_position = 0.0;
    if is_facing_right {
        y_position = -5.0;
    } else {
        y_position = 5.0;
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 4, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("dante_gun_muzzle"), Hash40::new("havel"), 0, y_position, 0, 0, 0, 0, 1, 2, 1, 0.5, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 4, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("dante_gun_muzzle"), Hash40::new("havel"), 0, y_position, 0, 0, 0, 0, 1, 2, 1, 0.5, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 4, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("dante_gun_muzzle"), Hash40::new("havel"), 0, y_position, 0, 0, 0, 0, 1, 2, 1, 0.5, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_specialn3_ex(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dante_special_n06_03"));
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dante_special_n06_01"));
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dante_special_n06_02"));
    }
}

unsafe extern "C" fn expression_specialn3_ex(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_63_bullet"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_63_bullet"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_63_bullet"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}
/// END JAB 3
/// 
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
                    WorkModule::set_int(boma, 90, FIGHTER_INSTANCE_WORK_ID_INT_STYLE_TIMER);
                    // ^ 60 for Mario & Luigi-specific version ^

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
            _EFFECT_COUNTER += 1;
            if _EFFECT_COUNTER == 10 {
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_superstar"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1, true, 0.6);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_status_attack_up"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.0, true, 1);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.5, 1.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_status_attack_up"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1.0, true, 1);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.5, 1.0);
            }
            if _EFFECT_COUNTER >= 20 {
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_status_attack_up"), false, false);
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_superstar"), false, false);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_superstar"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1, true, 0.6);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_status_attack_up"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.0, true, 1);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.5, 1.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_status_attack_up"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1.0, true, 1);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.5, 1.0);
                _EFFECT_COUNTER = 0;
            }
        }
    }
}

pub fn install() {
    let agent = &mut smashline::Agent::new("cloud");
    //install aerials
    Aerials::install();
    Tilts::install();
    Agent::new("cloud")
		.game_acmd("game_specialn1_ex_dante", dante_specialn1_ex, Default)
        .effect_acmd("effect_specialn1_ex_dante", dante_effect_specialn1_ex, Default)
        .expression_acmd("expression_specialn1_ex_dante", dante_expression_specialn1_ex, Default)
        .sound_acmd("sound_specialn1_ex_dante", dante_sound_specialn1_ex, Default)
        .game_acmd("game_specialn2_ex_dante", game_specialn2_ex, Default)
        .effect_acmd("effect_specialn2_ex_dante", effect_specialn2_ex, Default)
        .expression_acmd("expression_specialn2_ex_dante", expression_specialn2_ex, Default)
        .sound_acmd("sound_specialn2_ex_dante", sound_specialn2_ex, Default)
        .game_acmd("game_specialn3_ex_dante", game_specialn3_ex, Default)
        .effect_acmd("effect_specialn3_ex_dante", effect_specialn3_ex, Default)
        .expression_acmd("expression_specialn3_ex_dante", expression_specialn3_ex, Default)
        .sound_acmd("sound_specialn3_ex_dante", sound_specialn3_ex, Default)
        .on_line(Main, dante_frame)
        .install();



}
