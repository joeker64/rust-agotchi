#[path = "./ram.rs"]
mod ram;

const FLAG_C: u16 = 0x1;
const FLAG_Z: u16 = 0x2;

pub struct Opcode {
    name: &'static str,
    code: u16,
    cycles: u8,
    operation: unsafe fn (*mut super::CPU, u16),
}

unsafe fn nop5_operation(cpu: *mut super::CPU, step: u16){
    //Operand does nothing for 5 cycles
}
unsafe fn nop7_operation(cpu: *mut super::CPU, step: u16){
    //Operand does nothing for 7 cycles
}
unsafe fn halt_operation(cpu: *mut super::CPU, step: u16){
    //TODO - Stop clock
}
unsafe fn pset_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).new_pointer = (step & 0x1F);
}
unsafe fn jp_s_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).next_program_counter = step & 0xFF;
}
unsafe fn jp_cs_operation(cpu: *mut super::CPU, step: u16){
    if ((*cpu).flags & FLAG_C) == 1 {
        (*cpu).next_program_counter = step & 0xFF;
    }
}
unsafe fn jp_ncs_operation(cpu: *mut super::CPU, step: u16){
    if ((*cpu).flags & FLAG_C) != 1 {
        (*cpu).next_program_counter = step & 0xFF;
    }
}
unsafe fn jp_zs_operation(cpu: *mut super::CPU, step: u16){
    if ((*cpu).flags & FLAG_Z) == 1 {
        (*cpu).next_program_counter = step & 0xFF;
    }
}
unsafe fn jp_nzs_operation(cpu: *mut super::CPU, step: u16){
    if ((*cpu).flags & FLAG_Z) != 1 {
        (*cpu).next_program_counter = step & 0xFF;
    }
}
unsafe fn jpba_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).next_program_counter = ((*cpu).register_a | ((*cpu).register_b << 4)) | ((*cpu).new_pointer << 8)
}
unsafe fn call_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).program_counter = ((*cpu).program_counter + 1) & 0x1FFF;
    (*cpu).memory = ram::set_memory((*cpu).memory, ((*cpu).stack_pointer) - 1, ((*cpu).program_counter >> 8) & 0xF);
    (*cpu).memory = ram::set_memory((*cpu).memory, ((*cpu).stack_pointer) - 2, ((*cpu).program_counter >> 4) & 0xF);
    (*cpu).memory = ram::set_memory((*cpu).memory, ((*cpu).stack_pointer) - 3, (*cpu).program_counter & 0xF);
    (*cpu).stack_pointer = ((*cpu).stack_pointer - 3) & 0xFF;
    (*cpu).next_program_counter = (step & 0xFF) | ((*cpu).new_pointer & 0xF) << 8 | (((*cpu).program_counter >> 12) & 0x1) << 12
}
unsafe fn callz_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).program_counter = ((*cpu).program_counter + 1) & 0x1FFF;
    (*cpu).memory = ram::set_memory((*cpu).memory, ((*cpu).stack_pointer) - 1, ((*cpu).program_counter >> 8) & 0xF);
    (*cpu).memory = ram::set_memory((*cpu).memory, ((*cpu).stack_pointer) - 2, ((*cpu).program_counter >> 4) & 0xF);
    (*cpu).memory = ram::set_memory((*cpu).memory, ((*cpu).stack_pointer) - 3, (*cpu).program_counter & 0xF);
    (*cpu).stack_pointer = ((*cpu).stack_pointer - 3) & 0xFF;
    (*cpu).next_program_counter = (step & 0xFF) | 0 << 8 | (((*cpu).program_counter >> 12) & 0x1) << 12
}
unsafe fn ret_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).next_program_counter = ram::get_memory((*cpu).memory,(*cpu).stack_pointer) | ram::get_memory((*cpu).memory,(*cpu).stack_pointer + 1) << 4 | ram::get_memory((*cpu).memory,(*cpu).stack_pointer + 2) << 8 | ((*cpu).program_counter >> 12) & 0x1;
}
unsafe fn ret_s_operation(cpu: *mut super::CPU, step: u16){
    //TODO 
}
unsafe fn ret_d_operation(cpu: *mut super::CPU, step: u16){
    //TODO 
}
unsafe fn inc_x_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).register_x = ((*cpu).register_x + 1) & 0xFF;
    //TODO - double check XP
}
unsafe fn inc_y_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).register_y = ((*cpu).register_y + 1) & 0xFF;
    //TODO - double check YP
}
unsafe fn ld_x_operation(cpu: *mut super::CPU, step: u16){
    //TODO 
}
unsafe fn ld_y_operation(cpu: *mut super::CPU, step: u16){
    //TODO 
}
unsafe fn ld_xpr_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn ld_xhr_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn ld_xlr_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn ld_ypr_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn ld_yhr_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn ld_ylr_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn ld_rxp_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn ld_rxh_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn ld_rxl_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn ld_ryp_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn ld_ryh_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn ld_yl_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn adc_xh_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn adc_xl_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn adc_yh_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn adc_yl_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn cp_xh_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn cp_yh_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn cp_yl_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn ld_r_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn ld_rq_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn ld_am_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn ld_bm_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn ld_ma_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn ld_mb_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn ldpx_mx_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn ldpx_rq_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn ldpy_my_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn ldpy_rq_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn lbpx_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn set_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn rst_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn scf_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn rcf_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn szf_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn rzf_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn sdf_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn rdf_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn ei_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn di_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn inc_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn dec_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn push_r_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn push_xp_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn push_xh_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn push_xl_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn push_yp_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn push_yh_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn push_yl_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn push_f_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn pop_r_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn pop_xp_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn pop_xh_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn pop_xl_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn pop_yp_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn pop_yh_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn pop_yl_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn pop_f_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn ld_sph_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn ld_spl_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn ld_rsph_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn ld_rspl_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn add_r_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn add_rq_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn adc_r_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn adc_rq_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn sub_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn sbc_r_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn sbc_rq_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn and_r_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn and_rq_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn or_r_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn or_rq_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn xor_r_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn xor_rq_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn cp_r_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn cp_rq_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn fan_r_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn fan_rq_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn rlc_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn rrc_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn inc_m_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn dec_m_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn acpx_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn acpy_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn scpx_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn scpy_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}
unsafe fn not_operation(cpu: *mut super::CPU, step: u16){
    //TODO
}

pub const ISA : [Opcode; 108] = [
    Opcode{
        name: "NOP5",
        code: 0xFFB,
        cycles: 5,
        operation: nop5_operation,
    },
    Opcode{
        name: "NOP7",
        code: 0xFFF,
        cycles: 7,
        operation: nop7_operation,
    },
    Opcode{
        name: "HALT",
        code: 0xFF8,
        cycles: 5,
        operation: halt_operation,
    },
    Opcode{
        name: "PSET",
        code: 0xE40,
        cycles: 5,
        operation: pset_operation,
    },
    Opcode{
        name: "JP_S",
        code: 0x000,
        cycles: 5,
        operation: jp_s_operation,
    },
    Opcode{
        name: "JP_CS",
        code: 0x200,
        cycles: 5,
        operation: jp_cs_operation,
    },
    Opcode{
        name: "JP_NCS",
        code: 0x300,
        cycles: 5,
        operation: jp_ncs_operation,
    },
    Opcode{
        name: "JP_ZS",
        code: 0x600,
        cycles: 5,
        operation: jp_zs_operation,
    },
    Opcode{
        name: "JP_NZS",
        code: 0x700,
        cycles: 5,
        operation: jp_nzs_operation,
    },
    Opcode{
        name: "JPBA",
        code: 0xFE8,
        cycles: 5,
        operation: jpba_operation,
    },
    Opcode{
        name: "CALL",
        code: 0x400,
        cycles: 7,
        operation: call_operation,
    },
    Opcode{
        name: "CALZ",
        code: 0x500,
        cycles: 7,
        operation: callz_operation,
    },
    Opcode{
        name: "RET",
        code: 0xFDF,
        cycles: 7,
        operation: ret_operation,
    },
    Opcode{
        name: "RETS",
        code: 0xFDE,
        cycles: 12,
        operation: ret_s_operation,
    },
    Opcode{
        name: "RETD",
        code: 0x100,
        cycles: 12,
        operation: ret_d_operation,
    },
    Opcode{
        name: "INC_X",
        code: 0xEE0,
        cycles: 5,
        operation: inc_x_operation,
    },
    Opcode{
        name: "INC_Y",
        code: 0xEF0,
        cycles: 5,
        operation: inc_y_operation,
    },
    Opcode{
        name: "LD_X",
        code: 0xB00,
        cycles: 5,
        operation: ld_x_operation,
    },
    Opcode{
        name: "LD_Y",
        code: 0x800,
        cycles: 5,
        operation: ld_y_operation,
    },
    Opcode{
        name: "LD_XPR",
        code: 0xE80,
        cycles: 5,
        operation: ld_xpr_operation,
    },
    Opcode{
        name: "LD_XHR",
        code: 0xE84,
        cycles: 5,
        operation: ld_xhr_operation,
    },
    Opcode{
        name: "LD_XLR",
        code: 0xE88,
        cycles: 5,
        operation: ld_xlr_operation
    },
    Opcode{
        name: "LD_YPR",
        code: 0xE90,
        cycles: 5,
        operation: ld_ypr_operation
    },
    Opcode{
        name: "LD_YHR",
        code: 0xE94,
        cycles: 5,
        operation: ld_yhr_operation,
    },
    Opcode{
        name: "LD_YLR",
        code: 0xE98,
        cycles: 5,
        operation: ld_ylr_operation,
    },
    Opcode{
        name: "LD_RXP",
        code: 0xEA0,
        cycles: 5,
        operation: ld_rxp_operation,
    },
    Opcode{
        name: "LD_RXH",
        code: 0xEA4,
        cycles: 5,
        operation: ld_rxh_operation,
    },
    Opcode{
        name: "LD_RXL",
        code: 0xEA8,
        cycles: 5,
        operation: ld_rxl_operation,
    },
    Opcode{
        name: "LD_RYP",
        code: 0xEB0,
        cycles: 5,
        operation: ld_ryp_operation,
    },
    Opcode{
        name: "LD_RYH",
        code: 0xEB4,
        cycles: 5,
        operation: ld_ryh_operation,
    },
    Opcode{
        name: "LD_YL",
        code: 0xEB8,
        cycles: 5,
        operation: ld_yl_operation,
    },
    Opcode{
        name: "ADC_XH",
        code: 0xA00,
        cycles: 7,
        operation: adc_xh_operation,
    },
    Opcode{
        name: "ADC_XL",
        code: 0xA10,
        cycles: 7,
        operation: adc_xl_operation,
    },
    Opcode{
        name: "ADC_YH",
        code: 0xA20,
        cycles: 7,
        operation: adc_yh_operation,
    },
    Opcode{
        name: "ADC_YL",
        code: 0xA30,
        cycles: 7,
        operation: adc_yl_operation,
    },
    Opcode{
        name: "CP_XH",
        code: 0xA40,
        cycles: 7,
        operation: cp_xh_operation,
    },
    Opcode{
        name: "CP_XL",
        code: 0xA50,
        cycles: 7,
        operation: cp_xh_operation,
    },
    Opcode{
        name: "CP_YH",
        code: 0xA60,
        cycles: 7,
        operation: cp_yh_operation,
    },
    Opcode{
        name: "CP_YL",
        code: 0xA70,
        cycles: 7,
        operation: cp_yl_operation,
    },
    Opcode{
        name: "LD_R",
        code: 0xE00,
        cycles: 5,
        operation: ld_r_operation,
    },
    Opcode{
        name: "LD_RQ",
        code: 0xEC0,
        cycles: 5,
        operation: ld_rq_operation,
    },
    Opcode{
        name: "LD_AM",
        code: 0xFA0,
        cycles: 5,
        operation: ld_am_operation,
    },
    Opcode{
        name: "LD_BM",
        code: 0xFB0,
        cycles: 5,
        operation: ld_bm_operation
    },
    Opcode{
        name: "LD_MA",
        code: 0xF80,
        cycles: 5,
        operation: ld_ma_operation
    },
    Opcode{
        name: "LD_MB",
        code: 0xF90,
        cycles: 5,
        operation: ld_mb_operation,
    },
    Opcode{
        name: "LDPX_MX",
        code: 0xE60,
        cycles: 5,
        operation:ldpx_mx_operation,
    },
    Opcode{
        name: "LDPX_RQ",
        code: 0xEE0,
        cycles: 5,
        operation: ldpx_rq_operation,
    },
    Opcode{
        name: "LDPY_MY",
        code: 0xE70,
        cycles: 5,
        operation: ldpy_my_operation,
    },
    Opcode{
        name: "LDPY_RQ",
        code: 0xEF0,
        cycles: 5,
        operation: ldpy_rq_operation,
    },
    Opcode{
        name: "LBPX",
        code: 0x900,
        cycles: 5,
        operation: lbpx_operation,
    },
    Opcode{
        name: "SET",
        code: 0xF40,
        cycles: 7,
        operation: set_operation,
    },
    Opcode{
        name: "RST",
        code: 0xF50,
        cycles: 7,
        operation: rst_operation
    },
    Opcode{
        name: "SCF",
        code: 0xF41,
        cycles: 7,
        operation: scf_operation,
    },
    Opcode{
        name: "RCF",
        code: 0xF5E,
        cycles: 7,
        operation: rcf_operation,
    },
    Opcode{
        name: "SZF",
        code: 0xF42,
        cycles: 7,
        operation: szf_operation,
    },
    Opcode{
        name: "RZF",
        code: 0xF5D,
        cycles: 7,
        operation: rzf_operation,
    },
    Opcode{
        name: "SDF",
        code: 0xF44,
        cycles: 7,
        operation: sdf_operation,
    },
    Opcode{
        name: "RDF",
        code: 0xF5B,
        cycles: 7,
        operation: rdf_operation,
    },
    Opcode{
        name: "EI",
        code: 0xF48,
        cycles: 7,
        operation: ei_operation,
    },
    Opcode{
        name: "DI",
        code: 0xF57,
        cycles: 7,
        operation: di_operation,
    },
    Opcode{
        name: "INC",
        code: 0xFDB,
        cycles: 5,
        operation: inc_operation,
    },
    Opcode{
        name: "DEC",
        code: 0xFCB,
        cycles: 5,
        operation: dec_operation,
    },
    Opcode{
        name: "PUSH_R",
        code: 0xFC0,
        cycles: 5,
        operation: push_r_operation,
    },
    Opcode{
        name: "PUSH_XP",
        code: 0xFC4,
        cycles: 5,
        operation: push_xp_operation,
    },
    Opcode{
        name: "PUSH_XH",
        code: 0xFC5,
        cycles: 5,
        operation: push_xh_operation,
    },
    Opcode{
        name: "PUSH_XL",
        code: 0xFC6,
        cycles: 5,
        operation: push_xl_operation,
    },
    Opcode{
        name: "PUSH_YP",
        code: 0xFC7,
        cycles: 5,
        operation: push_yp_operation,
    },
    Opcode{
        name: "PUSH_YH",
        code: 0xFC8,
        cycles: 5,
        operation: push_yh_operation
    },
    Opcode{
        name: "PUSH_YL",
        code: 0xFC9,
        cycles: 5,
        operation: push_yl_operation
    },
    Opcode{
        name: "PUSH_F",
        code: 0xFCA,
        cycles: 5,
        operation: push_f_operation,
    },
    Opcode{
        name: "POP_R",
        code: 0xFD0,
        cycles: 5,
        operation: pop_r_operation,
    },
    Opcode{
        name: "POP_XP",
        code: 0xFD4,
        cycles: 5,
        operation: pop_xp_operation,
    },
    Opcode{
        name: "POP_XH",
        code: 0xFD5,
        cycles: 5,
        operation: pop_xh_operation,
    },
    Opcode{
        name: "POP_XL",
        code: 0xFD6,
        cycles: 5,
        operation: pop_xl_operation,
    },
    Opcode{
        name: "POP_YP",
        code: 0xFD7,
        cycles: 5,
        operation: pop_yp_operation,
    },
    Opcode{
        name: "POP_YH",
        code: 0xFD8,
        cycles: 5,
        operation: pop_yh_operation,
    },
    Opcode{
        name: "POP_YL",
        code: 0xFD9,
        cycles: 5,
        operation: pop_yl_operation,
    },
    Opcode{
        name: "POP_F",
        code: 0xFDA,
        cycles: 5,
        operation: pop_f_operation,
    },
    Opcode{
        name: "LD_SPH",
        code: 0xFE0,
        cycles: 5,
        operation: ld_sph_operation,
    },
    Opcode{
        name: "LD_SPL",
        code: 0xFF0,
        cycles: 5,
        operation: ld_spl_operation,
    },
    Opcode{
        name: "LD_RSPH",
        code: 0xFE4,
        cycles: 5,
        operation: ld_rsph_operation,
    },
    Opcode{
        name: "LD_RSPL",
        code: 0xFF4,
        cycles: 5,
        operation: ld_rspl_operation,
    },
    Opcode{
        name: "ADD_R",
        code: 0xC00,
        cycles: 7,
        operation: add_r_operation,
    },
    Opcode{
        name: "ADD_RQ",
        code: 0xA80,
        cycles: 7,
        operation: add_rq_operation,
    },
    Opcode{
        name: "ADC_R",
        code: 0xC40,
        cycles: 7,
        operation: adc_r_operation,
    },
    Opcode{
        name: "ADC_RQ",
        code: 0xA90,
        cycles: 7,
        operation: adc_rq_operation,
    },
    Opcode{
        name: "SUB",
        code: 0xAA0,
        cycles: 7,
        operation: sub_operation,
    },
    Opcode{
        name: "SBC_R",
        code: 0xB40,
        cycles: 7,
        operation: sbc_r_operation,
    },
    Opcode{
        name: "SBC_RQ",
        code: 0xAB0,
        cycles: 7,
        operation: sbc_rq_operation,
    },
    Opcode{
        name: "AND_R",
        code: 0xC80,
        cycles: 7,
        operation: and_r_operation,
    },
    Opcode{
        name: "AND_RQ",
        code: 0xAC0,
        cycles: 7,
        operation: and_rq_operation,
    },
    Opcode{
        name: "OR_R",
        code: 0xCC0,
        cycles: 7,
        operation: or_r_operation,
    },Opcode{
        name: "OR_RQ",
        code: 0xAD0,
        cycles: 7,
        operation: or_rq_operation,
    },
    Opcode{
        name: "XOR_R",
        code: 0xD00,
        cycles: 7,
        operation: xor_r_operation,
    },
    Opcode{
        name: "XOR_RQ",
        code: 0xAE0,
        cycles: 7,
        operation: xor_rq_operation,
    },
    Opcode{
        name: "CP_R",
        code: 0xDC0,
        cycles: 7,
        operation: cp_r_operation,
    },
    Opcode{
        name: "CP_RQ",
        code: 0xF00,
        cycles: 7,
        operation: cp_rq_operation,
    },
    Opcode{
        name: "FAN_R",
        code: 0xD80,
        cycles: 7,
        operation: fan_r_operation,
    },
    Opcode{
        name: "FAN_RQ",
        code: 0xF10,
        cycles: 7,
        operation: fan_rq_operation,
    },
    Opcode{
        name: "RLC",
        code: 0xAF0,
        cycles: 7,
        operation: rlc_operation,
    },
    Opcode{
        name: "RRC",
        code: 0xE8C,
        cycles: 5,
        operation: rrc_operation,
    },
    Opcode{
        name: "INC_M",
        code: 0xF60,
        cycles: 7,
        operation: inc_m_operation,
    },
    Opcode{
        name: "DEC_M",
        code: 0xF70,
        cycles: 7,
        operation: dec_m_operation,
    },
    Opcode{
        name: "ACPX",
        code: 0xF28,
        cycles: 7,
        operation: acpx_operation,
    },
    Opcode{
        name: "ACPY",
        code: 0xF2C,
        cycles: 7,
        operation: acpy_operation,
    },
    Opcode{
        name: "SCPX",
        code: 0xF38,
        cycles: 7,
        operation: scpx_operation,
    },
    Opcode{
        name: "SCPY",
        code: 0xF3C,
        cycles: 7,
        operation: scpy_operation,
    },
    Opcode{
        name: "NOT",
        code: 0xD0F,
        cycles: 7,
        operation: not_operation
    },
];