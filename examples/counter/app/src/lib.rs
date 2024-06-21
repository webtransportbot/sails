#![no_std]

use core::cell::RefCell;

use sails_rtl::gstd::gprogram;

pub mod dec;
pub mod inc;
pub mod query;

#[derive(Default)]
pub struct Program {
    // Shared program state, actually stored as `static mut`
    value: RefCell<u64>,
}

#[gprogram]
impl Program {
    pub const fn with_initial_value(init_value: u64) -> Self {
        Self {
            value: RefCell::new(init_value),
        }
    }

    pub fn inc(&self) -> inc::IncSvc {
        inc::IncSvc::new(&self.value)
    }

    pub fn dec(&self) -> dec::DecSvc {
        dec::DecSvc::new(&self.value)
    }

    pub fn query(&self) -> query::QuerySvc {
        query::QuerySvc::new(&self.value)
    }
}
