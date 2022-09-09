use crate::helpers::where_was::WhereWasOneOrMany;
use crate::traits::get_where_was_one_or_many::GetWhereWasOneOrMany;

pub trait GetBunyanWhereWas {
    fn get_bunyan_format(&self) -> String;
}

impl<T> GetBunyanWhereWas for T
where
    Option<T>: GetWhereWasOneOrMany,
{
    fn get_bunyan_format(self) {
        match self.get_where_was_one_or_many() {
            WhereWasOneOrMany::One(where_was) => todo!(),
            WhereWasOneOrMany::Many(vec_where_was_with_addition) => todo!(),
        }
    }
}
