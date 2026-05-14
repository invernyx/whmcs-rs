use crate::models::mod_export;

mod_export!(validate_login);

#[cfg(test)]
mod tests {
    #[test]
    fn auth_module_reexports_validate_login_types() {
        let _ = super::ValidateLoginParams::default();
    }
}
