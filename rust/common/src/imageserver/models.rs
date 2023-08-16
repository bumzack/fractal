pub mod imageservermodels {
    use serde_derive::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Hash, Eq)]
    pub struct Image {
        pub filename: String,
        pub prompt: String,
        pub created_at: String,
        pub url: String,
        pub timestamp: u64,
        pub id: u32,
    }

    #[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Hash, Eq)]
    pub struct Images {
        pub images: Vec<Image>,
    }
}
