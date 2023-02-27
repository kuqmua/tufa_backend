use tufa_common::traits::error_logs_logic::error_log::ErrorLog;

//todo - bug - if i wanna share impl for errors - i need to put it in tufa_common, but if i put it to tufa_common, i github links for repos would be not correct
//so i need to remove display impl for deserialize and use
pub fn dev() {
    let _f = one();
    if let Err(e) = _f {
        //todo - this actually must be a config struct function, not an error - config.error_log(e)
        println!("{}", *e);
        e.error_log(once_cell::sync::Lazy::force(
            &crate::global_variables::runtime::config::CONFIG,
        ));
    }
}

pub fn dev_with_deserialize() {
    let _f = one_with_deserialize();
    if let Err(e) = _f {
        println!("------++++-___");
        println!("{}", e);
    }
}

pub fn one<'a>() -> Result<(), Box<tufa_common::repositories_types::one::OneError<'a>>> {
    if let Err(e) = tufa_common::dev::three() {
        return Err(Box::new(
            tufa_common::repositories_types::one::OneError::Something {
                inner_error: *e,
                code_occurence: tufa_common::code_occurence!(),
            },
        ));
    }
    Ok(())
}

pub fn one_with_deserialize<'a>(
) -> Result<(), Box<tufa_common::repositories_types::one::OneErrorWithDeserialize<'a>>> {
    if let Err(e) = tufa_common::dev::three_with_deserialize() {
        return Err(Box::new(
            tufa_common::repositories_types::one::OneErrorWithDeserialize::Something {
                inner_error: *e,
                code_occurence: tufa_common::code_occurence_with_deserialize!(),
            },
        ));
    }
    Ok(())
}
