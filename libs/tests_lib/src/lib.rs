#[cfg(test)]
mod tests {
    pub mod continuous_integration {
        // pub mod ci_check_compromised_github_auth_info;//todo rewrite it into .env file
        // pub mod ci_check_compromised_reddit_auth_info;//todo rewrite it into .env file
        pub mod ci_check_config_files_exists;
        pub mod ci_check_new_config_fields;
        // pub mod ci_check_new_user_credentials_fields;//todo rewrite it into .env file
    }
    mod tests_constants;
}
