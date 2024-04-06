use core::ptr::addr_of;

use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable};
use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use lazy_static::lazy_static;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    // TSS used to hold information (registers' state, I/O port perms., stack pointers, prev. TSS
    // link) about task on x86 in order to allow for HW context switching, which is not supported
    // on x86_64, where TSS doesn't hold task-specific information at all and only holds two stack
    // tables and I/O port perms.
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { addr_of!(STACK) });
            let stack_end = stack_start + STACK_SIZE as u64;
            stack_end
        };
        tss
    };

    // GDT defines boundaries of memory areas (segments, e.g. stack, heap, code, OS-reserved, ...)
    // used by programs. Segmentation was superseded by paging.
    static ref GDT: GlobalDescriptorTable = {
        let mut gdt = GlobalDescriptorTable::new();
        gdt.append(Descriptor::kernel_code_segment());
        gdt.append(Descriptor::tss_segment(&TSS));
        gdt
    };
}

pub fn init() {
    GDT.load();
}
