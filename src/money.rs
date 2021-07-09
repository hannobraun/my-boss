use indexmap::IndexMap;
use time::Date;

pub struct Transaction {
    /// The date the transaction occurred
    pub date: Date,

    /// Description of the transaction
    pub description: String,

    /// The accounts the transaction affected
    pub accounts: IndexMap<String, i64>,

    /// The budgets the transaction affected
    pub budgets: IndexMap<String, i64>,
}
