use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "snake_case")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum StorageType {
    Local,
    S3,
    GCS,
}
