#[path = "./ram.rs"]
mod ram;

const FLAG_C: u16 = 0x1;
const FLAG_Z: u16 = 0x1 << 1;
const FLAG_D: u16 = 0x1 << 2;
const FLAG_I: u16 = 0x1 << 3;

const MASK_4B:u16 =0xF00;
const MASK_6B:u16 =0xFC0;
const MASK_7B:u16 =0xFE0;
const MASK_8B:u16 =0xFF0;
const MASK_10B:u16 =0xFFC;
const MASK_12B:u16 =0xFFF;

fn clear_flag_c(flags: u16) -> u16{
    return flags & !FLAG_C;
}
fn clear_flag_z(flags: u16) -> u16{
    return flags & !FLAG_Z;
}
fn clear_flag_d(flags: u16) -> u16{
    return flags & !FLAG_D;
}
fn clear_flag_i(flags: u16) -> u16{
    return flags & !FLAG_I;
}

fn set_flag_c(flags: u16) -> u16{
    return flags | FLAG_C
}
fn set_flag_z(flags: u16) -> u16{
    return flags | FLAG_Z
}
fn set_flag_d(flags: u16) -> u16{
    return flags | FLAG_D
}
fn set_flag_i(flags: u16) -> u16{
    return flags | FLAG_I
}

pub struct Opcode {
    pub name: &'static str,
    pub code: u16,
    pub cycles: u8,
    pub operation: unsafe fn (*mut super::CPU, u16),
    pub mask: u16,
}

unsafe fn get_rq (cpu: *mut super::CPU, rq: u16) -> u16{
    match rq{
        0x0 => return (*cpu).register_a,
        0x1 => return (*cpu).register_b,
        0x2 => return ram::get_memory(cpu, (*cpu).register_x),
        0x3 => return ram::get_memory(cpu, (*cpu).register_y),
        _ => return 0,
    }
}

unsafe fn set_rq (cpu: *mut super::CPU, rq: u16, value: u16){
    match rq{
        0x0 => {
            println!("a value {:#05x}", value);
            (*cpu).register_a = value;
            //(*cpu).register_a = 0x6;
        }
        0x1 => {
            (*cpu).register_b = value;
        }
        0x2 => ram::set_memory(cpu, (*cpu).register_x, value),
        0x3 => ram::set_memory(cpu, (*cpu).register_y, value),
        _ => println!("ERROR"),
    }
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
    (*cpu).next_program_counter = (step & 0xFF) | ((*cpu).new_pointer << 8);
}
unsafe fn jp_cs_operation(cpu: *mut super::CPU, step: u16){
    if ((*cpu).flags & FLAG_C) > 0 {
        (*cpu).next_program_counter = (step & 0xFF) | ((*cpu).new_pointer << 8);
    }
}
unsafe fn jp_ncs_operation(cpu: *mut super::CPU, step: u16){
    if ((*cpu).flags & FLAG_C) == 0 {
        (*cpu).next_program_counter = (step & 0xFF) | ((*cpu).new_pointer << 8);
    }
}
unsafe fn jp_zs_operation(cpu: *mut super::CPU, step: u16){
    if ((*cpu).flags & FLAG_Z) > 0 {
        (*cpu).next_program_counter = (step & 0xFF) | ((*cpu).new_pointer << 8);
    }
}
unsafe fn jp_nzs_operation(cpu: *mut super::CPU, step: u16){
    if ((*cpu).flags & FLAG_Z) == 0 {
        (*cpu).next_program_counter = (step & 0xFF) | ((*cpu).new_pointer << 8);
    }
}
unsafe fn jpba_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).next_program_counter = ((*cpu).register_a | ((*cpu).register_b << 4)) | ((*cpu).new_pointer << 8)
}
unsafe fn call_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).program_counter = ((*cpu).program_counter + 1) & 0x1FFF;
    ram::set_memory(cpu, ((*cpu).stack_pointer) - 1, ((*cpu).program_counter >> 8) & 0xF);
    ram::set_memory(cpu, ((*cpu).stack_pointer) - 2, ((*cpu).program_counter >> 4) & 0xF);
    ram::set_memory(cpu, ((*cpu).stack_pointer) - 3, (*cpu).program_counter & 0xF);
    (*cpu).stack_pointer = ((*cpu).stack_pointer - 3) & 0xFF;
    (*cpu).next_program_counter = (step & 0xFF) | ((*cpu).new_pointer & 0xF) << 8 | (((*cpu).program_counter >> 12) & 0x1) << 12
}
unsafe fn callz_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).program_counter = ((*cpu).program_counter + 1) & 0x1FFF;
    ram::set_memory(cpu, ((*cpu).stack_pointer) - 1, ((*cpu).program_counter >> 8) & 0xF);
    ram::set_memory(cpu, ((*cpu).stack_pointer) - 2, ((*cpu).program_counter >> 4) & 0xF);
    ram::set_memory(cpu, ((*cpu).stack_pointer) - 3, (*cpu).program_counter & 0xF);
    (*cpu).stack_pointer = ((*cpu).stack_pointer - 3) & 0xFF;
    (*cpu).next_program_counter = (step & 0xFF) | 0 << 8 | (((*cpu).program_counter >> 12) & 0x1) << 12
}
unsafe fn ret_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).next_program_counter = ram::get_memory(cpu,(*cpu).stack_pointer) | ram::get_memory(cpu,(*cpu).stack_pointer + 1) << 4 | ram::get_memory(cpu,(*cpu).stack_pointer + 2) << 8 | ((*cpu).program_counter >> 12) & 0x1;
    (*cpu).stack_pointer = ((*cpu).stack_pointer + 3) & 0xFF;
}
unsafe fn ret_s_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).next_program_counter = ram::get_memory(cpu,(*cpu).stack_pointer) | ram::get_memory(cpu,(*cpu).stack_pointer + 1) << 4 | ram::get_memory(cpu,(*cpu).stack_pointer + 2) << 8 | ((*cpu).program_counter >> 12) & 0x1;
    (*cpu).stack_pointer = ((*cpu).stack_pointer + 3) & 0xFF;
    (*cpu).next_program_counter = ((*cpu).program_counter + 1) & 0x1FFF;
}
unsafe fn ret_d_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).next_program_counter = ram::get_memory(cpu,(*cpu).stack_pointer) | ram::get_memory(cpu,(*cpu).stack_pointer + 1) << 4 | ram::get_memory(cpu,(*cpu).stack_pointer + 2) << 8 | ((*cpu).program_counter >> 12) & 0x1;
    (*cpu).stack_pointer = ((*cpu).stack_pointer + 3) & 0xFF;
    ram::set_memory(cpu, (*cpu).register_x, (step & 0xFF));
    ram::set_memory(cpu, (*cpu).register_x + 1, ((step & 0xFF) >> 4) & 0xF);
    (*cpu).register_x = ((*cpu).register_x + 2) & 0xFF | (((*cpu).register_x >> 8) & 0xF);
}
unsafe fn inc_x_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).register_x = (((*cpu).register_x + 1) & 0xF) | ((((*cpu).register_x >> 8) & 0xF) << 8)
}
unsafe fn inc_y_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).register_y = ((*cpu).register_y + 1) & 0xFF | ((((*cpu).register_y >> 8) & 0xF) << 8);
}
unsafe fn ld_x_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).register_x = (step & 0xFF) | ((((*cpu).register_x >> 8) & 0xF) << 8);
}
unsafe fn ld_y_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).register_y = (step & 0xFF) | ((((*cpu).register_y >> 8) & 0xF) << 8);
}
unsafe fn ld_xpr_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).register_x = ((*cpu).register_x & 0xFF) | (get_rq(cpu, (step & 0x3)) << 8);
}
unsafe fn ld_xhr_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).register_x = ((*cpu).register_x & 0xF) | (get_rq(cpu, (step & 0x3)) << 4) | ((((*cpu).register_x >> 8) & 0xF) << 8);
}
unsafe fn ld_xlr_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).register_x = (get_rq(cpu, (step & 0x3))) | (((*cpu).register_x >> 4 & 0xF) << 4) | ((((*cpu).register_x >> 8) & 0xF) << 8);
}
unsafe fn ld_ypr_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).register_y = ((*cpu).register_y & 0xFF) | (get_rq(cpu, (step & 0x3)) << 8);
}
unsafe fn ld_yhr_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).register_y = ((*cpu).register_y & 0xF) | (get_rq(cpu, (step & 0x3)) << 4) | ((((*cpu).register_y >> 8) & 0xF) << 8);
}
unsafe fn ld_ylr_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).register_y = (get_rq(cpu, (step & 0x3))) | (((*cpu).register_y >> 4 & 0xF) << 4) | ((((*cpu).register_y >> 8) & 0xF) << 8);
}
unsafe fn ld_rxp_operation(cpu: *mut super::CPU, step: u16){
    set_rq(cpu, step & 0x3,((*cpu).register_x >> 8) & 0xF);
}
unsafe fn ld_rxh_operation(cpu: *mut super::CPU, step: u16){
    set_rq(cpu, step & 0x3,((*cpu).register_x >> 4) & 0xF);
}
unsafe fn ld_rxl_operation(cpu: *mut super::CPU, step: u16){
    set_rq(cpu, step & 0x3,(*cpu).register_x & 0xF);
}
unsafe fn ld_ryp_operation(cpu: *mut super::CPU, step: u16){
    set_rq(cpu, step & 0x3,((*cpu).register_y >> 4) & 0xF);
}
unsafe fn ld_ryh_operation(cpu: *mut super::CPU, step: u16){
    set_rq(cpu, step & 0x3,((*cpu).register_y >> 4) & 0xF);
}
unsafe fn ld_yl_operation(cpu: *mut super::CPU, step: u16){
    set_rq(cpu, step & 0x3,(*cpu).register_y & 0xF);
}
unsafe fn adc_xh_operation(cpu: *mut super::CPU, step: u16){
    let mut temp: u16 = (((*cpu).register_x >> 4) & 0xF) + (step & 0xF) + ((*cpu).flags & FLAG_C);
    (*cpu).register_x = ((*cpu).register_x & 0xF) | ((temp & 0x4) << 4) | (((*cpu).register_x >> 8) & 0xF);

    if (temp >> 4) > 0 {
        (*cpu).flags = set_flag_c((*cpu).flags);
    }else {
        (*cpu).flags = clear_flag_c((*cpu).flags);
    }

    if (temp & 0xF) == 0 {
        (*cpu).flags = set_flag_z((*cpu).flags);
    }else {
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn adc_xl_operation(cpu: *mut super::CPU, step: u16){
    let mut temp: u16 = ((*cpu).register_x & 0xF) + (step & 0xF) + FLAG_C;
    (*cpu).register_x = (temp & 0x4) | ((((*cpu).register_x >> 4) & 0xF) << 4) | ((((*cpu).register_x >> 8) & 0xF) << 4);

    if (temp >> 4) > 0 {
        (*cpu).flags = set_flag_c((*cpu).flags);
    }else {
        (*cpu).flags = clear_flag_c((*cpu).flags);
    }

    if (temp & 0xF) == 0 {
        (*cpu).flags = set_flag_z((*cpu).flags);
    }else {
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn adc_yh_operation(cpu: *mut super::CPU, step: u16){
    let mut temp: u16 = (((*cpu).register_y >> 4) & 0xF) + (step & 0xF) + ((*cpu).flags & FLAG_C);
    (*cpu).register_y = ((*cpu).register_y & 0xF) | ((temp & 0x4) << 4) | (((*cpu).register_y >> 8) & 0xF);

    if (temp >> 4) > 0 {
        (*cpu).flags = set_flag_c((*cpu).flags);
    }else {
        (*cpu).flags = clear_flag_c((*cpu).flags);
    }

    if (temp & 0xF) == 0 {
        (*cpu).flags = set_flag_z((*cpu).flags);
    }else {
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn adc_yl_operation(cpu: *mut super::CPU, step: u16){
    let mut temp: u16 = ((*cpu).register_y & 0xF) + (step & 0xF) + ((*cpu).flags & FLAG_C);
    (*cpu).register_y = (temp & 0x4) | ((((*cpu).register_y >> 4) & 0xF) << 4) | ((((*cpu).register_y >> 8) & 0xF) << 4);

    if (temp >> 4) > 0 {
        (*cpu).flags = set_flag_c((*cpu).flags);
    }else {
        (*cpu).flags = clear_flag_c((*cpu).flags);
    }

    if (temp & 0xF) == 0 {
        (*cpu).flags = set_flag_z((*cpu).flags);
    }else {
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn cp_xh_operation(cpu: *mut super::CPU, step: u16){
    if (((*cpu).register_x >> 4) & 0xF) < (step & 0xF){
        (*cpu).flags = set_flag_c((*cpu).flags);
    }else {
        (*cpu).flags = clear_flag_c((*cpu).flags);
    }

    if (((*cpu).register_x >> 4) & 0xF) == (step & 0xF){
        (*cpu).flags = set_flag_z((*cpu).flags);
    }else {
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn cp_xl_operation(cpu: *mut super::CPU, step: u16){
    if ((*cpu).register_x & 0xF) < (step & 0xF){
        (*cpu).flags = set_flag_c((*cpu).flags);
    }else {
        (*cpu).flags = clear_flag_c((*cpu).flags);
    }

    if ((*cpu).register_x & 0xF) == (step & 0xF){
        (*cpu).flags = set_flag_z((*cpu).flags);
    }else {
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn cp_yh_operation(cpu: *mut super::CPU, step: u16){
    if (((*cpu).register_y >> 4) & 0xF) < (step & 0xF){
        (*cpu).flags = set_flag_c((*cpu).flags);
    }else {
        (*cpu).flags = clear_flag_c((*cpu).flags);
    }

    if (((*cpu).register_y >> 4) & 0xF) == (step & 0xF){
        (*cpu).flags = set_flag_z((*cpu).flags);
    }else {
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn cp_yl_operation(cpu: *mut super::CPU, step: u16){
    if ((*cpu).register_y & 0xF) < (step & 0xF){
        (*cpu).flags = set_flag_c((*cpu).flags);
    }else {
        (*cpu).flags = clear_flag_c((*cpu).flags);
    }

    if ((*cpu).register_y & 0xF) == (step & 0xF){
        (*cpu).flags = set_flag_z((*cpu).flags);
    }else {
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn ld_r_operation(cpu: *mut super::CPU, step: u16){
    set_rq(cpu, (step >> 4) & 0x3, step & 0xF);
}
unsafe fn ld_rq_operation(cpu: *mut super::CPU, step: u16){
    set_rq(cpu, (step >> 2) & 0x3, get_rq(cpu, step & 0x3));
}
unsafe fn ld_am_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).register_a = ram::get_memory(cpu, step & 0xF);
}
unsafe fn ld_bm_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).register_b = ram::get_memory(cpu, step & 0xF);
}
unsafe fn ld_ma_operation(cpu: *mut super::CPU, step: u16){
    ram::set_memory(cpu, step & 0xF, (*cpu).register_a);
}
unsafe fn ld_mb_operation(cpu: *mut super::CPU, step: u16){
    ram::set_memory(cpu, step & 0xF, (*cpu).register_b);
}
unsafe fn ldpx_mx_operation(cpu: *mut super::CPU, step: u16){
    ram::set_memory(cpu, (*cpu).register_x, step & 0xF);
    (*cpu).register_x = (((*cpu).register_x + 1) & 0xFF) | ((((*cpu).register_x >> 8) & 0xF) << 8);
}
unsafe fn ldpx_rq_operation(cpu: *mut super::CPU, step: u16){
    set_rq(cpu, (step >> 2) & 0x3, get_rq(cpu, step & 0x2));
    (*cpu).register_x = (((*cpu).register_x + 1) & 0xFF) | ((((*cpu).register_x >> 8) & 0xF) << 8);
}
unsafe fn ldpy_my_operation(cpu: *mut super::CPU, step: u16){
    ram::set_memory(cpu, (*cpu).register_y, step & 0xF);
    (*cpu).register_y = (((*cpu).register_y + 1) & 0xFF) | (((*cpu).register_y >> 8) & 0xF);
}
unsafe fn ldpy_rq_operation(cpu: *mut super::CPU, step: u16){
    set_rq(cpu, (step >> 2) & 0x3, get_rq(cpu, step & 0x2));
    (*cpu).register_y = (((*cpu).register_y + 1) & 0xFF) | (((*cpu).register_y >> 8) & 0xF);
}
unsafe fn lbpx_operation(cpu: *mut super::CPU, step: u16){
    ram::set_memory(cpu, (*cpu).register_x, step & 0xF);
    ram::set_memory(cpu, (*cpu).register_x + 1, (step >> 4) & 0xF);
    (*cpu).register_x = (((*cpu).register_x + 2) & 0xFF) | ((((*cpu).register_x >> 8) & 0xF) << 8);
}
unsafe fn set_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).flags = (*cpu).flags | (step & 0x4)
}
unsafe fn rst_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).flags = (*cpu).flags & (step & 0x4)
}
unsafe fn scf_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).flags = set_flag_c((*cpu).flags);
}
unsafe fn rcf_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).flags = clear_flag_c((*cpu).flags);
}
unsafe fn szf_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).flags = set_flag_z((*cpu).flags);
}
unsafe fn rzf_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).flags = clear_flag_z((*cpu).flags);
}
unsafe fn sdf_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).flags = set_flag_d((*cpu).flags);
}
unsafe fn rdf_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).flags = clear_flag_d((*cpu).flags);
}
unsafe fn ei_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).flags = set_flag_i((*cpu).flags);
}
unsafe fn di_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).flags = clear_flag_i((*cpu).flags);
}
unsafe fn inc_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).stack_pointer = ((*cpu).stack_pointer + 1) & 0xFF;
}
unsafe fn dec_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).stack_pointer = ((*cpu).stack_pointer - 1) & 0xFF;
}
unsafe fn push_r_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).stack_pointer = ((*cpu).stack_pointer - 1) & 0xFF;
    ram::set_memory(cpu, (*cpu).stack_pointer, get_rq(cpu, step & 0x3));
}
unsafe fn push_xp_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).stack_pointer = ((*cpu).stack_pointer - 1) & 0xFF;
    ram::set_memory(cpu, (*cpu).stack_pointer, (((*cpu).register_x >> 8) & 0xF));
}
unsafe fn push_xh_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).stack_pointer = ((*cpu).stack_pointer - 1) & 0xFF;
    ram::set_memory(cpu, (*cpu).stack_pointer, (((*cpu).register_x >> 4) & 0xF));
}
unsafe fn push_xl_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).stack_pointer = ((*cpu).stack_pointer - 1) & 0xFF;
    ram::set_memory(cpu, (*cpu).stack_pointer, (*cpu).register_x & 0xF);
}
unsafe fn push_yp_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).stack_pointer = ((*cpu).stack_pointer - 1) & 0xFF;
    ram::set_memory(cpu, (*cpu).stack_pointer, (((*cpu).register_y >> 8) & 0xF));
}
unsafe fn push_yh_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).stack_pointer = ((*cpu).stack_pointer - 1) & 0xFF;
    ram::set_memory(cpu, (*cpu).stack_pointer, (((*cpu).register_y >> 4) & 0xF));
}
unsafe fn push_yl_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).stack_pointer = ((*cpu).stack_pointer - 1) & 0xFF;
    ram::set_memory(cpu, (*cpu).stack_pointer, (*cpu).register_y & 0xF);
}
unsafe fn push_f_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).stack_pointer = ((*cpu).stack_pointer - 1) & 0xFF;
    ram::set_memory(cpu, (*cpu).stack_pointer, (*cpu).flags);
}
unsafe fn pop_r_operation(cpu: *mut super::CPU, step: u16){
    set_rq(cpu, step & 0x3, ram::get_memory(cpu, (*cpu).stack_pointer));
    (*cpu).stack_pointer = ((*cpu).stack_pointer + 1) & 0xFF;
}
unsafe fn pop_xp_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).register_x = ((*cpu).register_x & 0x4) | ((((*cpu).register_x >> 4) & 0xF) << 4) | (ram::get_memory(cpu, (*cpu).stack_pointer) << 8);
    (*cpu).stack_pointer = ((*cpu).stack_pointer + 1) & 0xFF;
}
unsafe fn pop_xh_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).register_x = ((*cpu).register_x & 0x4) | (ram::get_memory(cpu, (*cpu).stack_pointer) << 4) | ((((*cpu).register_x >> 8) & 0xF) << 8);
    (*cpu).stack_pointer = ((*cpu).stack_pointer + 1) & 0xFF;
}
unsafe fn pop_xl_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).register_x = ram::get_memory(cpu, (*cpu).stack_pointer) | ((((*cpu).register_x >> 4) & 0xF) << 4) | ((((*cpu).register_x >> 8) & 0xF) << 8);
    (*cpu).stack_pointer = ((*cpu).stack_pointer + 1) & 0xFF;
}
unsafe fn pop_yp_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).register_y = ((*cpu).register_y & 0x4) | ((((*cpu).register_y >> 4) & 0xF) << 4) | (ram::get_memory(cpu, (*cpu).stack_pointer) << 8);
    (*cpu).stack_pointer = ((*cpu).stack_pointer + 1) & 0xFF;
}
unsafe fn pop_yh_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).register_y = ((*cpu).register_y & 0x4) | (ram::get_memory(cpu, (*cpu).stack_pointer) << 4) | ((((*cpu).register_y >> 8) & 0xF) << 8);
    (*cpu).stack_pointer = ((*cpu).stack_pointer + 1) & 0xFF;
}
unsafe fn pop_yl_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).register_y = ram::get_memory(cpu, (*cpu).stack_pointer) | ((((*cpu).register_y >> 4) & 0xF) << 4) | ((((*cpu).register_y >> 8) & 0xF) << 8);
    (*cpu).stack_pointer = ((*cpu).stack_pointer + 1) & 0xFF;
}
unsafe fn pop_f_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).flags = ram::get_memory(cpu, (*cpu).stack_pointer);
    (*cpu).stack_pointer = ((*cpu).stack_pointer + 1) & 0xFF;
}
unsafe fn ld_sph_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).stack_pointer = ((*cpu).stack_pointer & 0x4) | (get_rq(cpu, step & 0x3) << 4);
}
unsafe fn ld_spl_operation(cpu: *mut super::CPU, step: u16){
    (*cpu).stack_pointer = get_rq(cpu, step & 0x3) | ((((*cpu).stack_pointer >> 4) & 0xF) << 4);
}
unsafe fn ld_rsph_operation(cpu: *mut super::CPU, step: u16){
    set_rq(cpu, step & 0x3 ,((*cpu).stack_pointer >> 4) & 0xf)
}
unsafe fn ld_rspl_operation(cpu: *mut super::CPU, step: u16){
    set_rq(cpu, step & 0x3 ,(*cpu).stack_pointer & 0xf)
}
unsafe fn add_r_operation(cpu: *mut super::CPU, step: u16){
    let mut temp: u16 = get_rq(cpu, (step >> 4) & 0x3) + (step & 0xF);
    if ((*cpu).flags & FLAG_D) > 0{
        if (temp >= 10){
            set_rq(cpu, (step >> 4) & 0x3 ,(temp - 10) & 0xF);
            (*cpu).flags = set_flag_c((*cpu).flags);
        }else{
            set_rq(cpu, (step >> 4) & 0x3 ,temp);
            (*cpu).flags = clear_flag_c((*cpu).flags);
        }
    } else{
        set_rq(cpu, (step >> 4) & 0x3 ,temp & 0xF);
        if (temp >> 4) > 0{
            (*cpu).flags = set_flag_c((*cpu).flags);
        }else{
            (*cpu).flags = clear_flag_c((*cpu).flags);
        }
    }

    if get_rq(cpu, (step >> 4) & 0x3) == 0{
        (*cpu).flags = set_flag_z((*cpu).flags);
    }else{
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn add_rq_operation(cpu: *mut super::CPU, step: u16){
    let mut temp: u16 = get_rq(cpu, (step >> 2) & 0x3) + get_rq(cpu, (step) & 0x3);
    if ((*cpu).flags & FLAG_D) > 0{
        if (temp >= 10){
            set_rq(cpu, (step >> 2) & 0x3 ,(temp - 10) & 0xF);
            (*cpu).flags = set_flag_c((*cpu).flags);
        }else{
            set_rq(cpu, (step >> 2) & 0x3 ,temp);
            (*cpu).flags = clear_flag_c((*cpu).flags);
        }
    } else{
        set_rq(cpu, (step >> 2) & 0x3 ,temp & 0xF);
        if (temp >> 4) > 0{
            (*cpu).flags = set_flag_c((*cpu).flags);
        }else{
            (*cpu).flags = clear_flag_c((*cpu).flags);
        }
    }

    if get_rq(cpu, (step >> 2) & 0x3) == 0{
        (*cpu).flags = set_flag_z((*cpu).flags);
    }else{
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn adc_r_operation(cpu: *mut super::CPU, step: u16){
    let mut temp: u16 = get_rq(cpu, (step >> 4) & 0x3) + (step & 0xF) + ((*cpu).flags & FLAG_C);
    if ((*cpu).flags & FLAG_D) > 0{
        if (temp >= 10){
            set_rq(cpu, (step >> 2) & 0x3 ,(temp - 10) & 0xF);
            (*cpu).flags = set_flag_c((*cpu).flags);
        }else{
            set_rq(cpu, (step >> 2) & 0x3 ,temp);
            (*cpu).flags = clear_flag_c((*cpu).flags);
        }
    } else{
        set_rq(cpu, (step >> 2) & 0x3 ,temp & 0xF);
        if (temp >> 4) > 0{
            (*cpu).flags = set_flag_c((*cpu).flags);
        }else{
            (*cpu).flags = clear_flag_c((*cpu).flags);
        }
    }

    if get_rq(cpu, (step >> 4) & 0x3) == 0{
        (*cpu).flags = set_flag_z((*cpu).flags);
    }else{
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn adc_rq_operation(cpu: *mut super::CPU, step: u16){
    let mut temp: u16 = get_rq(cpu, (step >> 2) & 0x3) + get_rq(cpu, (step) & 0x3) + ((*cpu).flags & FLAG_C);
    if ((*cpu).flags & FLAG_D) > 0{
        if (temp >= 10){
            set_rq(cpu, (step >> 2) & 0x3 ,(temp - 10) & 0xF);
            (*cpu).flags = set_flag_c((*cpu).flags);
        }else{
            set_rq(cpu, (step >> 2) & 0x3 ,temp);
            (*cpu).flags = clear_flag_c((*cpu).flags);
        }
    } else{
        set_rq(cpu, (step >> 2) & 0x3 ,temp & 0xF);
        if (temp >> 4) > 0{
            (*cpu).flags = set_flag_c((*cpu).flags);
        }else{
            (*cpu).flags = clear_flag_c((*cpu).flags);
        }
    }

    if get_rq(cpu, (step >> 2) & 0x3) == 0{
        (*cpu).flags = set_flag_z((*cpu).flags);
    }else{
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn sub_operation(cpu: *mut super::CPU, step: u16){
    let mut temp: u16 = get_rq(cpu, (step >> 2) & 0x3) - get_rq(cpu, (step) & 0x3);
    if ((*cpu).flags & FLAG_D) > 0{
        if (temp >> 4) > 0{
            set_rq(cpu, (step >> 2) & 0x3 ,(temp - 6) & 0xF);
        }else{
            set_rq(cpu, (step >> 2) & 0x3 ,temp);
        }
    } else{
        set_rq(cpu, (step >> 2) & 0x3 ,temp & 0xF);
    }

    if (temp >> 4) > 0{
        (*cpu).flags = set_flag_c((*cpu).flags);
    }else{
        (*cpu).flags = clear_flag_c((*cpu).flags);
    }

    if get_rq(cpu, (step >> 2) & 0x3) == 0{
        (*cpu).flags = set_flag_z((*cpu).flags);
    }else{
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn sbc_r_operation(cpu: *mut super::CPU, step: u16){
    let mut temp: u16 = get_rq(cpu, (step >> 4) & 0x3) - (step & 0xF) - ((*cpu).flags & FLAG_C);
    if ((*cpu).flags & FLAG_D) > 0{
        if (temp >> 4) > 0{
            set_rq(cpu, (step >> 4) & 0x3 ,(temp - 6) & 0xF);
        }else{
            set_rq(cpu, (step >> 4) & 0x3 ,temp);
        }
    } else{
        set_rq(cpu, (step >> 4) & 0x3 ,temp & 0xF);
    }

    if (temp >> 4) > 0{
        (*cpu).flags = set_flag_c((*cpu).flags);
    }else{
        (*cpu).flags = clear_flag_c((*cpu).flags);
    }

    if get_rq(cpu, (step >> 4) & 0x4) == 0{
        (*cpu).flags = set_flag_z((*cpu).flags);
    }else{
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn sbc_rq_operation(cpu: *mut super::CPU, step: u16){
    let mut temp: u16 = get_rq(cpu, (step >> 2) & 0x3) - get_rq(cpu, step & 0x3) - ((*cpu).flags & FLAG_C);
    if ((*cpu).flags & FLAG_D) > 0{
        if (temp >> 4) > 0{
            set_rq(cpu, (step >> 2) & 0x3 ,(temp - 6) & 0xF);
        }else{
            set_rq(cpu, (step >> 2) & 0x3 ,temp);
        }
    } else{
        set_rq(cpu, (step >> 2) & 0x3 ,temp & 0xF);
    }

    if (temp >> 4) > 0{
        (*cpu).flags = set_flag_c((*cpu).flags);
    }else{
        (*cpu).flags = clear_flag_c((*cpu).flags);
    }

    if get_rq(cpu, (step >> 2) & 0x4) == 0{
        (*cpu).flags = set_flag_z((*cpu).flags);
    }else{
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn and_r_operation(cpu: *mut super::CPU, step: u16){
    set_rq(cpu, (step >> 4) & 0x3, get_rq(cpu,(step >> 4) & 0x3) & (step & 0xF));

    if get_rq(cpu,(step >> 4) & 0x3) == 0{
        (*cpu).flags = set_flag_z((*cpu).flags);
    }else{
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn and_rq_operation(cpu: *mut super::CPU, step: u16){
    set_rq(cpu, (step >> 2) & 0x3 , get_rq(cpu,(step >> 2) & 0x3) & get_rq(cpu,step & 0x3));

    if get_rq(cpu,(step >> 2) & 0x3) == 0{
        (*cpu).flags = set_flag_z((*cpu).flags);
    }else{
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn or_r_operation(cpu: *mut super::CPU, step: u16){
    set_rq(cpu, (step >> 4) & 0x3, get_rq(cpu,(step >> 4) & 0x3) | (step & 0xF));

    if get_rq(cpu,(step >> 4) & 0x3) == 0{
        (*cpu).flags = set_flag_z((*cpu).flags);
    }else{
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn or_rq_operation(cpu: *mut super::CPU, step: u16){
    set_rq(cpu, (step >> 2) & 0x3 , get_rq(cpu,(step >> 2) & 0x3) | get_rq(cpu,step & 0x3));

    if get_rq(cpu,(step >> 2) & 0x3) == 0{
        (*cpu).flags = set_flag_z((*cpu).flags);
    }else{
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn xor_r_operation(cpu: *mut super::CPU, step: u16){
    set_rq(cpu, (step >> 4) & 0x3, get_rq(cpu,(step >> 4) & 0x3) ^ (step & 0xF));

    if get_rq(cpu,(step >> 4) & 0x3) == 0{
        (*cpu).flags = set_flag_z((*cpu).flags);
    }else{
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn xor_rq_operation(cpu: *mut super::CPU, step: u16){
    set_rq(cpu, (step >> 2) & 0x3 , get_rq(cpu,(step >> 2) & 0x3) ^ get_rq(cpu,step & 0x3));

    if get_rq(cpu,(step >> 2) & 0x3) == 0{
        (*cpu).flags = set_flag_z((*cpu).flags);
    }else{
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn cp_r_operation(cpu: *mut super::CPU, step: u16){
    if get_rq(cpu,(step >> 4) & 0x3) < (step & 0xF){
        (*cpu).flags = set_flag_c((*cpu).flags);
    } else{
        (*cpu).flags = clear_flag_c((*cpu).flags);
    }

    if get_rq(cpu,(step >> 4) & 0x3) == (step & 0xF){
        (*cpu).flags = set_flag_z((*cpu).flags);
    } else{
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn cp_rq_operation(cpu: *mut super::CPU, step: u16){
    if get_rq(cpu,(step >> 2) & 0x3) < get_rq(cpu,step & 0x3){
        (*cpu).flags = set_flag_c((*cpu).flags);
    } else{
        (*cpu).flags = clear_flag_c((*cpu).flags);
    }

    if get_rq(cpu,(step >> 4) & 0x3) == get_rq(cpu,step & 0x3){
        (*cpu).flags = set_flag_z((*cpu).flags);
    } else{
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn fan_r_operation(cpu: *mut super::CPU, step: u16){
    if get_rq(cpu,(step >> 4) & 0x3) & (step & 0xF) == 0{
        (*cpu).flags = set_flag_z((*cpu).flags);
    } else{
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn fan_rq_operation(cpu: *mut super::CPU, step: u16){
    if get_rq(cpu,(step >> 2) & 0x3) & get_rq(cpu,step & 0x3) == 0{
        (*cpu).flags = set_flag_z((*cpu).flags);
    } else{
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn rlc_operation(cpu: *mut super::CPU, step: u16){
    let mut temp: u16 = (get_rq(cpu, step & 0xF) << 1) | ((*cpu).flags & FLAG_C);
    if (get_rq(cpu, step & 0xF) & 0x8) > 0{
        (*cpu).flags = set_flag_c((*cpu).flags);
    } else{
        (*cpu).flags = clear_flag_c((*cpu).flags);
    }
    set_rq(cpu, step & 0xF, temp & 0xF);
}
unsafe fn rrc_operation(cpu: *mut super::CPU, step: u16){
    let mut temp: u16 = (get_rq(cpu, step & 0x3) >> 1) | (((*cpu).flags & FLAG_C) << 3);
    if (get_rq(cpu, step & 0x3) & 0x1) > 0{
        (*cpu).flags = set_flag_c((*cpu).flags);
    } else{
        (*cpu).flags = clear_flag_c((*cpu).flags);
    }
    set_rq(cpu, step & 0x3, temp & 0xF);
}
unsafe fn inc_m_operation(cpu: *mut super::CPU, step: u16){
    let mut temp: u16 = ram::get_memory(cpu, step & 0xF) + 1;
    ram::set_memory(cpu, step & 0xF, temp & 0xF);

    if (temp >> 4) > 0{
        (*cpu).flags = set_flag_c((*cpu).flags);
    } else{
        (*cpu).flags = clear_flag_c((*cpu).flags);
    }

    if (ram::get_memory(cpu, (*cpu).register_x) == 0){
        (*cpu).flags = set_flag_z((*cpu).flags);
    } else{
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn dec_m_operation(cpu: *mut super::CPU, step: u16){
    let mut temp: u16 = ram::get_memory(cpu, step & 0xF) - 1;
    ram::set_memory(cpu, step & 0xF, temp & 0xF);

    if (temp >> 4) > 0{
        (*cpu).flags = set_flag_c((*cpu).flags);
    } else{
        (*cpu).flags = clear_flag_c((*cpu).flags);
    }

    if (ram::get_memory(cpu, (*cpu).register_x) == 0){
        (*cpu).flags = set_flag_z((*cpu).flags);
    } else{
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}
unsafe fn acpx_operation(cpu: *mut super::CPU, step: u16){
    let mut temp: u16 = ram::get_memory(cpu, (*cpu).register_x) + get_rq(cpu, step & 0x3) + ((*cpu).flags & FLAG_C);

    if ((*cpu).flags & FLAG_D) > 0{
        if (temp >= 10){
            ram::set_memory(cpu,(*cpu).register_x ,(temp - 10) & 0xF);
            (*cpu).flags = set_flag_c((*cpu).flags);
        }else{
            ram::set_memory(cpu, (*cpu).register_x, temp);
            (*cpu).flags = clear_flag_c((*cpu).flags);
        }
    } else{
        ram::set_memory(cpu, (*cpu).register_x, temp & 0xF);
        if (temp >> 4) > 0{
            (*cpu).flags = set_flag_c((*cpu).flags);
        } else{
            (*cpu).flags = clear_flag_c((*cpu).flags);
        }
    }

    if (ram::get_memory(cpu, (*cpu).register_x) == 0){
        (*cpu).flags = set_flag_z((*cpu).flags);
    } else{
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }

    (*cpu).register_x = (((*cpu).register_x + 1) & 0xFF) | ((((*cpu).register_x >> 8) & 0xF) << 8);
}
unsafe fn acpy_operation(cpu: *mut super::CPU, step: u16){
    let mut temp: u16 = ram::get_memory(cpu, (*cpu).register_y) + get_rq(cpu, step & 0x3) + ((*cpu).flags & FLAG_C);

    if ((*cpu).flags & FLAG_D) > 0{
        if (temp >= 10){
            ram::set_memory(cpu,(*cpu).register_y ,(temp - 10) & 0xF);
            (*cpu).flags = set_flag_c((*cpu).flags);
        }else{
            ram::set_memory(cpu, (*cpu).register_y, temp);
            (*cpu).flags = clear_flag_c((*cpu).flags);
        }
    } else{
        ram::set_memory(cpu, (*cpu).register_y, temp & 0xF);
        if (temp >> 4) > 0{
            (*cpu).flags = set_flag_c((*cpu).flags);
        } else{
            (*cpu).flags = clear_flag_c((*cpu).flags);
        }
    }

    if (ram::get_memory(cpu, (*cpu).register_y) == 0){
        (*cpu).flags = set_flag_z((*cpu).flags);
    } else{
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }

    (*cpu).register_y = (((*cpu).register_y + 1) & 0xFF) | ((((*cpu).register_y >> 8) & 0xF) << 8);
}
unsafe fn scpx_operation(cpu: *mut super::CPU, step: u16){
    let mut temp: u16 = ram::get_memory(cpu, (*cpu).register_x) - get_rq(cpu, step & 0x3) - ((*cpu).flags & FLAG_C);

    if ((*cpu).flags & FLAG_D) > 0{
        if (temp >> 4) > 0{
            ram::set_memory(cpu,(*cpu).register_x ,(temp - 6) & 0xF);
        }else{
            ram::set_memory(cpu, (*cpu).register_x, temp);
        }
    } else{
        ram::set_memory(cpu, (*cpu).register_x, temp & 0xF);
    }

    if (temp >> 4) > 0{
        (*cpu).flags = set_flag_c((*cpu).flags);
    } else{
        (*cpu).flags = clear_flag_c((*cpu).flags);
    }

    if (ram::get_memory(cpu, (*cpu).register_x) == 0){
        (*cpu).flags = set_flag_z((*cpu).flags);
    } else{
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }

    (*cpu).register_x = (((*cpu).register_x + 1) & 0xFF) | ((((*cpu).register_x >> 8) & 0xF) << 8);
}
unsafe fn scpy_operation(cpu: *mut super::CPU, step: u16){
    let mut temp: u16 = ram::get_memory(cpu, (*cpu).register_y) - get_rq(cpu, step & 0x3) - ((*cpu).flags & FLAG_C);

    if ((*cpu).flags & FLAG_D) > 0{
        if (temp >> 4) > 0{
            ram::set_memory(cpu,(*cpu).register_y ,(temp - 6) & 0xF);
        }else{
            ram::set_memory(cpu, (*cpu).register_y, temp);
        }
    } else{
        ram::set_memory(cpu, (*cpu).register_y, temp & 0xF);
    }

    if (temp >> 4) > 0{
        (*cpu).flags = set_flag_c((*cpu).flags);
    } else{
        (*cpu).flags = clear_flag_c((*cpu).flags);
    }

    if (ram::get_memory(cpu, (*cpu).register_y) == 0){
        (*cpu).flags = set_flag_z((*cpu).flags);
    } else{
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }

    (*cpu).register_y = (((*cpu).register_y + 1) & 0xFF) | ((((*cpu).register_y >> 8) & 0xF) << 8);
}
unsafe fn not_operation(cpu: *mut super::CPU, step: u16){
    set_rq(cpu, (step >> 4) & 0x3, !((step >> 4) & 0x3) & 0xF);

    if get_rq(cpu, (step >> 4) & 0x3) == 0{
        (*cpu).flags = set_flag_z((*cpu).flags);
    } else{
        (*cpu).flags = clear_flag_z((*cpu).flags);
    }
}

pub const ISA : [Opcode; 108] = [
    Opcode{
        name: "NOP5",
        code: 0xFFB,
        cycles: 5,
        operation: nop5_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "NOP7",
        code: 0xFFF,
        cycles: 7,
        operation: nop7_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "HALT",
        code: 0xFF8,
        cycles: 5,
        operation: halt_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "PSET",
        code: 0xE40,
        cycles: 5,
        operation: pset_operation,
        mask: MASK_7B,
    },
    Opcode{
        name: "JP_S",
        code: 0x000,
        cycles: 5,
        operation: jp_s_operation,
        mask: MASK_4B,
    },
    Opcode{
        name: "JP_CS",
        code: 0x200,
        cycles: 5,
        operation: jp_cs_operation,
        mask: MASK_4B,
    },
    Opcode{
        name: "JP_NCS",
        code: 0x300,
        cycles: 5,
        operation: jp_ncs_operation,
        mask: MASK_4B,
    },
    Opcode{
        name: "JP_ZS",
        code: 0x600,
        cycles: 5,
        operation: jp_zs_operation,
        mask: MASK_4B,
    },
    Opcode{
        name: "JP_NZS",
        code: 0x700,
        cycles: 5,
        operation: jp_nzs_operation,
        mask: MASK_4B,
    },
    Opcode{
        name: "JPBA",
        code: 0xFE8,
        cycles: 5,
        operation: jpba_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "CALL",
        code: 0x400,
        cycles: 7,
        operation: call_operation,
        mask: MASK_4B,
    },
    Opcode{
        name: "CALZ",
        code: 0x500,
        cycles: 7,
        operation: callz_operation,
        mask: MASK_4B,
    },
    Opcode{
        name: "RET",
        code: 0xFDF,
        cycles: 7,
        operation: ret_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "RETS",
        code: 0xFDE,
        cycles: 12,
        operation: ret_s_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "RETD",
        code: 0x100,
        cycles: 12,
        operation: ret_d_operation,
        mask: MASK_4B,
    },
    Opcode{
        name: "INC_X",
        code: 0xEE0,
        cycles: 5,
        operation: inc_x_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "INC_Y",
        code: 0xEF0,
        cycles: 5,
        operation: inc_y_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "LD_X",
        code: 0xB00,
        cycles: 5,
        operation: ld_x_operation,
        mask: MASK_4B,
    },
    Opcode{
        name: "LD_Y",
        code: 0x800,
        cycles: 5,
        operation: ld_y_operation,
        mask: MASK_4B,
    },
    Opcode{
        name: "LD_XPR",
        code: 0xE80,
        cycles: 5,
        operation: ld_xpr_operation,
        mask: MASK_10B,
    },
    Opcode{
        name: "LD_XHR",
        code: 0xE84,
        cycles: 5,
        operation: ld_xhr_operation,
        mask: MASK_10B,
    },
    Opcode{
        name: "LD_XLR",
        code: 0xE88,
        cycles: 5,
        operation: ld_xlr_operation,
        mask: MASK_10B,
    },
    Opcode{
        name: "LD_YPR",
        code: 0xE90,
        cycles: 5,
        operation: ld_ypr_operation,
        mask: MASK_10B,
    },
    Opcode{
        name: "LD_YHR",
        code: 0xE94,
        cycles: 5,
        operation: ld_yhr_operation,
        mask: MASK_10B,
    },
    Opcode{
        name: "LD_YLR",
        code: 0xE98,
        cycles: 5,
        operation: ld_ylr_operation,
        mask: MASK_10B,
    },
    Opcode{
        name: "LD_RXP",
        code: 0xEA0,
        cycles: 5,
        operation: ld_rxp_operation,
        mask: MASK_10B,
    },
    Opcode{
        name: "LD_RXH",
        code: 0xEA4,
        cycles: 5,
        operation: ld_rxh_operation,
        mask: MASK_10B,
    },
    Opcode{
        name: "LD_RXL",
        code: 0xEA8,
        cycles: 5,
        operation: ld_rxl_operation,
        mask: MASK_10B,
    },
    Opcode{
        name: "LD_RYP",
        code: 0xEB0,
        cycles: 5,
        operation: ld_ryp_operation,
        mask: MASK_10B,
    },
    Opcode{
        name: "LD_RYH",
        code: 0xEB4,
        cycles: 5,
        operation: ld_ryh_operation,
        mask: MASK_10B,
    },
    Opcode{
        name: "LD_YL",
        code: 0xEB8,
        cycles: 5,
        operation: ld_yl_operation,
        mask: MASK_10B,
    },
    Opcode{
        name: "ADC_XH",
        code: 0xA00,
        cycles: 7,
        operation: adc_xh_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "ADC_XL",
        code: 0xA10,
        cycles: 7,
        operation: adc_xl_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "ADC_YH",
        code: 0xA20,
        cycles: 7,
        operation: adc_yh_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "ADC_YL",
        code: 0xA30,
        cycles: 7,
        operation: adc_yl_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "CP_XH",
        code: 0xA40,
        cycles: 7,
        operation: cp_xh_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "CP_XL",
        code: 0xA50,
        cycles: 7,
        operation: cp_xl_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "CP_YH",
        code: 0xA60,
        cycles: 7,
        operation: cp_yh_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "CP_YL",
        code: 0xA70,
        cycles: 7,
        operation: cp_yl_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "LD_R",
        code: 0xE00,
        cycles: 5,
        operation: ld_r_operation,
        mask: MASK_6B,
    },
    Opcode{
        name: "LD_RQ",
        code: 0xEC0,
        cycles: 5,
        operation: ld_rq_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "LD_AM",
        code: 0xFA0,
        cycles: 5,
        operation: ld_am_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "LD_BM",
        code: 0xFB0,
        cycles: 5,
        operation: ld_bm_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "LD_MA",
        code: 0xF80,
        cycles: 5,
        operation: ld_ma_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "LD_MB",
        code: 0xF90,
        cycles: 5,
        operation: ld_mb_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "LDPX_MX",
        code: 0xE60,
        cycles: 5,
        operation:ldpx_mx_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "LDPX_RQ",
        code: 0xEE0,
        cycles: 5,
        operation: ldpx_rq_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "LDPY_MY",
        code: 0xE70,
        cycles: 5,
        operation: ldpy_my_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "LDPY_RQ",
        code: 0xEF0,
        cycles: 5,
        operation: ldpy_rq_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "LBPX",
        code: 0x900,
        cycles: 5,
        operation: lbpx_operation,
        mask: MASK_4B,
    },
    Opcode{
        name: "SET",
        code: 0xF40,
        cycles: 7,
        operation: set_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "RST",
        code: 0xF50,
        cycles: 7,
        operation: rst_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "SCF",
        code: 0xF41,
        cycles: 7,
        operation: scf_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "RCF",
        code: 0xF5E,
        cycles: 7,
        operation: rcf_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "SZF",
        code: 0xF42,
        cycles: 7,
        operation: szf_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "RZF",
        code: 0xF5D,
        cycles: 7,
        operation: rzf_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "SDF",
        code: 0xF44,
        cycles: 7,
        operation: sdf_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "RDF",
        code: 0xF5B,
        cycles: 7,
        operation: rdf_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "EI",
        code: 0xF48,
        cycles: 7,
        operation: ei_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "DI",
        code: 0xF57,
        cycles: 7,
        operation: di_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "INC",
        code: 0xFDB,
        cycles: 5,
        operation: inc_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "DEC",
        code: 0xFCB,
        cycles: 5,
        operation: dec_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "PUSH_R",
        code: 0xFC0,
        cycles: 5,
        operation: push_r_operation,
        mask: MASK_10B,
    },
    Opcode{
        name: "PUSH_XP",
        code: 0xFC4,
        cycles: 5,
        operation: push_xp_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "PUSH_XH",
        code: 0xFC5,
        cycles: 5,
        operation: push_xh_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "PUSH_XL",
        code: 0xFC6,
        cycles: 5,
        operation: push_xl_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "PUSH_YP",
        code: 0xFC7,
        cycles: 5,
        operation: push_yp_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "PUSH_YH",
        code: 0xFC8,
        cycles: 5,
        operation: push_yh_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "PUSH_YL",
        code: 0xFC9,
        cycles: 5,
        operation: push_yl_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "PUSH_F",
        code: 0xFCA,
        cycles: 5,
        operation: push_f_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "POP_R",
        code: 0xFD0,
        cycles: 5,
        operation: pop_r_operation,
        mask: MASK_10B,
    },
    Opcode{
        name: "POP_XP",
        code: 0xFD4,
        cycles: 5,
        operation: pop_xp_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "POP_XH",
        code: 0xFD5,
        cycles: 5,
        operation: pop_xh_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "POP_XL",
        code: 0xFD6,
        cycles: 5,
        operation: pop_xl_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "POP_YP",
        code: 0xFD7,
        cycles: 5,
        operation: pop_yp_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "POP_YH",
        code: 0xFD8,
        cycles: 5,
        operation: pop_yh_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "POP_YL",
        code: 0xFD9,
        cycles: 5,
        operation: pop_yl_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "POP_F",
        code: 0xFDA,
        cycles: 5,
        operation: pop_f_operation,
        mask: MASK_12B,
    },
    Opcode{
        name: "LD_SPH",
        code: 0xFE0,
        cycles: 5,
        operation: ld_sph_operation,
        mask: MASK_10B,
    },
    Opcode{
        name: "LD_SPL",
        code: 0xFF0,
        cycles: 5,
        operation: ld_spl_operation,
        mask: MASK_10B,
    },
    Opcode{
        name: "LD_RSPH",
        code: 0xFE4,
        cycles: 5,
        operation: ld_rsph_operation,
        mask: MASK_10B,
    },
    Opcode{
        name: "LD_RSPL",
        code: 0xFF4,
        cycles: 5,
        operation: ld_rspl_operation,
        mask: MASK_10B,
    },
    Opcode{
        name: "ADD_R",
        code: 0xC00,
        cycles: 7,
        operation: add_r_operation,
        mask: MASK_6B,
    },
    Opcode{
        name: "ADD_RQ",
        code: 0xA80,
        cycles: 7,
        operation: add_rq_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "ADC_R",
        code: 0xC40,
        cycles: 7,
        operation: adc_r_operation,
        mask: MASK_6B,
    },
    Opcode{
        name: "ADC_RQ",
        code: 0xA90,
        cycles: 7,
        operation: adc_rq_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "SUB",
        code: 0xAA0,
        cycles: 7,
        operation: sub_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "SBC_R",
        code: 0xB40,
        cycles: 7,
        operation: sbc_r_operation,
        mask: MASK_6B,
    },
    Opcode{
        name: "SBC_RQ",
        code: 0xAB0,
        cycles: 7,
        operation: sbc_rq_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "AND_R",
        code: 0xC80,
        cycles: 7,
        operation: and_r_operation,
        mask: MASK_6B,
    },
    Opcode{
        name: "AND_RQ",
        code: 0xAC0,
        cycles: 7,
        operation: and_rq_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "OR_R",
        code: 0xCC0,
        cycles: 7,
        operation: or_r_operation,
        mask: MASK_6B,
    },Opcode{
        name: "OR_RQ",
        code: 0xAD0,
        cycles: 7,
        operation: or_rq_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "XOR_R",
        code: 0xD00,
        cycles: 7,
        operation: xor_r_operation,
        mask: MASK_6B,
    },
    Opcode{
        name: "XOR_RQ",
        code: 0xAE0,
        cycles: 7,
        operation: xor_rq_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "CP_R",
        code: 0xDC0,
        cycles: 7,
        operation: cp_r_operation,
        mask: MASK_6B,
    },
    Opcode{
        name: "CP_RQ",
        code: 0xF00,
        cycles: 7,
        operation: cp_rq_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "FAN_R",
        code: 0xD80,
        cycles: 7,
        operation: fan_r_operation,
        mask: MASK_6B,
    },
    Opcode{
        name: "FAN_RQ",
        code: 0xF10,
        cycles: 7,
        operation: fan_rq_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "RLC",
        code: 0xAF0,
        cycles: 7,
        operation: rlc_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "RRC",
        code: 0xE8C,
        cycles: 5,
        operation: rrc_operation,
        mask: MASK_10B,
    },
    Opcode{
        name: "INC_M",
        code: 0xF60,
        cycles: 7,
        operation: inc_m_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "DEC_M",
        code: 0xF70,
        cycles: 7,
        operation: dec_m_operation,
        mask: MASK_8B,
    },
    Opcode{
        name: "ACPX",
        code: 0xF28,
        cycles: 7,
        operation: acpx_operation,
        mask: MASK_10B,
    },
    Opcode{
        name: "ACPY",
        code: 0xF2C,
        cycles: 7,
        operation: acpy_operation,
        mask: MASK_10B,
    },
    Opcode{
        name: "SCPX",
        code: 0xF38,
        cycles: 7,
        operation: scpx_operation,
        mask: MASK_10B,
    },
    Opcode{
        name: "SCPY",
        code: 0xF3C,
        cycles: 7,
        operation: scpy_operation,
        mask: MASK_10B,
    },
    Opcode{
        name: "NOT",
        code: 0xD0F,
        cycles: 7,
        operation: not_operation,
        mask: 0xFCF,
    },
];