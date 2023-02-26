use tufa_common::traits::error_logs_logic::error_log::ErrorLog;

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

pub fn one<'a>() -> Result<(), Box<tufa_common::repositories_types::one::OneWrapperError<'a>>> {
    if let Err(e) = tufa_common::dev::three() {
        return Err(Box::new(
            tufa_common::repositories_types::one::OneWrapperError::Something {
                inner_error: *e,
                code_occurence: tufa_common::code_occurence!(),
            },
        ));
    }
    Ok(())
}

pub fn one_with_deserialize<'a>(
) -> Result<(), Box<tufa_common::repositories_types::one::OneWrapperErrorWithDeserialize<'a>>> {
    if let Err(e) = tufa_common::dev::three_with_deserialize() {
        return Err(Box::new(
            tufa_common::repositories_types::one::OneWrapperErrorWithDeserialize::Something {
                inner_error: *e,
                code_occurence:
                    tufa_common::common::code_occurence::CodeOccurenceWithDeserialize::new(
                        &tufa_common::global_variables::compile_time::git_info::GIT_INFO,
                        file!(),
                        line!(),
                        column!(),
                    ),
            },
        ));
    }
    Ok(())
}
