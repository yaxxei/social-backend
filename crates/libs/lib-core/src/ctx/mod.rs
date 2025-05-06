pub mod error;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Ctx {
    pub user_id: Option<Uuid>,
}

impl Ctx {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id: Some(user_id),
        }
    }
}
