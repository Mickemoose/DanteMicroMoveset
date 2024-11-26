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

/// DOWN AERIAL ///
unsafe extern "C" fn game_specialairndown(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    //macros::FT_MOTION_RATE(agent, 0.5);
    for _ in 0..20 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 50, 80, 0, 25, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.2, 45, 0, 0, 29, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-2.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.2, 55, 0, 0, 29, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 1.2, 45, 0, 0, 29, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(2.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 5, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 5, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 5, 4, Hash40::new("dante_gun_hit2"), Hash40::new("se_dante_special_n02_01"));
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 3, 5, 4);
        }
        
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_63_bullet"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.6, 50, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.8, 45, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-2.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.8, 55, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.8, 45, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(2.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 5, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 5, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 5, 4, Hash40::new("dante_gun_hit2"), Hash40::new("se_dante_special_n02_01"));
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 3, 5, 4);
        }
        
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_63_bullet"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 6.0);
    }
}

unsafe extern "C" fn effect_specialairndown(agent: &mut L2CAgentBase) {
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_UNSYNC_VIS(agent, Hash40::new("dante_gun_muzzle"), Hash40::new("havel"), 0, -5.0, 1.9, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 2);
    }
    wait(agent.lua_state_agent, 7.0);
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_UNSYNC_VIS(agent, Hash40::new("dante_gun_muzzle"), Hash40::new("havel"), 0, -5.0, 1.9, 0, 0, 0, 1, true);
    }
    wait(agent.lua_state_agent, 14.0);
}

unsafe extern "C" fn sound_specialairndown(agent: &mut L2CAgentBase) {
    
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dante_special_n06_01"));
    }
    
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_04"));
    }
    frame(agent.lua_state_agent, 8.0);
    for _ in 0..19 {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_dante_special_n06_02"));
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_swing_04"));
        }
        wait(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_dante_special_n06_03"));
        }
        wait(agent.lua_state_agent, 7.0);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dante_special_n05"));
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_04"));
    }
    frame(agent.lua_state_agent, 11.0);
    for _ in 0..19 {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_swing_04"));
        }
        wait(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_dante_special_n05"));
        }
        wait(agent.lua_state_agent, 10.0);
    }
}

unsafe extern "C" fn expression_specialairndown(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
}

//END DOWN AERIAL//

pub fn install() {
    // Status
    // Motion
    Agent::new("cloud")
    .game_acmd("game_specialndown_dante", game_specialairndown, Default)
    .effect_acmd("effect_specialndown_dante", effect_specialairndown, Default)
    .sound_acmd("sound_specialndown_dante", sound_specialairndown, Default)
    .expression_acmd("effect_specialndown_dante", expression_specialairndown, Default)
    .install();
}
