use uuid::Uuid;

/// Used in queries that just return an ID
pub(crate) struct Id {
    pub id: Uuid,
}
