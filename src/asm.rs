pub fn load_eflags() -> i32 {
    let result: i32;
    unsafe {
        llvm_asm!("PUSHFD" : : : : "intel");
        llvm_asm!("POP EAX" : "={EAX}"(result) : : : "intel")
    }
    result
}

pub fn store_eflags(flags: i32) {
    unsafe {
        llvm_asm!("PUSH EAX" : : "EAX"(flags) : : "intel");
        asm!("POPFD");
    }
}

pub fn cli(){
    unsafe {
        llvm_asm!("CLI" : : : : "intel");
    }
}

pub fn out8(port: u32, data: u8) {
    unsafe {
        llvm_asm!("OUT DX,AL" : : "{EDX}"(port), "{AL}"(data) : : "intel");
    }
}
