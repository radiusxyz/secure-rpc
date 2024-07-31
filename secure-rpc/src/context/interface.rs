use std::{mem::MaybeUninit, sync::Once};

use crate::context::Context;

static mut CONTEXT: MaybeUninit<Context> = MaybeUninit::uninit();
static INIT: Once = Once::new();

pub fn context() -> &'static Context {
    if INIT.is_completed() {
        unsafe { CONTEXT.assume_init_ref() }
    } else {
        panic!("Runtime has not been initialized");
    }
}

impl Context {
    pub fn init(context: Context) {
        unsafe {
            INIT.call_once(|| {
                CONTEXT.write(context);
            });
        }
    }
}
