#![no_std]

use catalogs::RmrkCatalog;
use sails_macros::{gprogram, groute};
use sails_rtl::gstd::{
    calls::{Args, Remoting},
    events::GStdEventTrigger,
    GStdExecContext,
};
use services::{ResourceStorage, ResourceStorageEvent};

mod catalogs;
// Exposed publicly because of tests which use generated data
// while there is no generated client
pub mod services;

type ResourceStorageEventTrigger = GStdEventTrigger<ResourceStorageEvent>;

#[derive(Default)]
pub struct Program;

#[gprogram]
impl Program {
    // Initialize program and seed hosted services
    pub fn new() -> Self {
        let exec_context = GStdExecContext::default();
        ResourceStorage::<_, RmrkCatalog<Remoting, Args>, ResourceStorageEventTrigger>::seed(
            exec_context,
        );
        Self
    }

    // Expose hosted service
    #[groute("")]
    pub fn resource_storage(
        &self,
    ) -> ResourceStorage<GStdExecContext, RmrkCatalog<Remoting, Args>, ResourceStorageEventTrigger>
    {
        ResourceStorage::new(
            GStdExecContext::default(),
            RmrkCatalog::new(Remoting),
            GStdEventTrigger::new(),
        )
    }

    // Will be generated by the `ginject_defaults` macro from the `new` method
    fn __new(exec_context: GStdExecContext) -> Self {
        ResourceStorage::<_, RmrkCatalog<Remoting, Args>, ResourceStorageEventTrigger>::seed(
            exec_context,
        );
        Self
    }

    // Will be generated by the `ginject_defaults` macro from the `resource_storage` method
    fn __resource_storage(
        &self,
        exec_context: GStdExecContext,
        remoting: Remoting,
        event_trigger: ResourceStorageEventTrigger,
    ) -> ResourceStorage<GStdExecContext, RmrkCatalog<Remoting, Args>, ResourceStorageEventTrigger>
    {
        ResourceStorage::new(exec_context, RmrkCatalog::new(remoting), event_trigger)
    }
}
