use serde::Serialize;

pub type Id = i64;

#[derive(Serialize, Clone, sqlx::FromRow)]
pub struct Asset {
    pub id: Id,
    pub name: String,
    pub unit_value: f64,
}
