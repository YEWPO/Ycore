use riscv::register::sstatus::{self, Sstatus, SPP};

#[repr(C)]
#[derive(Debug)]
pub struct TrapContext {
    pub gpr: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
    pub kernel_satp: usize,
    pub trap_handler: usize,
    pub kernel_sp: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.gpr[2] = sp;
    }

    pub fn app_init_context(entry: usize, sp: usize, kernel_satp: usize, trap_handler: usize, kernel_sp: usize) -> Self {
        unsafe { sstatus::set_spp(SPP::User) };
        let sstatus = sstatus::read();

        let mut cx = Self {
            gpr: [0; 32],
            sstatus,
            sepc: entry,
            kernel_satp,
            trap_handler,
            kernel_sp,
        };
        cx.gpr[2] = sp;
        cx
    }
}
