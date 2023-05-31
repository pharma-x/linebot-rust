use firestore::*;
use std::env;
use std::sync::Arc;

#[derive(Clone)]
pub struct Firestore(pub(crate) Arc<FirestoreDb>);

impl Firestore {
    pub async fn new() -> Self {
        let firestore = FirestoreDb::new(
            &env::var("FIRESTORE_PROJECT_ID")
                .unwrap_or_else(|_| panic!("FIRESTORE_PROJECT_ID must be set!")),
        )
        .await
        .unwrap_or_else(|_| {
            panic!("Cannot connect to the Firestore. Please check your configuration.")
        });

        Self(Arc::new(firestore))
    }
}
