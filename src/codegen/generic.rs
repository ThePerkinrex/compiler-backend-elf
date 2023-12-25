use crate::{data::St, json::Statement};

pub trait Codegen<S: St> {
    fn enter_fn(&mut self, entry: S::StEntryId);
    fn exit_fn(&mut self);
    fn gen_statement(&mut self, statement: Statement);
}
