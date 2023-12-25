pub trait St {
    type StEntryId: Clone + Copy;

    fn get(&self, idx: Self::StEntryId) -> &StEntry;
}

#[derive(Debug, serde::Deserialize)]
pub struct StEntry {
    pub lexeme: String,
    #[serde(flatten)]
    pub kind: StEntryKind,
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum StEntryKind {
    Function(FunctionStEntry),
}

#[derive(Debug, serde::Deserialize)]
pub struct FunctionStEntry {
    pub args: Vec<String>, // TODO Type
    pub ret: String,
    pub inner_st: usize,
}

impl StEntryKind {
    pub const fn unwrap_function(&self) -> Option<&FunctionStEntry> {
        match self {
            Self::Function(s) => Some(s),
            _ => None,
        }
    }
}
