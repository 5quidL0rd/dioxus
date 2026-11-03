use crate::{BuildId, BuilderUpdate, BundleFormat, Error, TraceMsg};
use axum::extract::ws::Message as WsMessage;
use std::path::PathBuf;

/// One fat enum to rule them all....
///
/// Thanks to libraries like winit for the inspiration
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RebuildTrigger {
    Manual,
    Signal,
}

pub(crate) enum ServeUpdate {
    NewConnection {
        id: BuildId,
        aslr_reference: Option<u64>,
        pid: Option<u32>,
    },
    WsMessage {
        bundle: BundleFormat,
        msg: WsMessage,
    },

    /// An update regarding the state of the build and running app from an AppBuilder
    BuilderUpdate {
        id: BuildId,
        update: BuilderUpdate,
    },

    FilesChanged {
        files: Vec<PathBuf>,
    },

    OpenApp,
    RequestRebuild {
        trigger: RebuildTrigger,
    },

    CycleHotreloadMode,

    OpenDebugger {
        id: BuildId,
    },

    Redraw,

    TracingLog {
        log: TraceMsg,
    },

    Exit {
        error: Option<Error>,
    },
}
