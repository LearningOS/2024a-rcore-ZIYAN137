//! Memory management implementation
//!
//! SV39 page-based virtual-memory architecture for RV64 systems, and
//! everything about memory management, like frame allocator, page table,
//! map area and memory set, is implemented here.
//!
//! Every task or process has a memory_set to control its virtual memory.

pub mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod page_table;

pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
use address::{StepByOne, VPNRange};
pub use frame_allocator::{frame_alloc, FrameTracker};
pub use memory_set::remap_test;
pub use memory_set::{kernel_stack_position, MapPermission, MemorySet, KERNEL_SPACE};
pub use page_table::{translated_byte_buffer, PageTableEntry};
use page_table::{PTEFlags, PageTable};
use crate::task::{current_user_token};

/// initiate heap allocator, frame allocator and kernel space
pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}

/// translate virtual address to physical address
pub fn v2p(virt_addr: VirtAddr) -> Option<PhysAddr> {
    let offset = virt_addr.page_offset();
    let vpn = virt_addr.floor();
    let ppn = PageTable::from_token(current_user_token()).translate(vpn).map(|pte| pte.ppn());
    if let Some(ppn) = ppn {
        Some(PhysAddr(usize::from(PhysAddr::from(ppn)) + offset))
    } else {
        println!("v2p failed: {:x?}", virt_addr);
        None
    }
}