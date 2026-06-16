use uuid::Uuid;

/// Used in queries that just return an ID
pub(crate) struct Id {
    pub id: Uuid,
}

/// Used in queries which return whether a row exists
pub(crate) struct Exists {
    pub exists: bool,
}
