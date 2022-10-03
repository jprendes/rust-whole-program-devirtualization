#![no_std]
#![no_main]

#[panic_handler]
fn panic_fmt(_: &core::panic::PanicInfo) -> ! { loop {} }

struct ProcessStandard<'a>(Option<&'a Kernel<'a>>);
struct Kernel<'a>(Option<&'a dyn Process>);

trait Process {
    fn print_full_process(&self);
}

impl Process for ProcessStandard<'_> {
    fn print_full_process(&self) {
        loop {}
    }
}

static mut KERNEL: Kernel = Kernel(None);
static mut PROCESS: ProcessStandard = ProcessStandard(None);

#[export_name = "_start"]
unsafe fn _start() {
    PROCESS.0 = Some(&KERNEL);
    KERNEL.0 = Some(&PROCESS);
}