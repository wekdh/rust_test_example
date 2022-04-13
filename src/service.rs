use mongodb::{error::Error, results::InsertOneResult, sync::Collection};

use bson::oid::ObjectId;

#[derive(Clone)]
pub struct UserService {
    collection: mongodb::sync::Collection<bson::Document>,
}

impl UserService {
    pub fn new(collection: Collection<bson::Document>) -> UserService {
        UserService { collection }
    }

    pub fn create(&self, name: &str, age:&str) -> Result<InsertOneResult, Error> {
        self.collection.insert_one(bson::doc! {"name": name, "age":age}, None)
    }

    pub fn get(&self, id:&str) -> Result<Option<bson::Document>, Error> {

        //let find_options = mongodb::options::FindOneOptions::builder().projection(bson::doc!{"_id":1}).build();

        self.collection.find_one( bson::doc! {"_id": ObjectId::parse_str(id).expect("id is not valid ObjectID") } , None )
    }
}
