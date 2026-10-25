//! `src/camera/cameracontroller.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::at as at_2;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_2;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::wscript::wscriptman::p;
use crate::guest::Addr;
use callee::*;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0015_cba0, cam_controller_ctor, "CamController_ctor"),
    (0x0015_cc10, cam_controller_dtor, "CamController_dtor"),
    (0x0015_ccf0, cam_controller_reset, "CamController_Reset"),
    (
        0x0015_cd80,
        manual_cam_controller_ctor,
        "ManualCamController_ctor",
    ),
    (
        0x0015_ce10,
        manual_cam_controller_dtor,
        "ManualCamController_dtor",
    ),
    (
        0x0015_ce98,
        manual_cam_controller_reset_for_rider,
        "ManualCamController_ResetForRider",
    ),
    (
        0x0015_cee8,
        manual_cam_controller_reset,
        "ManualCamController_Reset",
    ),
    (
        0x0015_cf28,
        cam_controller_set_algorithm,
        "CamController_SetAlgorithm",
    ),
    (
        0x0015_d050,
        chase_cam_controller_set_algorithm_immediate,
        "ChaseCamController_SetAlgorithmImmediate",
    ),
    (
        0x0015_d078,
        chase_cam_controller_set_algorithm,
        "ChaseCamController_SetAlgorithm",
    ),
];

// ---------------------------------------------------------------------------
// Callees (addresses the generated body reaches through the function table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::be::beintmission::callee::BUY_ATTRIB_UPDATE_BARS;
    pub(crate) use crate::game::be::beintmission::callee::CAMERA_TRIGGER_EMPTY_STUB;
    pub(crate) use crate::game::be::beintmission::callee::CAREER_STATS_SHOW_HIGHLIGHTS;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM;
    pub(crate) use crate::game::be::beintmission::callee::MEMCARD_SCAN_STEP;
    pub(crate) use crate::game::be::beintmission::callee::MISSION_COUNT;
    pub(crate) use crate::game::be::beintmission::callee::MISSION_NTH;
    pub(crate) use crate::game::be::beintmission::callee::MPEG2_ERROR;
    pub(crate) use crate::game::be::beintmission::callee::NET_FE_STATE_ON_WIDGET_ACTION;
    pub(crate) use crate::game::be::beintmission::callee::REWARDS_FILE_LOAD;
    pub(crate) use crate::game::be::beintmission::callee::UBER_TRICK_REFRESH;
    pub(crate) use crate::game::be::beintmission::callee::UITEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::be::beintmission::callee::VIEW_POPUP_SHOW_MESSAGE;
    pub(crate) use crate::game::be::beintmission::callee::WORLD_OBJ_RUN_KILL_SCRIPT;
    pub(crate) use crate::game::camera::camera::callee::BEBAGT_RESET_PROFILE;
    pub(crate) use crate::game::camera::camera::callee::BENEWPLAYER_GET_SLOT;
    pub(crate) use crate::game::camera::camera::callee::BE_LIBRARY_RESET_PROFILE;
    pub(crate) use crate::game::camera::camera::callee::BE_OPTION_GET_MAX_HANDICAP;
    pub(crate) use crate::game::camera::camera::callee::CAMERA_TRIGGER_FREE_TRIGGERS;
    pub(crate) use crate::game::camera::camera::callee::CAMERA_TRIGGER_RESET_HEADER;
    pub(crate) use crate::game::camera::camera::callee::CAM_CONTROLLER_RESET;
    pub(crate) use crate::game::camera::camera::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::camera::camera::callee::FE_REFRESH_PROFILE_LIST;
    pub(crate) use crate::game::camera::camera::callee::FE_REFRESH_SLOTS;
    pub(crate) use crate::game::camera::camera::callee::MANUAL_CAM_ALGO_BASE_CTOR;
    pub(crate) use crate::game::camera::camera::callee::MEASURE_ROW;
    pub(crate) use crate::game::camera::camera::callee::MEMCARD_SET_STATE;
    pub(crate) use crate::game::camera::camera::callee::SPHERE_TREE_RESET_TRANSFORM;
    pub(crate) use crate::game::camera::camera::callee::TRANSITION_MAN_START;
    pub(crate) use crate::game::camera::camera::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::camera::camera::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::camera::camera::callee::WSCRIPT_DELETE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::ACTIVE_CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_MUSIC_TIER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_TRACK_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_APP;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_BE_2 as BE_GET_BE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_INTERFACE_GET_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_CLEAR_EMITTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_INIT_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_COUNT_2 as BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_ENTRY_2 as BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_SET_OWNER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_SET_IN_GAME_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_STACK_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBE_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_FILL_PLAYER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_GET_CHARACTER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_INIT_FROM_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_RESET_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CUI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::EDITOR_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_CREATED_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_ERROR_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_SHOW_ROOM_TAB;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_UPDATE_PLAYERS_IN_ROOM;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_MANAGER_ADD_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_READ_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_BUILD_MESH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_SPRINGS_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_UPDATE_VERTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_ANIM_MAP_SET_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODE_MAN_RESET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_REQUEST_STATE_5 as GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_RESET_RACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_START_NEXT_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_PROCESS_DTOR_2 as GAME_PROCESS_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_FINISH_CLEAR_FLAG40;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_FLAG_MANAGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_NEXT_SECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_HIDE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_REQUEST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SHOW;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_SELECT_ALGORITHM;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_EMPTY_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_IS_FLAG_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_2 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_10 as NET_FE_STATE_ON_CHILD_EVENT_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_9 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP_2 as NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_UPDATE_3 as NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_MESSAGE_CENTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_MEM_CARD_CLEAR_DIR_CACHE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_PAD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SET_INPUT_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ANIM_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_3 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_DATA_GET_BANK_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_3 as SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_KILL_ALL_PROCESSES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS_2 as SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE_2 as SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_OBJ_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS_2 as SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_UNREGISTER_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_2 as SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_2 as SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_3 as SCRIPT_OBJ_ACTIVATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CAM_BUILD_MATRIX_FROM_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_SOUND_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ON_ACTIVATED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SECTION_TABLE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_LOAD_BANK_FROM_MEMORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_FREE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_AABB_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COMPUTE_OFFSET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_5 as SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_3 as SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME_2 as UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_2 as UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST as UI_LIST_FIRST_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_4 as UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_3 as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_ADD_CHILD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM_2 as UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_2 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_4 as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_2 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_BLOCK_POOL_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_STREAMER_UPDATE_VIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_MEMMAN_CREATE_SECTION_ALLOCATOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2_2 as WORLD_OBJ_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_3 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_NODE_DTOR;
    pub(crate) use crate::game::render::particle::callee::ALGORITHM_LIST_DTOR;
    pub(crate) use crate::game::render::particle::callee::ANIM_OBJECT_BASE_DTOR;
    pub(crate) use crate::game::render::particle::callee::AVA_SPLINE_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::BANK_MONITOR_INSTANCE_PAUSE;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::render::particle::callee::BENEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::render::particle::callee::BENEW_RACE_INT_IS_STATION_TRACK_BY_INDEX;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_COLLECTIBLE_COLLECTED;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_COPY;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_CONCAT_IN_PLACE;
    pub(crate) use crate::game::render::particle::callee::CFE_STATE_CHAR_SELECT_UPDATE_ATTRIBUTE_BARS;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAMERA_ALGORITHM_CTOR as CHASE_CAMERA_ALGORITHM_CTOR_2;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE;
    pub(crate) use crate::game::render::particle::callee::CPS2_CHUNK_RESOLVE_DTOR_STUB;
    pub(crate) use crate::game::render::particle::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::render::particle::callee::CROWD_MAN3D_GROUP_REMOVE_SECTION;
    pub(crate) use crate::game::render::particle::callee::CUT_SCENE_TRANSITION_SAVE_VIEWPORT;
    pub(crate) use crate::game::render::particle::callee::EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::render::particle::callee::FLEX_BRIDGE_SPRINGS_SAVE;
    pub(crate) use crate::game::render::particle::callee::INI_PARSER_SKIP_TO_EOL;
    pub(crate) use crate::game::render::particle::callee::ISIGNAL_SEMA;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_POP;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_SELECT;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_DRAW_ROW_2 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::render::particle::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER;
    pub(crate) use crate::game::render::particle::callee::PATH_POST_EVENT;
    pub(crate) use crate::game::render::particle::callee::PATH_SET_PAUSED;
    pub(crate) use crate::game::render::particle::callee::POSITION_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_FINISH_PRESS;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_SETTLE_SLEW;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_FINISH_BOARD_PRESS;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_DELETE;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_MKDIR;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_READ;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_SEEK;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_WRITE;
    pub(crate) use crate::game::render::particle::callee::SNDSTRM_SET_PITCH;
    pub(crate) use crate::game::render::particle::callee::SND_LOCK;
    pub(crate) use crate::game::render::particle::callee::SND_SPU_MEM_SET_BASE;
    pub(crate) use crate::game::render::particle::callee::SND_UNLOCK;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_MS_TO_NEXT_BEAT;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_STREAM_HANDLE;
    pub(crate) use crate::game::render::particle::callee::STRLEN;
    pub(crate) use crate::game::render::particle::callee::TEX_MAN_LOAD_TEXTURE as TEX_MAN_LOAD_TEXTURE_2;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_PUSH_BACK_2 as UI_LIST_PUSH_BACK;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_RESET;
    pub(crate) use crate::game::render::particle::callee::WORLD_CREATE_DEAD_NODE;
    pub(crate) use crate::game::render::particle::callee::WORLD_OBJ_SAVE;
    pub(crate) use crate::game::wscript::wscriptman::callee::ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::wscript::wscriptman::callee::APP_MODULE_UPDATE_FADE;
    pub(crate) use crate::game::wscript::wscriptman::callee::AVALANCHE_BAKE_NODE_HISTORY;
    pub(crate) use crate::game::wscript::wscriptman::callee::AV_PLAYER_INIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_POLL_FREE;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_SET_PITCH;
    pub(crate) use crate::game::wscript::wscriptman::callee::CAM_CONTROLLER_CLEAR_ALGORITHMS;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_CHUNK_RESOLVE_CTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_MEMCARD_SET_STATE;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_CREATE_MESHES_2 as CROWD_MAN3D_GROUP_CREATE_MESHES;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_FREE_MESHES;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FLAG_MANAGER_GROUP_LOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_SETUP_MAN_SEND_QUIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::DEBUG_MENU_MODULE_UPDATE_CURSOR_TRAIL;
    pub(crate) use crate::game::wscript::wscriptman::callee::DISABLE_INTC;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_FILE_MOUNT_BIG;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLEX_RAIL_UPDATE_SPACE_NODE;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLOATING_V49;
    pub(crate) use crate::game::wscript::wscriptman::callee::FOCUS_MAN_ADD_FOCUS_BOX;
    pub(crate) use crate::game::wscript::wscriptman::callee::LIGHT_MAN_EMPTY_HOOK_A;
    pub(crate) use crate::game::wscript::wscriptman::callee::MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::MOD_OBJ_DTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_CTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::wscript::wscriptman::callee::RAIL_MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::REMOVE_INTC_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::SET_FADE_GAIN;
    pub(crate) use crate::game::wscript::wscriptman::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::TEX_MAN_LOAD_TEXTURE;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_GET_STATE_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SET_USER_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::wscript::wscriptman::callee::WSCRIPT_MAN_CALL_FUNC;

    pub const CAM_CONTROLLER_RESET_FOR_RIDER: Callee = Callee {
        addr: 0x0015_cc70,
        f: ee::s01::f_0015cc70,
    };

    /// `ChaseCamController_PushAlgorithm` (0x161e58).
    pub const CHASE_CAM_PUSH_ALGORITHM: Callee = Callee {
        addr: 0x0016_1e58,
        f: ee::s01::f_00161e58,
    };
}

// ---------------------------------------------------------------------------
// Camera algorithms.

/// `ChaseCamController_SetAlgorithmImmediate(Ctrl* c, int algo, void* arg)`
/// (0x15d050): `SetAlgorithm(c, algo, arg, 1.0f)`.
pub fn chase_cam_controller_set_algorithm_immediate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 12);
        frame.save_ra(c, 0);
        callee::CHASE_CAM_SET_ALGORITHM.call(c, 0x0015_d068)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cMemMan_alloc` (0x317d70, another shard).
const MEM_ALLOC: u32 = 0x0031_7d70;
/// The algorithm id jump table (0x62 entries).
const ALGORITHM_TABLE: u32 = 0x0045_b5c0;
const ALGORITHM_COUNT: u64 = 0x62;
/// The block every case ends in: activate the algorithm and push it.
const SET_ALGORITHM_PUSH: u32 = 0x0015_d628;
/// `ChaseCamController::flags` (+0x30): bit 1 makes every switch instant.
const CHASE_CAM_FLAGS: u32 = 0x30;
const CHASE_CAM_RIDER: u32 = 0x20;

/// Cases that allocate `size` bytes and run a constructor on them:
/// (case, cycles, size, name offset below 0x460000, constructor).
const PLAIN_ALGORITHMS: &[(u32, u64, u64, u32, u32)] = &[
    (0x0015_d23c, 10, 0x390, 0x4cd8, 0x0016_16a8), // IdleChaseCameraAlgorithm
    (0x0015_d264, 10, 0x390, 0x4cb8, 0x0016_89c8),
    (0x0015_d28c, 10, 0x390, 0x4c90, 0x0016_8cd8),
    (0x0015_d2b4, 10, 0x390, 0x4c70, 0x0016_0438), // ManualChaseCameraAlgorithm
    (0x0015_d2dc, 10, 0x390, 0x4c50, 0x0016_82b8),
    (0x0015_d430, 10, 0x390, 0x4af0, 0x0016_8650),
    (0x0015_d458, 10, 0x3a0, 0x4ac8, 0x0016_0760), // MatrixChaseCameraAlgorithm
    (0x0015_d4b8, 10, 0x390, 0x4d28, 0x0017_6a68),
    (0x0015_d4e0, 10, 0x390, 0x4d00, 0x0017_7068),
    (0x0015_d508, 10, 0x390, 0x4d50, 0x0017_6d68),
    (0x0015_d530, 10, 0x3a0, 0x4a88, 0x0017_7e50),
];

/// Cases of the 0x1774c0 algorithm, one per variant 0..7:
/// (case, cycles, name offset below 0x460000).
const VARIANT_ALGORITHMS: &[(u32, u64, u32)] = &[
    (0x0015_d304, 9, 0x4c30),
    (0x0015_d328, 9, 0x4c08),
    (0x0015_d34c, 9, 0x4be0),
    (0x0015_d370, 9, 0x4bb8),
    (0x0015_d394, 9, 0x4b90),
    (0x0015_d3b8, 9, 0x4b68),
    (0x0015_d3dc, 9, 0x4b40),
    (0x0015_d400, 8, 0x4b18),
];

/// `RelativeChaseCameraAlgorithm` cases with a fixed distance from the
/// game's constants: (case, cycles, offset below gp).
const RELATIVE_ALGORITHMS: &[(u32, u64, u32)] = &[
    (0x0015_d58c, 9, 0x6cd0),
    (0x0015_d5b0, 9, 0x6ccc),
    (0x0015_d5d4, 9, 0x6cc8),
    (0x0015_d5f8, 8, 0x6cc4),
];

/// `cMemMan_alloc(size, 0x460000 - name, 0x20000000, 0)`: the algorithm's
/// memory, named after its class.
fn alloc_algorithm(c: &mut Cpu, size: u64, name: u32, ret: u32) -> Flow {
    c.g[A0][0] = size;
    c.g[A1][0] = sx(0x46_0000u32.wrapping_sub(name));
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    call_ptr(c, MEM_ALLOC, ret)
}

/// `void ChaseCamController_SetAlgorithm(ChaseCamController* cam, int id,
/// void* params, float rate f12)` (0x15d078): build camera algorithm `id`
/// (0..0x61, through its jump table; other ids push a null algorithm) and
/// push it onto the controller's blend list. `rate` is the per-tick blend
/// step the list node keeps; bit 1 of the flags (+0x30) makes it 1.0
/// (instant). The rate is the caller's: converted callers pass a step
/// already scaled to the tick rate (`camera::blend_rate_60`), mission and
/// script data keep their authored rates, so it is forwarded unchanged.
pub fn chase_cam_controller_set_algorithm(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xb0);
        frame.save(c, S1, 0xa0);
        c.g[S0][0] = 0;
        frame.save(c, S5, 0x60);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S6, 0x50);
        c.g[S5][0] = c.g[A2][0];
        frame.save_fpr(c, 21, 0xc8);
        c.g[S6][0] = c.g[A0][0];
        frame.save(c, S2, 0x90);
        frame.save(c, S3, 0x80);
        frame.save(c, S4, 0x70);
        frame.save_ra(c, 0x40);
        frame.save_fpr(c, 20, 0xc0);
        let flags = Addr((c.g[S6][0] as u32).wrapping_add(CHASE_CAM_FLAGS)).lw(c);
        c.g[V0][0] = flags & 2;
        let instant = c.g[V0][0] != 0;
        c.mov_s(21, 12);
        if instant {
            c.cycles += 2;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 21);
        }

        c.cycles += 3;
        let in_table = c.g[S1][0] < ALGORITHM_COUNT;
        c.g[V0][0] = 0x46_0000;
        if in_table {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[V0][0] = sx(ALGORITHM_TABLE);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(ALGORITHM_TABLE));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let case = c.g[A0][0] as u32;
            if !build_algorithm(c, case)? {
                c.jump_out(case, 0x0015_d078, 0x0015_d694);
                return Ok(());
            }
        }

        // 0x15d628: start the algorithm (vtable slot +0x60, with the rider),
        // push it with the rate, then its slot +0x20.
        c.cycles += 27;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(CHASE_CAM_RIDER)).lw(c);
        let start = VSlot::read(c, c.g[V0][0], 0x60);
        c.g[V1][0] = start.func;
        c.g[A0][0] = start.this(c.g[S0][0]);
        call_ptr(c, start.func as u32, 0x0015_d640)?;
        c.g[A0][0] = c.g[S6][0];
        c.mov_s(12, 21);
        c.g[A1][0] = c.g[S0][0];
        callee::CHASE_CAM_PUSH_ALGORITHM.call(c, 0x0015_d650)?;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x10)).lw(c);
        let pushed = VSlot::read(c, c.g[V0][0], 0x20);
        c.g[V1][0] = pushed.func;
        c.g[A0][0] = pushed.this(c.g[S0][0]);
        call_ptr(c, pushed.func as u32, 0x0015_d664)?;

        frame.restore(c, S0, 0xb0);
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, S6, 0x50);
        frame.restore_ra(c, 0x40);
        frame.restore_fpr(c, 21, 0xc8);
        frame.restore_fpr(c, 20, 0xc0);
        frame.close(c);
        Ok(())
    })
}

/// One case of `ChaseCamController_SetAlgorithm`'s jump table: build the
/// algorithm into `s0` (left 0 by the empty cases). `false` if `case` is not
/// one of the function's own blocks.
fn build_algorithm(c: &mut Cpu, case: u32) -> Result<bool, Unwound> {
    if let Some(&(_, cycles, size, name, ctor)) = PLAIN_ALGORITHMS.iter().find(|a| a.0 == case) {
        c.cycles += cycles;
        alloc_algorithm(c, size, name, case + 0x18)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, ctor, case + 0x20)?;
        c.g[S0][0] = c.g[V0][0];
        return Ok(true);
    }
    if let Some(variant) = VARIANT_ALGORITHMS.iter().position(|a| a.0 == case) {
        let (_, cycles, name) = VARIANT_ALGORITHMS[variant];
        c.cycles += cycles;
        alloc_algorithm(c, 0x3a0, name, case + 0x18)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = variant as u64;
        c.cycles += 4;
        call_ptr(c, 0x0017_74c0, 0x0015_d428)?;
        c.g[S0][0] = c.g[V0][0];
        return Ok(true);
    }
    if let Some(&(_, cycles, distance)) = RELATIVE_ALGORITHMS.iter().find(|a| a.0 == case) {
        c.cycles += cycles;
        alloc_algorithm(c, 0x3c0, 0x4aa8, case + 0x18)?;
        let at = (c.g[GP][0] as u32).wrapping_sub(distance);
        c.lwc1(12, at);
        c.g[A0][0] = c.g[V0][0];
        relative_algorithm(c)?;
        return Ok(true);
    }
    match case {
        // A track-spline algorithm (0x178520) when the params' track data
        // (+8) is a spline (type 2) and the career has 10 or more events
        // (or none), else the 0x174190 algorithm holding `params`.
        0x0015_d0f0 => {
            c.cycles += 6;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x28)).lw(c);
            c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
            c.g[A1][0] = 0;
            if c.g[A0][0] != 0 {
                c.cycles += 1;
                c.g[A1][0] = ((c.g[A0][0] as i64) < 10) as u64;
            }
            c.cycles += 2;
            let mut spline = false;
            if c.g[A1][0] == 0 {
                c.cycles += 5;
                c.g[S2][0] = Addr((c.g[S5][0] as u32).wrapping_add(8)).lw(c);
                c.g[V1][0] = 2;
                c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x28)).lw(c);
                spline = c.g[V0][0] == c.g[V1][0];
            }
            c.g[A1][0] = 0x46_0000;
            if spline {
                c.cycles += 20;
                alloc_algorithm(c, 0x3b0, 0x4db0, 0x0015_d140)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, 0x0017_0240, 0x0015_d150)?;
                c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0017_07b0, 0x0015_d160)?;
                c.g[A2][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, 0x0017_8520, 0x0015_d170)?;
                c.g[S0][0] = c.g[V0][0];
            } else {
                c.cycles += 10;
                alloc_algorithm(c, 0x90, 0x4d88, 0x0015_d18c)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0017_4190, 0x0015_d194)?;
                c.g[S0][0] = c.g[V0][0];
                Addr((c.g[S0][0] as u32).wrapping_add(0x80)).sw(c, c.g[S5][0]);
            }
        }
        // `SplineCameraAlgorithm` through the four control points of the
        // params' spline (+8), with the timing at params +0x10..+0x1c.
        0x0015_d1a0 => {
            c.cycles += 39;
            c.g[S0][0] = Addr((c.g[S5][0] as u32).wrapping_add(8)).lw(c);
            alloc_algorithm(c, 0x3d0, 0x4d68, 0x0015_d1bc)?;
            c.g[S4][0] = c.g[V0][0];
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = 0;
            call_ptr(c, 0x0017_0d20, 0x0015_d1d0)?;
            let points = [
                (S3, 0x10, 0x0015_d1e4),
                (S2, 0x20, 0x0015_d1f8),
                (S1, 0x30, 0x0015_d20c),
            ];
            for (i, &(reg, off, ret)) in points.iter().enumerate() {
                c.g[reg][0] = sx(sp(c).wrapping_add(off));
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[reg][0];
                c.g[A2][0] = i as u64 + 1;
                call_ptr(c, 0x0017_0d20, ret)?;
            }
            let params = c.g[S5][0] as u32;
            c.lwc1(15, params.wrapping_add(0x10));
            c.g[A0][0] = c.g[S4][0];
            c.lwc1(12, params.wrapping_add(0x18));
            c.g[A2][0] = c.g[S3][0];
            c.lwc1(13, params.wrapping_add(0x14));
            c.g[A3][0] = c.g[S2][0];
            c.lwc1(14, params.wrapping_add(0x1c));
            c.g[T0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x0016_1060, 0x0015_d234)?;
            c.g[S0][0] = c.g[V0][0];
        }
        // `RelativeChaseCameraAlgorithm` at (id - 0x21) times the distance
        // step at gp-0x6cd4.
        0x0015_d480 => {
            c.cycles += 14;
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x21));
            let step = (c.g[GP][0] as u32).wrapping_sub(0x6cd4);
            c.lwc1(0, step);
            c.mtc1(V0, 20);
            c.cvt_s_w(20, 20);
            c.mul_s(20, 20, 0);
            alloc_algorithm(c, 0x3c0, 0x4aa8, 0x0015_d4ac)?;
            c.g[A0][0] = c.g[V0][0];
            c.mov_s(12, 20);
            relative_algorithm(c)?;
        }
        0x0015_d558 => {
            c.cycles += 2;
            c.g[S0][0] = 0;
        }
        // `Post_Race_1ChaseCameraAlgorithm` for the controller's rider.
        0x0015_d560 => {
            c.cycles += 11;
            alloc_algorithm(c, 0x3b0, 0x4a68, 0x0015_d578)?;
            c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(CHASE_CAM_RIDER)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0017_89e8, 0x0015_d584)?;
            c.g[S0][0] = c.g[V0][0];
        }
        SET_ALGORITHM_PUSH => {}
        _ => return Ok(false),
    }
    Ok(true)
}

/// 0x15d618: `RelativeChaseCameraAlgorithm_ctor(a0, distance f12)`, then
/// keep the id at +0xc.
fn relative_algorithm(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    call_ptr(c, 0x0016_0b70, 0x0015_d620)?;
    c.g[S0][0] = c.g[V0][0];
    Addr((c.g[S0][0] as u32).wrapping_add(0xc)).sw(c, c.g[S1][0]);
    Ok(())
}

// ---------------------------------------------------------------------------
// Camera controller / algorithm constructors.

/// `void ManualCamController_Reset(this)` (0x15cee8): reset the controller and
/// then each algorithm in its list via the list's vtable slot at `+0x24`.
pub fn manual_cam_controller_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[A0][0] = this;
        callee::CAM_CONTROLLER_RESET.call(c, 0x0015_cefc)?;
        c.g[A1][0] = Addr((this as u32).wrapping_add(0x18)).lw(c);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(0x10).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x20).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x24).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0015_cf14)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ManualCamController_ResetForRider(cam, rider)` (0x15ce98): reset the
/// rider's camera controller and tell its algorithm list (slot 0x18/0x1c) to
/// follow the rider.
pub fn manual_cam_controller_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        CAM_CONTROLLER_RESET_FOR_RIDER.call(c, 0x0015_ceb4)?;
        c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x18)).lw(c);
        let cam = c.g[A2][0] as u32;
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = Addr(cam).off(0x10).lw(c);
        let vt = c.g[V0][0] as u32;
        c.g[A0][0] = Addr(vt).off(0x18).lh(c);
        c.g[V1][0] = Addr(vt).off(0x1c).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0015_ced0);
        c.g[A0][0] = sx(cam.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0015_ced0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CamController_SetAlgorithm(CamController* self, void* algo,
/// void* arg)` (0x15cf28): replace the controller's algorithm list with one
/// algorithm.
pub fn cam_controller_set_algorithm(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        let this = r32(c, A0);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        callee::CAM_CONTROLLER_CLEAR_ALGORITHMS.call(c, 0x0015_cf4c)?;
        c.g[A0][0] = Addr(p(this, 8)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[AT][0] = 0x3f80_0000; // 1.0
        c.mtc1(1, 12);
        c.g[A2][0] = c.g[S2][0];
        callee::ALGORITHM_LIST_PUSH.call(c, 0x0015_cf64)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CamController_dtor(CamController* self, int flags)` (0x15cc10):
/// destroy the algorithm list and optionally free the object.
pub fn cam_controller_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0046_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0045_b938);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(r32(c, S0)).off(0x14).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(r32(c, S0)).off(0x8).lw(c);
        let no_list = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_list {
            c.cycles += 2;
            c.g[RA][0] = sx(0x0015_cc44);
            c.g[A1][0] = 3;
            callee::ALGORITHM_LIST_DTOR.call(c, 0x0015_cc44)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 0x1;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0015_cc58);
            c.g[A0][0] = c.g[S0][0];
            callee::BUILTIN_DELETE.call(c, 0x0015_cc58)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CamController_Reset(CamController* self)` (0x15ccf0): reset every
/// camera node's override state and the current camera's chase/splash
/// managers.
pub fn cam_controller_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at(c, S1, 8)).lw(c);
        c.g[S0][0] = Addr(at(c, V0, 0)).lw(c);
        let none = c.g[S0][0] == 0;
        if !none {
            c.cycles += 1;
            c.g[A1][0] = Addr(at(c, S0, 0)).lw(c);
            loop {
                c.cycles += 8;
                c.g[V0][0] = Addr(at(c, A1, 0x10)).lw(c);
                c.g[A0][0] = Addr(at(c, V0, 0x20)).lh(c);
                c.g[V1][0] = Addr(at(c, V0, 0x24)).lw(c);
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx(at(c, A1, 0).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0015_cd2c)?;
                c.g[S0][0] = Addr(at(c, S0, 0x14)).lw(c);
                if c.g[S0][0] == 0 {
                    break;
                }
                c.g[A1][0] = Addr(at(c, S0, 0)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0015_cd18);
                }
            }
        }
        c.cycles += 9;
        c.g[A0][0] = Addr(at(c, S1, 0x10)).lw(c);
        call_ptr(c, 0x0015_e030, 0x0015_cd40)?;
        c.g[A0][0] = Addr(at(c, S1, 0x10)).lw(c);
        call_ptr(c, 0x002f_41a8, 0x0015_cd48)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cCameraController* cCameraController_ctor(cCameraController* c, int a)`
/// (0x15cba0).
pub fn cam_controller_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0045_b938;
        sqsp(c, 17, 0x10);
        sqsp(c, 16, 0x20);
        c.g[S1][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x0045_b220;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        Addr(p_2(r32(c, S0), 0x14)).sw(c, c.g[V0][0]);
        c.g[A0][0] = 8;
        call::call_ptr(c, 0x0031_7d70, 0x0015_cbdc)?;
        c.g[A0][0] = c.g[V0][0];
        call::call_ptr(c, 0x0015_c930, 0x0015_cbe4)?;
        Addr(p_2(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        Addr(p_2(r32(c, S0), 0x10)).sw(c, c.g[S1][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(p_2(r32(c, S0), 0xc)).sw(c, 0);
        Addr(c.g[S0][0] as u32).sw(c, 0);
        Addr(p_2(r32(c, S0), 4)).sw(c, 0);
        lqsp(c, 16, 0x20);
        lqsp(c, 17, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}

/// The `sDeleteTarget` sequence of the UI destructors: `v0 = lw(a2+slot)`,
/// `delta = lh(v0+8)`, `fn = lw(v0+0xc)`, `a0 = a2 + delta`, `jalr fn`. `a1`
/// (the delete kind, 3) must already be set.
pub(crate) fn delete_owner(c: &mut Cpu, slot: u32, ret: u32) -> Flow {
    c.g[V0][0] = Addr(p(r32(c, A2), slot)).lw(c);
    c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
    c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
    let target = r32(c, V1);
    c.g[RA][0] = sx(ret);
    c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
    call_ptr(c, target, ret)
}

// ---------------------------------------------------------------------------
// Camera controllers.

/// `void ManualCamController_dtor(ManualCamController* self, int flags)`
/// (0x15ce10): release the two camera controllers and run the base
/// destructor.
pub fn manual_cam_controller_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x46_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0045_b8d8);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(r32(c, S0), 0x14)).sw(c, c.g[V0][0]);
        c.g[A2][0] = Addr(p(r32(c, S0), 0x18)).lw(c);
        let null = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !null {
            c.cycles += 6;
            c.g[A1][0] = 0x3;
            delete_owner(c, 0x10, 0x0015_ce54)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p(r32(c, S0), 0x1c)).lw(c);
        let null = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !null {
            c.cycles += 5;
            delete_owner(c, 0x10, 0x0015_ce74)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0015_ce80);
        c.g[A1][0] = c.g[S1][0];
        callee::CAM_CONTROLLER_DTOR.call(c, 0x0015_ce80)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* ManualCamController_ctor(self, void* cam)` (0x15cd80): build the
/// camera controller and its two algorithms, then select the manual one.
pub fn manual_cam_controller_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x20);
        let self_ = abi::arg(c, 0);
        let cam = abi::arg64(c, 1);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(self_);
        call_ptr(c, callee::CAM_CONTROLLER_CTOR, 0x0015_cd94)?;
        c.g[V1][0] = 0x0046_0000;
        c.g[V0][0] = 1;
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x4728));
        c.g[A1][0] = 0x0046_0000;
        c.sw(at_2(c, S0, 0x14), c.g[V1][0]);
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x4dd0));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.sw(r32(c, S0), c.g[V0][0]);
        c.g[A0][0] = 0x40;
        let _ = cam;
        call_ptr(c, callee::MEM_ALLOC, 0x0015_cdc0)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::MANUAL_CAM_ALGO_CTOR, 0x0015_cdc8)?;
        c.g[A1][0] = 0x0046_0000;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x4dc0));
        c.sw(at_2(c, S0, 0x18), c.g[V0][0]);
        c.g[A0][0] = 0x40;
        call_ptr(c, callee::MEM_ALLOC, 0x0015_cde4)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::EDITOR_CAM_ALGO_CTOR, 0x0015_cdec)?;
        c.sw(at_2(c, S0, 0x1c), c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x51;
        call_ptr(c, callee::MANUAL_CAM_SELECT_ALGORITHM, 0x0015_cdfc)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fill;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::SP as STACK_TOP;
    pub(crate) use crate::game::ai::ai::tests::slot;
    pub(crate) use crate::game::ai::ai::tests::w16;
    pub(crate) use crate::game::be::bedbreward::tests::alloc_ready as alloc_ready_2;
    pub(crate) use crate::game::be::bedbreward::tests::call_setup;
    pub(crate) use crate::game::be::beintmission::tests::SEEDS;
    pub(crate) use crate::game::bx::bxstring2::tests::be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::fe::feasyncfile::tests::h;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_5 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_4 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5 as world_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_6;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_4;
    pub(crate) use crate::game::render::particle::tests::common;
    pub(crate) use crate::game::render::particle::tests::gp_at;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_3;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_2;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn chase_cam_controller_set_algorithm_immediate_matches() {
        crate::difftest::check(
            ee::s01::f_0015d050__generated,
            chase_cam_controller_set_algorithm_immediate,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15d0_50, 2);
                be_library(c);
                // Algorithm indices below 0x62 go through the (unloaded, all
                // zero) jump table, which leaves the function at once.
                c.g[A1][0] = (r.next_u32() % 0x62) as u64;
                c.g[A2][0] = sx(DATA + 0x400);
            },
        );
    }

    /// Every block `ChaseCamController_SetAlgorithm`'s jump table can name,
    /// and 0 (outside the function: `jump_out`).
    const SET_ALGORITHM_CASES: &[u32] = &[
        0x0015_d0f0,
        0x0015_d1a0,
        0x0015_d23c,
        0x0015_d264,
        0x0015_d28c,
        0x0015_d2b4,
        0x0015_d2dc,
        0x0015_d304,
        0x0015_d328,
        0x0015_d34c,
        0x0015_d370,
        0x0015_d394,
        0x0015_d3b8,
        0x0015_d3dc,
        0x0015_d400,
        0x0015_d430,
        0x0015_d458,
        0x0015_d480,
        0x0015_d4b8,
        0x0015_d4e0,
        0x0015_d508,
        0x0015_d530,
        0x0015_d558,
        0x0015_d560,
        0x0015_d58c,
        0x0015_d5b0,
        0x0015_d5d4,
        0x0015_d5f8,
        0x0015_d628,
        0,
    ];

    /// A chase camera at `DATA` (flags `flags`, rider at `+0x20`, an algorithm
    /// list at `+8` holding one algorithm), the main heap, and the dummy world.
    fn chase_cam_fixture(c: &mut Cpu, flags: u32) -> u32 {
        alloc_ready(c);
        be_library(c);
        let cam = DATA;
        let list = DATA + 0x600;
        clear(c, cam, 0x40);
        clear(c, list, 0x10);
        c.bus.write32(cam + 8, list);
        c.bus.write32(cam + 0x20, DATA + 0x700);
        c.bus.write32(cam + 0x30, flags);
        c.bus.write32(list + 4, 1);
        c.g[A0][0] = sx(cam);
        cam
    }

    #[test]
    fn chase_cam_controller_set_algorithm_matches() {
        check(
            ee::s01::f_0015d078__generated,
            chase_cam_controller_set_algorithm,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15d0_78, 4);
                let flags = r.next_u32() & 3;
                chase_cam_fixture(c, flags);
                // The id: in the table (its entry one of the cases) or not.
                let id = match seed % 8 {
                    0 => 0x62 + r.next_u32() % 0x100,
                    1 => r.next_u32() | 0x8000_0000,
                    _ => r.next_u32() % 0x62,
                };
                c.g[A1][0] = sx(id);
                let case = SET_ALGORITHM_CASES[seed as usize % SET_ALGORITHM_CASES.len()];
                if id < 0x62 {
                    c.bus.write32(ALGORITHM_TABLE + 4 * id, case);
                }
                // Params: the track spline (+8, type 2 at +0x28 or not) and
                // the timing floats.
                let params = DATA + 0x800;
                let spline = DATA + 0x900;
                clear(c, spline, 0x40);
                c.bus.write32(params + 8, spline);
                c.bus.write32(
                    spline + 0x28,
                    if seed % 3 == 0 { 2 } else { r.next_u32() % 4 },
                );
                for k in 0..4 {
                    c.bus.write32(params + 0x10 + 4 * k, r.f32(10.0));
                }
                c.g[A2][0] = sx(params);
                // The career's event count (none, under 10, 10 or more).
                let world = DATA + 0xa00;
                let module = DATA + 0xb00;
                let career = DATA + 0xc00;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, module);
                c.bus.write32(module + 0x28, career);
                c.bus.write32(career, [0, 3, 12][(seed / 3) as usize % 3]);
                // The relative distances.
                for off in [0x6cd4, 0x6cd0, 0x6ccc, 0x6cc8, 0x6cc4] {
                    c.bus.write32(GAME_GP.wrapping_sub(off), r.f32(20.0));
                }
                c.f[12] = if seed % 5 == 0 {
                    0x3f80_0000
                } else {
                    r.f32(1.0)
                };
                // The fixture's heap setup ran a function: enter at the top.
                c.pc = 0x0015_d078;
            },
        );
    }

    /// The blend rate reaches the list node unchanged, so a converted
    /// caller's rate (`camera::blend_rate_60`) blends over the same time at
    /// any tick rate: one second at 1/60 per 60 Hz tick takes 120 ticks at
    /// 120. Bit 1 of the flags still makes the switch instant.
    #[test]
    fn chase_cam_controller_set_algorithm_rate() {
        use crate::game::camera::camera::blend_rate_60;
        use crate::ps2f::FpuF32;
        let raw = FpuF32::from_f32(1.0 / 60.0);
        let node_after = |hz: u32, flags: u32| {
            let mut c = crate::difftest::fresh();
            machine(&mut c, 1, 0x15d0_78, 0);
            crate::tick::set_for_test(&mut c, hz);
            let cam = chase_cam_fixture(&mut c, flags);
            c.g[A1][0] = 0x62; // no new algorithm: the push alone
            c.f[12] = blend_rate_60(&mut c, raw).0;
            c.g[RA][0] = TEST_RA as u64;
            c.pc = 0x0015_d078;
            chase_cam_controller_set_algorithm(&mut c);
            assert_eq!(c.pc, TEST_RA);
            let list = c.bus.read32(cam + 8);
            let node = c.bus.read32(list);
            (
                FpuF32(c.bus.read32(node + 4)),
                FpuF32(c.bus.read32(node + 8)),
            )
        };
        let ticks_to_blend = |rate: FpuF32| {
            let mut w = FpuF32::ZERO;
            let mut n = 0;
            while w.to_f32() < 1.0 {
                w = w + rate;
                n += 1;
            }
            n
        };
        let (w60, rate60) = node_after(60, 0);
        assert_eq!((w60, rate60), (FpuF32::ZERO, raw), "60 Hz unchanged");
        let (_, rate120) = node_after(120, 0);
        let (t60, t120) = (ticks_to_blend(rate60), ticks_to_blend(rate120));
        assert!(
            (t120 as i32 - 2 * t60 as i32).abs() <= 1,
            "{t60} vs {t120} ticks"
        );
        let (w, rate) = node_after(120, 2);
        assert_eq!((w, rate), (FpuF32::ONE, FpuF32::ONE), "instant at any rate");
    }

    // -----------------------------------------------------------------------
    // Camera controller / algorithm constructors.

    #[test]
    fn manual_cam_controller_reset_matches() {
        check(
            ssx3_ee::s01::f_0015cee8__generated,
            manual_cam_controller_reset,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15ce_e8, 0);
                let this = DATA;
                let list = DATA + 0x100;
                let vt = DATA + 0x200;
                c.g[A0][0] = this as u64;
                // `CamController_Reset` walks `*(this + 8)` (null: nothing to do)
                // and resets the camera at `+0x10` (null: the splash manager's
                // rider chain is zeroed too).
                c.bus.write32(this + 0x8, list);
                c.bus.write32(this + 0x10, 0);
                c.bus.write32(this + 0x18, DATA + 0x300);
                c.bus.write32(DATA + 0x300 + 0x10, vt);
                c.bus.write16(vt + 0x20, (r.next_u32() & 7) as u16);
                c.bus.write32(vt + 0x24, FAKE);
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x15ce98 ManualCamController_ResetForRider.

    #[test]
    fn manual_cam_controller_reset_for_rider_matches() {
        check(
            ee::s01::f_0015ce98__generated,
            manual_cam_controller_reset_for_rider,
            16,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15ce_98, 2);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x400);
                c.bus.write32(this + 8, 0); // empty algorithm list
                c.bus.write32(this + 0x10, DATA + 0x200);
                c.bus.write32(this + 0x18, DATA + 0x300);
                let vt = vtable(c, &mut r, &[0x18]);
                c.bus.write32(DATA + 0x300 + 0x10, vt);
            },
        );
    }

    #[test]
    fn cam_controller_set_algorithm_matches() {
        check(
            ee::s01::f_0015cf28__generated,
            cam_controller_set_algorithm,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15cf_28, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let list = DATA + 0x800;
                clear(c, this, 0x40);
                clear(c, list, 0x100);
                c.bus.write32(this + 8, list);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    pub(crate) fn heap_block(c: &mut Cpu, size: u32) -> u32 {
        const MEM_HEAP: u32 = 0x004f_f250;
        call_setup(
            c,
            0x0032_0058,
            ssx3_ee::s10::f_00320058,
            &[MEM_HEAP as u64, 16, size as u64],
        ) as u32
    }

    #[test]
    fn cam_controller_dtor_matches() {
        check(
            ee::s01::f_0015cc10__generated,
            cam_controller_dtor,
            SEEDS_3,
            &|c, seed| {
                let _ = common(c, seed, 0x15cc10, 2);
                alloc_ready_2(c);
                let this = heap_block(c, 0x100);
                clear(c, this, 0x40);
                c.bus
                    .write32(this + 8, if seed % 3 == 0 { 0 } else { DATA + 0x2000 });
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    /// A v-table at `at` with `{delta = 0, fake}` at each slot.
    pub(crate) fn put_vtable(c: &mut Cpu, at: u32, slots: &[u32]) {
        for (i, &off) in slots.iter().enumerate() {
            c.bus.write16(at + off, 0);
            c.bus.write32(at + off + 4, FAKE + 0x10 * i as u32);
        }
    }

    #[test]
    fn cam_controller_reset_matches() {
        check(
            ee::s01::f_0015ccf0__generated,
            cam_controller_reset,
            50,
            &|c, seed| {
                machine(c, seed, 0x15cc_f0, 0);
                c.g[A0][0] = sx(DATA);
                let n0 = DATA + 0x1000;
                let n1 = if seed % 2 == 0 { 0 } else { DATA + 0x1200 };
                c.bus.write32(DATA + 8, n0);
                c.bus
                    .write32(n0, if seed % 3 == 0 { 0 } else { DATA + 0x2000 });
                c.bus.write32(n0 + 0x14, n1);
                if n1 != 0 {
                    c.bus.write32(n1, DATA + 0x2200);
                    c.bus.write32(n1 + 0x14, 0);
                }
                for p in [DATA + 0x2000u32, DATA + 0x2200] {
                    c.bus.write32(p + 0x10, p + 0x100);
                    put_vtable(c, p + 0x100, &[0x20]);
                }
                c.bus.write32(DATA + 0x10, 0);
            },
        );
    }

    /// The engine's size-class pool allocator, as in `oc106`: the "pools
    /// ready" flag plus eight free nodes per size class.
    pub(crate) fn pool(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w32(c, GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pool = DATA + 0x40_0000 + 0x4_0000 * i;
            let base = DATA + 0x80_0000 + 0x4000 * i;
            for n in 0..8u32 {
                let node = base + 0x400 * n;
                w32(c, node, if n == 7 { 0 } else { node + 0x400 });
            }
            w32(c, pool + head, base);
            w32(c, GAME_GP + slot, pool);
        }
    }

    #[test]
    fn cam_controller_ctor_matches() {
        check(
            ssx3_ee::s01::f_0015cba0__generated,
            cam_controller_ctor,
            SEEDS_4,
            &|c, seed| {
                let _ = machine(c, seed, 0x15cba0, 0);
                pool(c);
                let cam = DATA + 0x1000;
                c.g[A0][0] = cam as u64;
                c.g[A1][0] = (seed % 5) as u64;
            },
        );
    }

    /// A child object `obj+field` whose vtable (at `child+vt_field`) dispatches
    /// slot_2 8 to a fake.
    pub(crate) fn fake_child(c: &mut Cpu, obj: u32, field: u32, vt_field: u32, n: u32) {
        let child = DATA + 0x400 + 0x40 * n;
        let vt = DATA + 0x800 + 0x40 * n;
        w(c, obj + field, child);
        w(c, child + vt_field, vt);
        slot_2(c, vt, 8, 0, FAKE + 0x10 * n);
    }

    #[test]
    fn manual_cam_controller_dtor_matches() {
        check(
            ssx3_ee::s01::f_0015ce10__generated,
            manual_cam_controller_dtor,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15ce10, 3);
                be_library_2(c);
                world(c);
                let obj = DATA + 0x100;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = ((seed % 2) * 2) as u64;
                if seed % 3 != 0 {
                    fake_child(c, obj, 0x18, 0x10, 0);
                }
                if seed % 4 != 0 {
                    fake_child(c, obj, 0x1c, 0x10, 1);
                }
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn manual_cam_controller_ctor_matches() {
        check(
            ee::s01::f_0015cd80__generated,
            manual_cam_controller_ctor,
            SEEDS_6,
            &|c, seed| {
                machine(c, seed, 0x15cd, 0);
                env(c);
                let this = DATA + 0x400;
                w32(c, this + 0x10, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x800);
            },
        );
    }
}
