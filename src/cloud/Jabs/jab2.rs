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


//JAB2///
unsafe extern "C" fn game_specialn2_ex(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 1.0);
        //KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 0.0, y: 0.3, z: 0.0});
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 361, 50, 0, 30, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(24.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -3, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(44.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(89.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
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
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.5, 361, 50, 0, 30, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(24.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -3, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(44.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.5, 361, 0, 0, 0, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(89.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
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
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 50, 0, 30, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(24.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -3, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(44.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.5, 361, 0, 0, 0, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(89.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("dante_gun_hit2"), Hash40::new("se_dante_special_n02_01"));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
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
        //ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
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

pub fn install() {
    // Status
    // Motion
    Agent::new("cloud")
        .game_acmd("game_specialn2_ex_dante", game_specialn2_ex, Default)
        .effect_acmd("effect_specialn2_ex_dante", effect_specialn2_ex, Default)
        .expression_acmd("expression_specialn2_ex_dante", expression_specialn2_ex, Default)
        .sound_acmd("sound_specialn2_ex_dante", sound_specialn2_ex, Default)
        .install();
}
