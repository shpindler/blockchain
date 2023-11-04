use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug};
use std::clone::Clone;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFT {
    id: String, // A unique identifier for the NFT (e.g., a UUID or hash)
    owner: String, // The address of the current owner
    metadata: String, // Metadata could include a URI pointing to the NFT's data
}

impl NFT {
    pub fn new(id: String, owner: String, metadata: String) -> Self {
        NFT { id, owner, metadata }
    }

    // You might have a minting function that includes logic for creating a new NFT
    pub fn mint(owner: String, metadata: String) -> Self {
        let id = generate_unique_id(); // Ensure this function is implemented
        NFT::new(id, owner, metadata)
    }
}

// Implement or define the generate_unique_id function elsewhere in your code.
fn generate_unique_id() -> String {
    // Logic to generate a unique ID
    // For a simple placeholder, you can use a UUID library or create a simple counter-based ID.
    // But for production, ensure it's sufficiently unique.
    uuid::Uuid::new_v4().to_string() // Using the `uuid` crate
}
