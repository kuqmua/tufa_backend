#[cfg(test)]
mod tests {
    pub mod continuous_integration {
        pub mod ci_check_compromised_github_auth_info;//todo rewrite it into .env file
        pub mod ci_check_compromised_reddit_auth_info;//todo rewrite it into .env file
        pub mod ci_check_env_file_exists;
        pub mod ci_check_new_env_vars;
    }
    mod tests_constants;
}
