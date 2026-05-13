use serde::{Deserialize, Serialize};

use crate::models::u32_id;

u32_id!(UserId);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_id_roundtrip_json() {
        let id = UserId::new(42);
        let v = serde_json::to_string(&id).unwrap();
        assert_eq!(v, "42");
        let back: UserId = serde_json::from_str(&v).unwrap();
        assert_eq!(back, id);
    }
}
