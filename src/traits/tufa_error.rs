use crate::helpers::where_was::WhereWas;

pub trait TufaError {
    fn get_source(&self) -> String;
    fn get_where_was(&self) -> String; //Vec<WhereWasOneOrFew>
}
