extern crate core;

mod ingest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::borrow::Cow;
use std::path::Path;
use surrealdb_rs::param::Root;
use surrealdb_rs::protocol::Ws;
use surrealdb_rs::{Result, Surreal};
use chrono::prelude::*;

// use log::{info};

#[derive(Serialize, Deserialize)]
struct Author {
    first: Cow<'static, str>,
    last: Cow<'static, str>,
}

#[derive(Serialize, Deserialize)]
struct ADR {
    #[serde(skip_serializing)]
    id: Option<String>,
    title: Cow<'static, str>,
    time: DateTime<Utc>,
    name: Author,
    draft: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    ingest::ingest(
        // Path::new(".")
        std::env::current_dir().unwrap().as_ref()
    );
    // env_logger::init();

    // info!(target: "db_events", "Connecting to DB: {:?}", addr);
    let client = Surreal::connect::<Ws>("localhost:8000").await?;
    // info!(target: "db_events", "Connected to DB {:?}", addr);

    // Signin as a namespace, database, or root user
    // info!(target: "db_events", "Signing in...");
    client
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await?;
    // info!(target: "db_events", "Signed in.");

    // Select a specific namespace and database
    // info!(target: "db_events", "Using...");
    client.use_ns("test").use_db("test").await?;
    // info!(target: "db_events", "Used.");

    println!("Creating first ADR");

    // Create a new adr with a random ID
    let tobie: ADR = client
        .create("adr")
        .content(ADR {
            id: None,
            title: "Adopt Technology 1".into(),
            time: Utc::now(),
            name: Author {
                first: "Kevin".into(),
                last: "Chen".into(),
            },
            draft: true,
        })
        .await?;

    assert!(tobie.id.is_some());

    println!("Creating second ADR");

    // Create a new adr with a specific ID
    let mut adr2: ADR = client
        .create(("adr", "adr2"))
        .content(ADR {
            id: None,
            title: "Adopt Technology 2".into(),
            time: Utc::now(),
            name: Author {
                first: "Kevin".into(),
                last: "Chen".into(),
            },
            draft: false,
        })
        .await?;

    assert_eq!(adr2.id.unwrap(), "adr:adr2");

    // Update a adr record with a specific ID
    adr2 = client
        .update(("adr", "adr2"))
        .merge(json!({ "draft": true }))
        .await?;

    assert!(adr2.draft);

    // Select all adrs records
    let adrs: Vec<ADR> = client.select("adr").await?;

    assert!(!adrs.is_empty());

    // Perform a custom advanced query
    let groups = client
        .query("
            SELECT draft,
                   count()
            FROM type::table($table)
            GROUP BY draft
        ")
        .bind("table", "adr")
        .await?;

    dbg!(groups);

    // Delete all adrs upto but not including adr2
    client.delete("adr").range(.."adr2").await?;

    // Delete all adrs
    client.delete("adr").await?;

    Ok(())
}