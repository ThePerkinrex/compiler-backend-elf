use crate::data::{St, StEntry};

pub type JsonSt = Vec<Vec<StEntry>>;

#[derive(Debug, serde::Deserialize, Clone, Copy)]
pub struct StEntryRef {
    pub st_idx: usize,
    pub idx: usize,
}

impl St for JsonSt {
    type StEntryId = StEntryRef;

    fn get(&self, idx: Self::StEntryId) -> &StEntry {
        &self[idx.st_idx][idx.idx]
    }
}

pub type Code = Vec<Item>;

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Item {
    Function { entry: StEntryRef, body: Body },
}

pub type Body = Vec<Statement>;

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Statement {
    Syscall {
        syscall: Expression,
        args: Vec<Expression>,
    },
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Expression {
    IntConst { val: u64 },
    StrConst { val: String },
}
