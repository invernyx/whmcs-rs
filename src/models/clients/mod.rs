use serde::{Deserialize, Serialize};

use crate::models::{mod_export, u32_id};

u32_id!(ClientGroupId);
u32_id!(ClientId);

mod_export!(add_client);
mod_export!(get_client);
mod_export!(get_client_details);
mod_export!(get_client_groups);
mod_export!(get_client_password);

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
/// Represents the lifecycle status of a client.
pub enum ClientStatus {
    /// The client is active.
    Active,
    /// The client is inactive.
    Inactive,
    /// The client is closed.
    Closed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_id_roundtrip_json() {
        let id = ClientId::new(99);
        assert_eq!(serde_json::to_string(&id).unwrap(), "99");
        let back: ClientId = serde_json::from_str("99").unwrap();
        assert_eq!(back, id);
    }

    #[test]
    fn client_group_id_roundtrip_json() {
        let id = ClientGroupId::new(3);
        assert_eq!(serde_json::to_string(&id).unwrap(), "3");
        let back: ClientGroupId = serde_json::from_str("3").unwrap();
        assert_eq!(back, id);
    }

    #[test]
    fn client_status_roundtrips_json() {
        for status in [
            ClientStatus::Active,
            ClientStatus::Inactive,
            ClientStatus::Closed,
        ] {
            let s = serde_json::to_string(&status).unwrap();
            let back: ClientStatus = serde_json::from_str(&s).unwrap();
            assert_eq!(back, status);
        }
    }
}
