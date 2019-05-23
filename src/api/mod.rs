//! API helpers to make use of k8s-openapi easier

/// Empty struct for Void
#[derive(Clone, Deserialize)]
pub struct Discard {}
/// Shortcut type for discarding one type parameter option
pub type Void = Option<Discard>;

mod reflector;
pub use self::reflector::{
    ResourceMap,
    Reflector,
};

mod informer;
pub use self::informer::{
    Informer,
};

mod api;
pub use api::{
    Api,
    GetParams,
};

mod resource;
pub use self::resource::{
    Resource,
    WatchEvent,
    ApiError,
    PostResponse,
};

mod metadata;
pub use self::metadata::{
    Metadata,
    Initializers,
};