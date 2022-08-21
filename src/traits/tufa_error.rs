use crate::helpers::where_was::WhereWas;
use crate::helpers::where_was::WhereWasOneOrFew;

pub trait TufaError {
    fn get_source(&self) -> String;
    fn get_where_was(&self) -> String; //Vec<WhereWasOneOrFew>
}
