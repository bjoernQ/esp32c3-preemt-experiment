#![no_std]
#![no_main]
#![feature(asm_const)]

use embedded_hal::prelude::_embedded_hal_timer_CountDown;
use panic_halt as _;

use core::arch::asm;
use core::fmt::Write;
use riscv_rt::entry;

use esp32c3_pac as pac;

mod timer;
use timer::*;

mod uart;
use uart::*;

mod wdts;
use wdts::*;

mod preempt;
use preempt::*;

extern "C" {
    fn _start_trap_full();

    static _vector_table: *const u32;
}

#[entry]
fn main() -> ! {
    // disable interrupts, `csrwi        mie,0` throws an exception on the esp32c3
    unsafe {
        let mut _tmp: u32;
        asm!("csrrsi {0}, mstatus, {1}", out(reg) _tmp, const 0x00000008)
    };

    unsafe {
        riscv::interrupt::disable();
    }

    // disable wdt's
    disable_wdts();

    writeln!(Uart, "Hello world!").unwrap();

    let mut delay = EtsTimer::new(1_000_000);

    unsafe {
        let vec_table = &_vector_table as *const _ as usize;
        riscv::register::mtvec::write(vec_table, riscv::register::mtvec::TrapMode::Vectored);
    };

    let peripherals = pac::Peripherals::take().unwrap();

    // set systimer to 0
    peripherals
        .SYSTIMER
        .unit0_load_lo
        .write(|w| unsafe { w.bits(0) });
    peripherals
        .SYSTIMER
        .unit0_load_hi
        .write(|w| unsafe { w.bits(0) });
    peripherals
        .SYSTIMER
        .unit0_load
        .write(|w| unsafe { w.bits(1) });

    // PERIOD_MODE + PERIOD
    peripherals
        .SYSTIMER
        .target0_conf
        .write(|w| unsafe { w.bits((1 << 30) | 20_000) });
    // LOAD CONF VALUE
    peripherals
        .SYSTIMER
        .comp0_load
        .write(|w| unsafe { w.bits(1) });
    // set SYSTIMER_TARGET0_WORK_EN + UNIT0_WORK_EN
    peripherals
        .SYSTIMER
        .conf
        .write(|w| unsafe { w.bits(1 << 24 | 1 << 30) });

    peripherals
        .SYSTIMER
        .int_clr
        .write(|w| unsafe { w.bits(1 << 0) });

    // TARGET0 INT ENA
    peripherals
        .SYSTIMER
        .int_ena
        .write(|w| unsafe { w.bits(1 << 0) });

    peripherals
        .INTERRUPT_CORE0
        .systimer_target0_int_map
        .write(|w| unsafe { w.bits(10) });
    peripherals
        .INTERRUPT_CORE0
        .cpu_int_pri_10
        .write(|w| unsafe { w.bits(1) }); // PRIO = 1
    peripherals
        .INTERRUPT_CORE0
        .cpu_int_enable
        .write(|w| unsafe { w.bits(1 << 10) }); // ENABLE INT 10

    // create tasks before starting the task switching
    init_tasks();

    writeln!(Uart, "Starting ...").unwrap();

    unsafe {
        riscv::interrupt::enable();
    }

    loop {
        riscv::interrupt::free(|_| writeln!(Uart, "Hello from main task!").unwrap());
        nb::block!(delay.wait()).unwrap();
    }
}

fn init_tasks() {
    task_create(worker_task1);
    task_create(worker_task2);

    // needed when flashing with espflash until the direct-boot problems
    // are solved there, see https://github.com/esp-rs/espflash/pull/118
    unsafe {
        FIRST_SWITCH = true;
    }
}

pub extern "C" fn worker_task1() {
    loop {
        riscv::interrupt::free(|_| writeln!(Uart, "Hello from task 1!").unwrap());
        for _ in 0..100_000 {}
    }
}

pub extern "C" fn worker_task2() {
    loop {
        riscv::interrupt::free(|_| writeln!(Uart, "Hello from task 2!").unwrap());
        for _ in 0..150_000 {}
    }
}

#[link_section = ".trap.rust"]
#[export_name = "_start_trap_rust_full"]
pub unsafe extern "C" fn start_trap_rust_full(trap_frame: &mut FullTrapFrame) {
    // clear the systimer intr
    (*pac::SYSTIMER::ptr())
        .int_clr
        .write(|w| unsafe { w.bits(1 << 0) });

    task_switch(trap_frame);
}

/// Registers saved in trap handler
#[doc(hidden)]
#[allow(missing_docs)]
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct FullTrapFrame {
    pub ra: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    pub s0: usize,
    pub s1: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub gp: usize,
    pub tp: usize,
    pub sp: usize,
}
