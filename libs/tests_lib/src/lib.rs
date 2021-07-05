#[cfg(test)]
mod tests {
    pub mod continuous_integration {
        pub mod ci_check_compromised_github_auth_info;
        pub mod ci_check_compromised_reddit_auth_info;
        pub mod ci_check_config_files_exists;
        pub mod ci_check_new_config_fields;
        pub mod ci_check_new_user_credentials_fields;
    }
    mod tests_constants;
}
