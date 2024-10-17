// NOTE: Components in the schema of each API are defined here.
// However, the top-level schema specific to each URI is not defined here (e.g. ~Request, ~Response)

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LineItemSchema {
    pub variant_id: Option<String>,
    pub quantity: u32,
}
