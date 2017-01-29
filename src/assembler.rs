use encoding::{Encoding, EncoderTrap};
use encoding::all::UTF_16LE;
use std::collections::{HashMap, BTreeMap};
use byteorder::{LittleEndian, WriteBytesExt};

use types::*;
use npc::*;


type FunctionId = u16;
type Register = u8;
type GFlag = u16;

// TODO: make enum for gflags?

// TODO: possibly rename these from their qedit equivalent?
#[derive(Debug)]
pub enum OpCode {
    Label(u16),
    Nop,
    Ret,
    Sync,
    Exit,
    Thread(FunctionId),
    VaStart,
    VaEnd,
    VaCall(FunctionId),
    Let(Register, Register),
    LetI(Register, u32),
    Set(Register),
    Clear(Register),
    Rev(Register),
    GSet(GFlag),
    GClear(GFlag),
    //GRev,
    //GLet,
    GGet(GFlag, Register),
    Add(Register),
    AddI(Register, u32),
    Sub(Register, Register),
    SubI(Register, u32),
    Mul(Register, Register),
    MulI(Register, u32),
    Div(Register, Register),
    DivI(Register, u32),
    And(Register, Register),
    AndI(Register, u32),
    Or(Register, Register),
    OrI(Register, u32),
    Xor(Register, Register),
    XorI(Register, u32),
    Mod(Register, Register),
    ModI(Register, u32),
    Jmp(FunctionId),
    Call(FunctionId),
    //JmpOn,
    //JmpOff,
    JmpEq(Register, Register, FunctionId),
    JmpIEq(Register, u32, FunctionId),
    JmpNotEq(Register, Register, FunctionId),
    JmpINotEq(Register, u32, FunctionId),
    UJmpGt(Register, Register, FunctionId),
    UJmpIGt(Register, u32, FunctionId),
    JmpGt(Register, Register, FunctionId),
    JmpIGt(Register, u32, FunctionId),
    UJmpLt(Register, Register, FunctionId),
    UJmpILt(Register, u32, FunctionId),
    JmpLt(Register, Register, FunctionId),
    JmpILt(Register, u32, FunctionId),
    UJmpGtEq(Register, Register, FunctionId),
    UJmpIGtEq(Register, u32, FunctionId),
    JmpGtEq(Register, Register, FunctionId),
    JmpIGtEq(Register, u32, FunctionId),
    UJmpLtEq(Register, Register, FunctionId),
    UJmpILtEq(Register, u32, FunctionId),
    JmpLtEq,
    JmpILtEq,
    //SwitchJmp,
    //SwitchCall,
    StackPush(Register),
    StackPop(Register),
    //stack_pushm,
    //stack_popm,
    ArgPushR(Register),
    ArgPushL(u32),
    //arg_pushb, // u8?
    ArgPushW(u16),
    ArgPushS(String),
    Message(u32, String),
    //List(),
    FadeIn,
    FadeOut,
    Se(u32),
    Bgm(u32),
    //enable,
    //disable,
    WindowMsg(String),
    AddMsg(String),
    MesEnd,
    GetTime(Register),
    WinEnd,
    //npc_crt_V1,
    //npc_crt_V3,
    NpcStop(u32),
    NpcPlay(u32),
    NpcKill(u32),
    //npc_nont,
    //npc_talk,
    //npc_crp_V1,
    //npc_crp_V3,
    //create_pipe,
    //p_hpstat_V1,
    //p_hpstat_V3,
    //p_dead_V1,
    PDeadV3(Register, Register),
    PDisableWarp,
    PEnableWarp,
    //p_move_V1,
    //p_move_V3,
    //p_look,
    PActionDisable,
    PActionEnable,
    DisableMovement1(u32),
    EnableMovement1(u32),
    //p_noncol,
    //p_col,
    PSetPos(u32, Register),
    PReturnGuild,
    //p_talk_guild,
    //npc_talk_pl_V1,
    //npc_talk_pl_V3,
    //npc_talk_kill,
    //npc_crtpk_V1,
    //npc_crtpk_V3,
    //npc_crppk_V1,
    //npc_crppk_V3,
    //npc_crptalk_v1,
    //npc_crptalk_v3,
    PLookAtV1(u32, u32),
    //npc_crp_id_V1,
    //npc_crp_id_V3,
    CamQuake,
    CamAdj,
    CamZMIn,
    CamZMOut,
    //cam_pan_V1,
    //cam_pan_V3,
    //game_lev_super,
    //game_lev_reset,
    //pos_pipe_V1,
    PosPipeV3(Register),
    IfZoneClear(Register, Register),
    ChkEneNum(Register),
    UnhideObj(Register),
    UnhideEne(Register),
    AtCoordsCall(Register),
    AtCoordsTalk(Register),
    //col_npcin,
    //col_npcinr,
    SwitchOn(Register), // TODO: u32 also valid
    SwitchOff(Register), // TODO: u32 also valid
    //playbgm_epi,
    SetMainwarp(u32),
    SetObjParam(Register, Register),
    SetFloorHandler(u32, FunctionId),
    //clr_floor_handler,
    //col_plinaw,
    HudHide,
    HudShow,
    CineEnable,
    CineFisable,
    SetQtFailure(FunctionId),
    SetQtSuccess(FunctionId),
    //clr_qt_failure,
    //clr_qt_success,
    SetQtCancel(FunctionId),
    //clr_qt_cancel,
    //pl_walk_V1,
    //pl_walk_V3,
    //pl_add_meseta,
    ThreadStg(FunctionId),
    //del_obj_param,
    //item_create,
    //item_create2,
    //item_delete,
    ItemDelete2(Register, Register),
    //item_check,
    SetEvt(u32),
    //get_difflvl,
    SetQtExit(FunctionId),
    //clr_qt_exit,
    //particle_V1,
    //particle_V3,
    //npc_text,
    //npc_chkwarp,
    PlPkOff(),
    //map_designate,
    //masterkey_on,
    //masterkey_off,
    WindowTime,
    WinEndTime,
    WinSetTime(Register),
    //getmtime,
    SetQuestBoardHandler(u32, FunctionId, String),
    ClearQuestBoardHandler(u32),
    //particle_id_V1,
    //particle_id_V3,
    //npc_crptalk_id_V1,
    //npc_crptalk_id_V3,
    //npc_lang_clean,
    PlPkOn,
    //pl_chk_item2,
    EnableMainMenu,
    DisableMainMenu,
    //start_battlebgm,
    //end_battlebgm,
    DispMsgQb(String),
    CloseMsgQb,
    //set_eventflag_v1,
    //set_eventflag_v3,
    SyncLetI(Register, u32),
    //set_returnhunter,
    //set_returncity,
    //load_pvr,
    //load_midi,
    //npc_param_V1,
    //npc_param_V3,
    //pad_dragon,
    ClearMainWarp(u32),
    //pcam_param_V1,
    //pcam_param_V3,
    //start_setevt_v1,
    //start_setevt_v3,
    WarpOn,
    WarpOff,
    GetSlotNumber(Register),
    //get_servernumber,
    //set_eventflag2,
    //res,
    //unknownEA,
    EnableBgmCtrl(u32),
    //sw_send,
    CreateBgmCtrl,
    PlAddMeseta2(u32),
    SyncLet(Register, Register),
    SyncRegister(Register, Register), // TODO: also allows u32
    //send_regwork,
    LetIFixedCameraV1(Register),
    //leti_fixed_camera_V3,
    DefaultCameraPos1,
    //unknownF8,
    //GetGcNumber,
    //unknownFB,
    //unknownFF,
    //set_chat_callback?,
    GetDifficultyLevel2(Register),
    GetNumberOfPlayer1(Register),
    GetCoordOfPlayer(Register, Register),
    //unknownF80B,
    //unknownF80C,
    //map_designate_ex,
    //unknownF80E,
    //unknownF80F,
    //ba_initial_floor,
    //set_ba_rules,
    //unknownF812,
    //unknownF813,
    //unknownF814,
    //unknownF815,
    //unknownF816,
    //unknownF817,
    //unknownF818,
    //unknownF819,
    //unknownF81A,
    //unknownF81B,
    //ba_disp_msg,
    //death_lvl_up,
    //death_tech_lvl_up,
    //unknown,
    //cmode_stage,
    //unknown,
    //unknown,
    //unknownF823,
    //unknownF824,
    //exp_multiplication,
    //exp_division?,
    //get_user_is_dead?,
    GoFloor(Register, Register),
    //unknown,
    //unknown,
    UnlockDoor2(u32, u32),
    LockDoor2(u32, u32),
    //if_switch_not_pressed,
    IfSwitchPressed(Register),
    //unknownF82F,
    //control_dragon,
    //release_dragon,
    Shrink(Register),
    Unshrink(Register),
    //display_clock2?,
    //unknownF83D,
    //delete_area_title?,
    LoadNpcData,
    //get_npc_data,
    //give_damage_score,
    //take_damage_score,
    //unk_score_F84A,
    //unk_score_F84B,
    //kill_score,
    //death_score,
    //unk_score_F84E,
    //enemy_death_score,
    //meseta_score,
    //unknownF851,
    //unknownF852,
    //reverse_warps,
    //unreverse_warps,
    //set_ult_map,
    //unset_ult_map,
    //set_area_title,
    //unknownF858,
    //unknown,
    //equip_item,
    //unequip_item,
    //unknownF85E,
    //unknownF85F,
    //unknownF860,
    //unknownF861,
    //cmode_rank,
    //award_item_name?,
    //award_item_select?,
    //award_item_give_to?,
    //unknownF868,
    //unknownF869,
    //item_create_cmode,
    //unknownF86B,
    //award_item_ok?,
    //unknownF86D,
    //unknownF86E,
    //ba_set_lives,
    //ba_set_tech_lvl,
    //ba_set_lvl,
    //ba_set_time_limit,
    //boss_is_dead?,
    EnableTechs(Register),
    DisableTechs(Register),
    GetGender(Register, Register),
    GetCharaClass(Register, Register),
    TakeSlotMeseta(Register, Register),
    //read_guildcard_flag,
    //unknownF880,
    GetPlName(Register),
    //unknown,
    //unknownF883,
    //unknown,
    //unknown,
    //unknown,
    //unknown,
    //ba_close_msg,
    //unknown,
    //get_player_status,
    //send_mail,
    //online_check,
    //chl_set_timerecord?,
    //chl_get_timerecord?,
    //unknownF88F,
    //unknownF890,
    //load_enemy_data,
    //get_physical_data,
    //get_attack_data,
    //get_resist_data,
    //get_movement_data,
    ShiftLeft(Register, Register),
    ShiftRight(Register, Register),
    GetRandom(Register, Register),
    //reset_map,
    //disp_chl_retry_menu,
    //chl_reverser?,
    //unknownF89E,
    //unknownF89F,
    //unknownF8A0,
    //unknownF8A1,
    //unknownF8A8,
    //unknownF8A9,
    //get_number_of_player2,
    //unknownF8B8,
    //chl_recovery?,
    SetEpisode(u32),
    //file_dl_req,
    //get_dl_status,
    //gba_unknown4?,
    //get_gba_state?,
    //unknownF8C4,
    //unknownF8C5,
    QEXIT,
    UseAnimation(Register, Register),
    StopAnimation(Register),
    //run_to_coord,
    SetSlotInvincible(Register, Register),
    //unknownF8CB,
    SetSlotPoison(Register),
    SetSlotParalyze(Register),
    SetSlotShock(Register),
    SetSlotFreeze(Register),
    SetSlotSlow(Register),
    SetSlotConfuse(Register),
    SetSlotShifta(Register),
    SetSlotDeband(Register),
    SetSlotJellen(Register),
    SetSlotZalure(Register),
    //fleti_fixed_camera,
    //fleti_locked_camera,
    DefaultCameraPos2,
    SetMotionBlur,
    SetScreenBW,
    //unknownF8DB,
    //NPC_action_string,
    //get_pad_cond,
    //get_button_cond,
    FreezeEnemies,
    UnfreezeEnemies,
    FreezeEverything,
    UnfreezeEverything,
    RestoreHp,
    RestoreTp,
    //close_chat_bubble,
    //unknownF8E6,
    //unknownF8E7,
    //unknownF8E8,
    //unknownF8E9,
    //unknownF8EA,
    //unknownF8EB,
    //unknownF8EC,
    //animation_check,
    //call_image_data,
    //unknownF8EF,
    TurnOffBgmP2,
    TurnOnBgmP2,
    //load_unk_data,
    //particle2,
    Dec2Float(Register, Register),
    Float2Dec(Register, Register),
    FLet(Register, Register),
    FLetI(Register, f32),
    FAdd(Register, Register),
    FAddI(Register, f32),
    FSub(Register, Register),
    FSubI(Register, f32),
    FMul(Register, Register),
    FMulI(Register, f32),
    FDiv(Register, Register),
    FDivI(Register, f32),
    //get_unknown_count?,
    GetStackableItemCount(Register, Register),
    FreezeAndHideEquip,
    ThawAndShowEquip,
    SetPaletteXCallback(Register, FunctionId),
    ActivatePaletteX(Register),
    EnablePaletteX(Register),
    RestorePaletteX(Register),
    DisablePaletteX(Register),
    GetPaletteXActivated(Register, Register),
    //get_unknown_paletteX_status?,
    DisableMovement2(Register),
    EnableMovement2(Register),
    //get_time_played,
    //get_guildcard_total,
    GetSlotMeseta(Register),
    //get_player_level,
    GetSectionId(Register, Register),
    GetPlayerHp(Register, Register),
    GetFloorNumber(Register, Register),
    //get_coord_player_detect,
    ReadGlobalFlag(u32, Register),
    WriteGlobalFlag(u32, Register),
    //unknownF927,
    FloorPlayerDetect(Register),
    //read_disk_file?,
    //open_pack_select,
    //item_select,
    //get_item_id,
    //color_change,
    SendStatistic(Register, Register, Register, Register, Register, Register, Register, Register),
    //unknownF92F,
    ChatBox(u32, u32, u32, u32, u32, String),
    ChatBubble(u32, String),
    //unknownF933,
    ScrollText(u32, u32, u32, u32, f32, Register, String),
    //gba_unknown1,
    //gba_unknown2,
    //gba_unknown3,
    AddDamageTo(Register, u32),
    //item_delete2,
    //get_item_info,
    //item_packing1,
    //item_packing2,
    //get_lang_setting?,
    PrepareStatistic(Register, FunctionId, FunctionId),
    KeywordDetect,
    Keyword(Register, Register, String),
    GetGuildcardNum(Register, Register),
    //get_wrap_status,
    InitialFloor(u32),
    //sin,
    //cos,
    //boss_is_dead2?,
    //unknownF94B,
    //unknownF94C,
    //is_there_cardbattle,
    //BB_p2_menu,
    //BB_Map_Designate,
    GetNumberInPack(Register),
    //BB_swap_item,
    //BB_check_wrap,
    //BB_exchange_PD_item,
    //BB_exchange_PD_srank,
    //BB_exchange_PD_special,
    //BB_exchange_PD_percent,
    //unknownF959,
    //BB_exchange_SLT,
    //ExchangePc(),
    //BB_box_create_BP,
    //BB_exchange_PT,
    //unknownF960,
    //unknownF961,
}


enum OpCodeCmd {
    None,
    u8(u8),
    u16(u16),
}

#[derive(Debug)]
enum OpCodeArg {
    u8(u8),
    i8(i8),
    u16(u16),
    i16(i16),
    u32(u32),
    i32(i32),
    f32(f32),
    reg(Register),
    string(String),
}

enum OpCodeType {
    Imediate,
    Stack,
}

struct OpCodeBytes {
    cmd: OpCodeCmd,
    otype: OpCodeType,
    args: Vec<OpCodeArg>,
}

impl OpCodeBytes {
    pub fn imed() -> OpCodeBytes {
        OpCodeBytes {
            cmd: OpCodeCmd::None,
            otype: OpCodeType::Imediate,
            args: Vec::new()
        }
    }

    pub fn stack() -> OpCodeBytes {
        OpCodeBytes {
            cmd: OpCodeCmd::None,
            otype: OpCodeType::Stack,
            args: Vec::new()
        }
    }

    pub fn cmd_u8<'a>(&'a mut self, cmd: u8) -> &'a mut OpCodeBytes {
        self.cmd = OpCodeCmd::u8(cmd);
        self
    }

    pub fn cmd_u16<'a>(&'a mut self, cmd: u16) -> &'a mut OpCodeBytes {
        self.cmd = OpCodeCmd::u16(cmd);
        self
    }

    pub fn arg_u8<'a>(&'a mut self, arg: u8) -> &'a mut OpCodeBytes {
        self.args.push(OpCodeArg::u8(arg));
        self
    }

    pub fn arg_i8<'a>(&'a mut self, arg: i8) -> &'a mut OpCodeBytes {
        self.args.push(OpCodeArg::i8(arg));
        self
    }

    pub fn arg_u16<'a>(&'a mut self, arg: u16) -> &'a mut OpCodeBytes {
        self.args.push(OpCodeArg::u16(arg));
        self
    }

    pub fn arg_i16<'a>(&'a mut self, arg: i16) -> &'a mut OpCodeBytes {
        self.args.push(OpCodeArg::i16(arg));
        self
    }
    
    pub fn arg_u32<'a>(&'a mut self, arg: u32) -> &'a mut OpCodeBytes {
        self.args.push(OpCodeArg::u32(arg));
        self
    }

    pub fn arg_i32<'a>(&'a mut self, arg: i32) -> &'a mut OpCodeBytes {
        self.args.push(OpCodeArg::i32(arg));
        self
    }
    
    pub fn arg_f32<'a>(&'a mut self, arg: f32) -> &'a mut OpCodeBytes {
        self.args.push(OpCodeArg::f32(arg));
        self
    }
    
    pub fn arg_reg<'a>(&'a mut self, arg: Register) -> &'a mut OpCodeBytes {
        self.args.push(OpCodeArg::reg(arg));
        self
    }

    pub fn arg_string<'a>(&'a mut self, arg: String) -> &'a mut OpCodeBytes {
        self.args.push(OpCodeArg::string(arg));
        self
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self.cmd {
            OpCodeCmd::u8(cmd) => bytes.write_u8(cmd),
            OpCodeCmd::u16(cmd) => bytes.write_u16::<LittleEndian>(cmd),
            OpCodeCmd::None => panic!("noopcode")
        };

        for arg in self.args.iter() {
            match self.otype {
                OpCodeType::Imediate => {
                    match arg {
                        &OpCodeArg::u8(d) => bytes.write_u8(d),
                        &OpCodeArg::i8(d) => bytes.write_i8(d),
                        &OpCodeArg::u16(d) => bytes.write_u16::<LittleEndian>(d),
                        &OpCodeArg::i16(d) => bytes.write_i16::<LittleEndian>(d),
                        &OpCodeArg::u32(d) => bytes.write_u32::<LittleEndian>(d),
                        &OpCodeArg::i32(d) => bytes.write_i32::<LittleEndian>(d),
                        &OpCodeArg::f32(d) => bytes.write_f32::<LittleEndian>(d),
                        &OpCodeArg::reg(d) => bytes.write_u8(d),
                        _ => panic!("gjkrjglrg")
                    };
                }
                OpCodeType::Stack => {
                    match arg {
                        &OpCodeArg::u8(d) => {
                            bytes.write_u8(0x4A);
                            bytes.write_u8(d);
                        }
                        &OpCodeArg::i8(d) => {
                            bytes.write_u8(0x4A);
                            bytes.write_i8(d);
                        }
                        &OpCodeArg::u16(d) => {
                            bytes.write_u8(0x4B);
                            bytes.write_u16::<LittleEndian>(d);
                        }
                        &OpCodeArg::i16(d) => {
                            bytes.write_u8(0x4B);
                            bytes.write_i16::<LittleEndian>(d);
                        }
                        &OpCodeArg::u32(d) => {
                            bytes.write_u8(0x49);
                            bytes.write_u32::<LittleEndian>(d);
                        }
                        &OpCodeArg::i32(d) => {
                            bytes.write_u8(0x49);
                            bytes.write_i32::<LittleEndian>(d);
                        }
                        &OpCodeArg::f32(d) => {
                            bytes.write_u8(0x49); // assuming?
                            bytes.write_f32::<LittleEndian>(d);
                        }
                        &OpCodeArg::reg(d) => {
                            bytes.write_u8(0x48);
                            bytes.write_u8(d);
                        }
                        &OpCodeArg::string(ref d) => {
                            bytes.write_u8(0x4E);
                            let mut utf16str = UTF_16LE.encode(d.as_str(), EncoderTrap::Ignore).unwrap();
                            bytes.write_u16::<LittleEndian>(utf16str.len()as u16 +2);
                            bytes.append(&mut utf16str);
                            bytes.write_u16::<LittleEndian>(0);
                        }
                    };
                }
            }
        }
        bytes
    }
}


impl OpCode {
    fn as_bytes(&self) -> Vec<u8> {
        match self {
            &OpCode::SetEpisode(ep) => {
                OpCodeBytes::imed()
                    .cmd_u16(0xf8bc)
                    .arg_u32(ep)
                    .as_bytes()
            }
            &OpCode::Ret => {
                OpCodeBytes::imed()
                    .cmd_u8(0x01)
                    .as_bytes()
            }
            &OpCode::JmpIEq(reg, val, func) => {
                OpCodeBytes::imed()
                    .cmd_u8(0x2d)
                    .arg_reg(reg)
                    .arg_i32(val as i32)
                    .arg_u16(func)
                    .as_bytes()
            }
            &OpCode::Message(npcid, ref mstr) => {
                OpCodeBytes::stack()
                    .cmd_u8(0x50)
                    .arg_u32(npcid)
                    .arg_string(mstr.clone())
                    .as_bytes()
            }
            &OpCode::AddMsg(ref mstr) => {
                OpCodeBytes::stack()
                    .cmd_u8(0x5b)
                    .arg_string(mstr.clone())
                    .as_bytes()
            }
            &OpCode::MesEnd => {
                OpCodeBytes::imed()
                    .cmd_u8(0x5c)
                    .as_bytes()
            }
            &OpCode::Set(reg) => {
                OpCodeBytes::imed()
                    .cmd_u8(0x10)
                    .arg_reg(reg)
                    .as_bytes()
            }
            &OpCode::Jmp(label) => {
                OpCodeBytes::imed()
                    .cmd_u8(0x28)
                    .arg_u16(label)
                    .as_bytes()
            }
            _ => panic!("cannot turn opcode to bytes: {:?}", self)
        }
    }
}

#[derive(Debug)]
pub enum AssemblerError {
    InvalidArgument(String, String, String),
}

macro_rules! expect_type {
    ($arg:expr, $t:path) => {
        match $arg {
            $t(ref var) => Ok(var.clone()),
            _ => {
                Err(AssemblerError::InvalidArgument(String::from(format!("{}:{}", module_path!(), line!())),
                                                        $arg.to_string(),
                                                        String::from("expected different type")))
            }
        }
    }
}

#[derive(Debug)]
struct RegisterPool {
   registers: Vec<Register>,
}

impl RegisterPool {
    fn new() -> RegisterPool {
        RegisterPool {
            // a handful of registers are reserved
            // see http://qedit.info/index.php?title=Reservedregisters
            registers: (1..256).rev().map(|r| r as Register).collect()
        }
    }

    fn take(&mut self) -> Register {
        self.registers.pop().unwrap() // oh god if we use more than 256 registers at once...
    }

    fn release(&mut self, reg: Register) {
        self.registers.push(reg)
    }
}

#[derive(Debug)]
pub struct Assembler {
    next_label_id: u16,
    //opcodes: Vec<OpCode>,
    //opcode_exprs: Vec<LabeledExpr>,
    //opcode_exprs: BTreeMap<u16, Vec<OpCode>>,
    //quest: &'a mut Quest,
    opcodes: Vec<OpCode>,
    register_pool: RegisterPool,
    named_registers: HashMap<String, Register>,
    npc_labels: HashMap<String, u16>,
    npc_ids: HashMap<String, f32>,
    func_labels: HashMap<String, u16>,
        
}

impl Assembler {
    pub fn new(mut quest: &mut Quest) -> Assembler {
        let mut assembler = Assembler {
            next_label_id: 2,
            opcodes: Vec::new(),
            //opcode_exprs: BTreeMap::new(),
            register_pool: RegisterPool::new(),
            named_registers: HashMap::new(),
            npc_labels: HashMap::new(),
            npc_ids: HashMap::new(),
            func_labels: HashMap::new(),
            //quest: quest,
        };

        // precompile_* functions?
        for (label, npc) in quest.npcs.iter() {
            let nid = assembler.get_label();
            assembler.npc_labels.insert(label.clone(), nid);
            assembler.npc_ids.insert(label.clone(), npc.character_id);
        }

        assembler.set_variables(&quest.variables);
        assembler.assemble_0(&quest);
        //for func in quest.functions.iter() {
        //    assembler.assemble_function(func, &quest)
        //}
        //assembler.assemble_functions(&mut quest);
        
        assembler.assemble_npcs(&mut quest.npcs);


        assembler
    }

    fn get_label(&mut self) -> u16 {
        self.next_label_id += 1;
        self.next_label_id
    }

    fn set_variables(&mut self, vars: &Vec<Variable>) {
        for var in vars.iter() {
            self.named_registers.insert(var.name.clone(), self.register_pool.take());
        }
    }
    

    fn assemble_0(&mut self, quest: &Quest) {
        //code.push(OpCode::SetEpisode(quest.episode));
        self.opcodes.push(OpCode::Label(0));
        self.opcodes.push(OpCode::SetEpisode(quest.episode));
        
        // more stuff here!

        // set variable defaults
        
        /*self.opcode_exprs.push(LabeledExpr {
            label: 0.,
            opcodes: code,
    });*/

        
        self.opcodes.push(OpCode::Label(1));
        self.opcodes.push(OpCode::Ret);
    }

    
    fn assemble_block(&mut self, expr: &Vec<PExpr>) -> Vec<OpCode> {
        let mut pasm = Vec::new();
        for e in expr.iter() {
            pasm.append(&mut self.assemble_expr(&e));
        }

        pasm
    }

    fn assemble_conditional(&mut self, expr: &PExpr, label: u16) -> Vec<OpCode> {
        match expr {
            &PExpr::Equal(ref args) => {
                println!("eq, {:?}", args);
                match (&args[0], &args[1]) {
                    (&PExpr::Identifier(ref var1), &PExpr::Identifier(ref var2)) => {
                        vec![OpCode::JmpEq(*self.named_registers.get(var1).unwrap(),
                                           *self.named_registers.get(var2).unwrap(),
                                           label)]
                    }
                    (&PExpr::Identifier(ref var), &PExpr::Integer(ref val)) => {
                        vec![OpCode::JmpIEq(*self.named_registers.get(var).unwrap(),
                                           *val,
                                           label)]
                    }
                    (&PExpr::Identifier(ref var), &PExpr::Boolean(ref val)) => {
                        vec![OpCode::JmpIEq(*self.named_registers.get(var).unwrap(),
                                           *val as u32,
                                           label)]
                    }
                    _ => Vec::new()
                }
                
            }
            _ => Vec::new()
        }
    }
    
    fn assemble_if(&mut self, expr: &Vec<PExpr>) -> Vec<OpCode> {
        let true_label = self.get_label();
        let end_label = self.get_label();
        
        let mut cond_expr = self.assemble_conditional(&expr[0], true_label);
        let mut true_expr = self.assemble_expr(&expr[1]);
        let mut false_expr = self.assemble_expr(&expr[2]);

        
        let mut pasm = Vec::new();
        pasm.append(&mut cond_expr);
        pasm.append(&mut false_expr);
        pasm.push(OpCode::Jmp(end_label));
        pasm.push(OpCode::Label(true_label));
        pasm.append(&mut true_expr);
        pasm.push(OpCode::Label(end_label));
        
        pasm
    }


    fn assemble_npc_say(&mut self, expr: &Vec<PExpr>) -> Vec<OpCode> {
        let mut pasm = Vec::new();
        let npc_id = if let PExpr::Identifier(ref nlabel) = expr[0] {
            *self.npc_ids.get(nlabel).unwrap()
        }
        else {
            -1.
        };

        
        let npc_str = if let PExpr::StringLiteral(ref msg) = expr[1] {
            msg.clone()
        }
        else {
            String::from("azazazaz")
        };

        pasm.push(OpCode::Message(npc_id as u32, npc_str));
        
        for emsg in expr.iter().skip(2) {
            let npc_str = if let &PExpr::StringLiteral(ref msg) = emsg {
                msg.clone()
            }
            else {
                String::from("azazazaz")
            };
            pasm.push(OpCode::AddMsg(npc_str));
        }
        

        pasm.push(OpCode::MesEnd);
        
        pasm
    }
    
    fn assemble_set(&mut self, expr: &Vec<PExpr>) -> Vec<OpCode> {
        let mut pasm = Vec::new();

        let reg = if let PExpr::Identifier(ref vlabel) = expr[0] {
            *self.named_registers.get(vlabel).unwrap()
        }
        else {
            0
        };

        if let PExpr::Boolean(ref val) = expr[1] {
            if *val {
                pasm.push(OpCode::Set(reg));
            }
            else {
                pasm.push(OpCode::Clear(reg));
            }
        }
        else {
            // TODO
        }
        
        
        pasm
    }


    
    fn assemble_expr(&mut self, expr: &PExpr) -> Vec<OpCode> {
        match expr {
            &PExpr::Block(ref e) => self.assemble_block(&e),
            &PExpr::If(ref e) => self.assemble_if(&e),
            &PExpr::NpcSay(ref e) => self.assemble_npc_say(&e),
            &PExpr::Set(ref e) => self.assemble_set(&e),
            _ => Vec::new(),
        }
        
    }

    

    
    fn assemble_npcs(&mut self, npcs: &mut HashMap<String, Npc>) {
        //let mut npcs = &mut self.quest.npcs; // make borrow checker be quiet
        //for (_, npc) in self.quest.npcs.iter() {
        //for (_, npc) in self.quest.npcs.iter() {
        //for nlabel in self.quest.npcs.clone().keys() {
        for (nlabel, npc) in npcs.iter_mut() {
            //for nlabel in quest.npcs.keys() {
            //let mut npc = self.quest.npcs.get(nlabel).unwrap();
            let mut npc_func = if let Function::Expr(ref expr) = npc.function {
                self.assemble_expr(&expr)
            }
            else {
                Vec::new() // panic?
            };

            //self.quest.npcs.get(zz).unwrap().function = Function::Id(self.get_label() as f32);
            //npc.function = Function::Id(self.get_label() as f32);
            
            //self.opcodes.push(OpCode::Label(self.next_label_id));
            self.opcodes.push(OpCode::Label(*self.npc_labels.get(nlabel).unwrap()));
            self.opcodes.append(&mut npc_func);
            println!("npcop: {:?}", self.opcodes);
        }
    }


    pub fn as_bytes(self) -> (Vec<u8>, Vec<u8>) {
        let mut pasm = Vec::new();
        let mut func_table = Vec::new();

        let mut maxsize = 1;
        for opcode in self.opcodes.iter() {
            if let &OpCode::Label(label) = opcode {
                maxsize += 1;
            }
        }

        for _ in 0..maxsize {
            func_table.push(0xffffffff);
        }

        
        for opcode in self.opcodes.iter() {
            if let &OpCode::Label(label) = opcode {
                func_table[label as usize] = pasm.len();
            }
            else {
                pasm.append(&mut opcode.as_bytes());
            }
        }

        let mut func_table_bytes = Vec::new();
        for func in func_table {
            func_table_bytes.write_u32::<LittleEndian>(func as u32);
        }
        
        
        (pasm, func_table_bytes)
    }

}

