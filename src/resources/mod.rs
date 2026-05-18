pub mod auth;
pub mod clients;

// macro_rules! endpoint_no_response {
//     ($(#[$attr:meta])* $name:ident, $action:expr, $params:ty) => {
//         $(#[$attr])*
//         pub async fn $name(&self, params: impl Into<Option<$params>>) -> Result<(), WhmcsError> {
//             let p = params.into().unwrap_or_default();
//             self.request($action, p).await
//         }
//     };
// }

macro_rules! endpoint_no_params {
    ($(#[$attr:meta])* $name:ident, $action:expr, $resp:ty) => {
        $(#[$attr])*
        pub async fn $name(&self) -> Result<$resp, WhmcsError> {
            self.request($action, ()).await
        }
    };
}

macro_rules! endpoint {
    ($(#[$attr:meta])* $name:ident, $action:expr, $params:ty, $resp:ty) => {
        $(#[$attr])*
        pub async fn $name(&self, params: impl Into<Option<$params>>) -> Result<$resp, WhmcsError> {
            let p = params.into().unwrap_or_default();
            self.request($action, p).await
        }
    };
}

use {endpoint, endpoint_no_params};
