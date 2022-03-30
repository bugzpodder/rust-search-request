use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Field<T> {
    pub eq: Option<T>,
    pub neq: Option<T>,
    pub gt: Option<T>,
    pub gte: Option<T>,
    pub lt: Option<T>,
    pub lte: Option<T>,
    pub is_null: Option<bool>,
    pub is_not_null: Option<bool>,
    pub is_in: Option<Vec<T>>,
    pub is_not_in: Option<Vec<T>>,
}

#[derive(Serialize, Deserialize)]
pub struct StrField {
    pub eq: Option<String>,
    pub neq: Option<String>,
    pub gt: Option<String>,
    pub gte: Option<String>,
    pub lt: Option<String>,
    pub lte: Option<String>,
    pub is_null: Option<bool>,
    pub is_not_null: Option<bool>,
    pub is_in: Option<Vec<String>>,
    pub is_not_in: Option<Vec<String>>,
    pub ilike: Option<String>,
    pub nilike: Option<String>,
    pub like: Option<String>,
    pub nlike: Option<String>,
    pub iregex: Option<String>,
    pub niregex: Option<String>,
    pub similar: Option<String>,
    pub nsimilar: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WalletType {
    Public,
    Private,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WalletOrderByField {
    WalletId,
    WalletType,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderByDir {
    Asc,
    Desc,
}

#[derive(Serialize, Deserialize)]
pub struct WalletOrderBy {
    pub field: Option<WalletOrderByField>,
    pub dir: Option<OrderByDir>,
}

#[derive(Serialize, Deserialize)]
pub struct WalletClause {
    pub and: Option<Box<WalletClause>>,
    pub or: Option<Box<WalletClause>>,
    pub wallet_id: Option<Field<u64>>,
    pub wallet_type: Option<Field<WalletType>>,
    pub wallet_name: Option<StrField>,
}

#[derive(Serialize, Deserialize)]
pub struct WalletRequest {
    pub where_clause: Option<WalletClause>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub order_by: Option<Vec<WalletOrderBy>>,
}
