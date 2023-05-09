pub async fn change_password(
    form: actix_web::web::Form<tufa_common::common::change_password_form_data::ChangePasswordFormData>,
    pool: actix_web::web::Data<sqlx::PgPool>,
    user_id: actix_web::web::ReqData<tufa_common::repositories_types::tufa_server::authentication::UserId>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();
    if 
    {
        use secrecy::ExposeSecret;
        form.new_password.expose_secret()
    } 
    != 
    {
        use secrecy::ExposeSecret;
        form.new_password_check.expose_secret()
    } {
        actix_web_flash_messages::FlashMessage::error(
            "You entered two different new passwords - the field values must match.",
        )
        .send();
        return Ok(tufa_common::repositories_types::tufa_server::utils::status_codes::see_other("/admin/password"));
    }
    let username = crate::routes::admin::dashboard::get_username(*user_id, &pool).await.map_err(tufa_common::repositories_types::tufa_server::utils::status_codes::e500)?;
    let credentials = tufa_common::common::postgres_credentials::PostgresCredentials {
        username,
        password: form.0.current_password,
    };
    if let Err(e) = tufa_common::repositories_types::tufa_server::authentication::password::validate_credentials(credentials, &pool).await {
        return match e {
            tufa_common::repositories_types::tufa_server::authentication::password::AuthError::InvalidCredentials(_) => {
                actix_web_flash_messages::FlashMessage::error("The current password is incorrect.").send();
                Ok(tufa_common::repositories_types::tufa_server::utils::status_codes::see_other("/admin/password"))
            }
            tufa_common::repositories_types::tufa_server::authentication::password::AuthError::UnexpectedError(_) => Err(tufa_common::repositories_types::tufa_server::utils::status_codes::e500(e)),
        };
    }
    tufa_common::repositories_types::tufa_server::authentication::change_password(*user_id, form.0.new_password, &pool)
        .await
        .map_err(tufa_common::repositories_types::tufa_server::utils::status_codes::e500)?;
    actix_web_flash_messages::FlashMessage::error("Your password has been changed.").send();
    Ok(tufa_common::repositories_types::tufa_server::utils::status_codes::see_other("/admin/password"))
}
