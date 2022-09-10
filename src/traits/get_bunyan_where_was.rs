use crate::helpers::where_was::WhereWasOneOrMany;
use crate::traits::get_where_was_one_or_many::GetWhereWasOneOrMany;

pub trait GetBunyanWhereWas {
    fn get_bunyan_format(&self) -> String;
}

impl<T> GetBunyanWhereWas for T
where
    T: GetWhereWasOneOrMany,
{
    fn get_bunyan_format(&self) -> String {
        match self.get_where_was_one_or_many() {
            WhereWasOneOrMany::One(where_was) => where_was.file_line_column(),
            WhereWasOneOrMany::Many(vec_where_was_with_addition) => {
                let mut formatted_into_string_vec = vec_where_was_with_addition
                    .iter()
                    .map(|where_was_with_addition| format!("{}, ", where_was_with_addition)) //maybe here use \n
                    .collect::<Vec<String>>()
                    .iter()
                    .fold(String::from(""), |mut acc, elem| {
                        acc.push_str(elem);
                        acc
                    });
                if !formatted_into_string_vec.is_empty() {
                    formatted_into_string_vec.pop();
                    formatted_into_string_vec.pop();
                }
                format!("[{}]", formatted_into_string_vec)
            }
        }
    }
}
