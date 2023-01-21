pub static HOSTNAME: once_cell::sync::Lazy<String> =
    once_cell::sync::Lazy::new(|| format!("{:?}", gethostname::gethostname()));
