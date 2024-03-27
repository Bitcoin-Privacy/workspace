use std::{fmt::Debug, marker::PhantomData};

use bonsaidb::core::{
    async_trait::async_trait,
    schema::{Collection, SerializedCollection},
};
use bonsaidb_local::AsyncDatabase;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Trait describing the common behavior of
// a repository. TEntity is the type of
// domain entity handled by this repository.
#[async_trait]
pub trait Repository<TEntity> {
    async fn query_all(&self) -> Vec<TEntity>;
    async fn query_by_id(&self, id: String) -> Option<TEntity>;
    async fn insert(&self, data: TEntity) -> TEntity;
    async fn edit(&self, id: String, data: TEntity) -> Option<TEntity>;
}

pub struct BonsaiRepository<'a, TData> {
    // gives access to a BonsaiDB database
    db: &'a AsyncDatabase,
    // required as generic type is not (yet) used in the struct
    phantom: PhantomData<TData>,
}

#[async_trait]
impl<'a, TData: Debug> Repository<TData> for BonsaiRepository<'a, TData>
// bounds are necessary to comply with BonsaiDB API
where
    TData:
        SerializedCollection<Contents = TData> + Collection<PrimaryKey = String> + 'static + Unpin,
{
    async fn query_all(&self) -> Vec<TData> {
        let docs = TData::all_async(self.db).await.unwrap();
        let entities: Vec<_> = docs.into_iter().map(|f| f.contents).collect();
        entities
    }
    // note that id is not required here, as already part of data
    async fn insert(&self, data: TData) -> TData {
        let new_doc = data.push_into_async(self.db).await.unwrap();
        new_doc.contents
    }
    async fn edit(&self, id: String, data: TData) -> Option<TData> {
        let doc = TData::overwrite_async(&id, data, self.db).await;
        match doc {
            Ok(doc) => Some(doc.contents),
            Err(_) => None,
        }
    }
    async fn query_by_id(&self, id: String) -> Option<TData> {
        let doc = TData::get_async(&id, self.db).await.unwrap().unwrap();
        Some(doc.contents)
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Collection)]
#[collection( // custom key definition for BonsaiDB
    name="classifiers",
    primary_key = String,
    natural_id = Some(self._id.clone()))]
pub struct Classifier {
    pub _id: String,
    pub name: String,
    pub is_interface: bool,
}

// classifier service holding a typed repository
pub struct ClassifierService {
    // constraints required by Tauri to support multi threading
    repository: Box<dyn Repository<Classifier> + Send + Sync>,
}

impl ClassifierService {
    fn new(repository: Box<dyn Repository<Classifier> + Send + Sync>) -> Self {
        Self { repository }
    }
}

impl ClassifierService {
    pub async fn create_new_classifier(&self, new_name: &str) -> Classifier {
        // we have to manage the ids on our own, so create a new one here
        let id = Uuid::new_v4().to_string();
        let new_classifier = self
            .repository
            .insert(Classifier {
                _id: id.to_string(),
                name: new_name.to_string(),
                is_interface: false,
            })
            .await;
        new_classifier
    }

    pub async fn update_classifier_name(&self, id: &str, new_name: &str) -> Classifier {
        let mut classifier = self.repository.query_by_id(id.to_string()).await.unwrap();
        classifier.name = new_name.to_string();
        // we need to copy the id because "edit" owns the containing struct
        let id = classifier._id.clone();
        let updated = self.repository.edit(id, classifier).await.unwrap();
        updated
    }
}
