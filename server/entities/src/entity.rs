use chrono::{DateTime, Utc};
use uuid::Uuid;

trait Entity {
    fn id(&self) -> &Uuid;
    fn created_at(&self) -> &DateTime<Utc>;
    fn updated_at(&self) -> &DateTime<Utc>;
}
