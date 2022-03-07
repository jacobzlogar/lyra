#[derive(Queryable)]
pub struct Account {
    pub id: i32,
    pub name: Option<String>,
}

#[derive(Queryable)]
pub struct Block {
    pub id: i32,
    pub chain_id: u16,
}

#[derive(Queryable)]
pub struct Transaction {
    pub id: i32,
}
