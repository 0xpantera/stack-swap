use core::arch::asm;

const SSIZE: isize = 48;

#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    rsp: u64,
}

fn main() {
    let mut ctx = ThreadContext::default();
    let mut stack = vec![0_u8; SSIZE as usize];

    unsafe {
        // Mutable pointer to the end of the stack space
        let stack_bottom = stack.as_mut_ptr().offset(SSIZE);
        // Round memory address down to nearest 16-byte aligned address
        let sb_aligned = (stack_bottom as usize & !15) as *mut u8;
        // Write address of the function pointer 16 bytes from the aligned stack bottom
        // Sets up the return address for the thread's first function call
        std::ptr::write(sb_aligned.offset(-16) as *mut u64, hello as u64);
        // Update the stack pointer (`rsp`) in the `ctx` to point to this new location
        // Sets up the stack pointer for when the thread context is switched to
        ctx.rsp = sb_aligned.offset(-16) as u64;

        // Print stack
        for i in 0..SSIZE {
            println!(
                "mem: {}, val: {}",
                sb_aligned.offset(-i as isize) as usize,
                *sb_aligned.offset(-i as isize)
            )
        }

        // Perform the actual context switch
        gt_switch(&mut ctx);
    }
}

fn hello() -> ! {
    println!("WAKING UP ON A NEW STACK");
    loop {}
}

unsafe fn gt_switch(new: *const ThreadContext) {
    asm!(
        "mov rsp, [{0} + 0x00]",
        "ret",
        in(reg) new,
    );
}