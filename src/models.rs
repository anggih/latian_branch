

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use crate::schema::accounts;

#[derive(Queryable, Serialize)]
pub struct  Account {
        pub id:i64,
        pub nama:String,
        pub email:String,
        pub alamat:String
}

#[derive(Insertable)]
#[table_name="accounts"]
pub struct NewAccount<'a>{
    pub nama:&'a str,
    pub email:&'a str,
    pub alamat:&'a str,
} 
