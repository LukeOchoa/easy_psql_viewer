// Axum
use axum::{routing::get, Json, Router};

// Serde
use serde::Serialize;
use serde_json::json;

// Standard Library
use std::net::SocketAddr;

// Herp Derp
use axum_test::db_tools::*;
use axum_test::*;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/netherportals", get(nether_portals_browser))
        .route("/userprofile", get(userprofile_browser))
        .route("/json", get(some_json));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3003));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn home() -> Json<serde_json::Value> {
    let available_routes = json!({
        "netherportals": "",
        "userprofile": "",
        "/json": ""
    });
    Json(available_routes)
}
async fn some_json() -> Json<serde_json::Value> {
    Json(json!({"Breaker": "Death to the breaker."}))
}

async fn nether_portals_browser() -> Json<serde_json::Value> {
    // Create Database Connection
    let pool = &db_connection_async().await.unwrap();

    // Struct to create "prettyness" when json is displayed in the browser
    #[derive(Serialize)]
    struct Payload {
        #[serde(rename = "Table NetherPortals")]
        netherportals: std::collections::BTreeMap<String, String>, //IndexMap<String, String>, //Vec<HashMap<String, String>>, //Vec<TableInfo>,
        #[serde(rename = "Columns&Rows NetherPortals")]
        nps_values: Display, //Vec<BTreeMap<String, String>>,
    }

    // Assemble Payload Members...

    // Get Table Schema? poor man's schema...
    let netherportals = anything_schema("netherportals", pool).await;

    // Get column&row values for nps
    let nps_values = anything_table("netherportals", pool).await;

    //<>=======================<>\\

    // Initialize the struct for json
    let payload = Payload {
        netherportals,
        nps_values,
    };

    // Serialize to Json
    let json_payload = serde_json::to_value(&payload).unwrap();

    // Test

    // Return Json
    Json(json_payload)
}

async fn userprofile_browser() -> Json<serde_json::Value> {
    #[derive(Serialize)]
    struct Payload {
        userprofile: DisplaySchema,
        userprofile_cvs: Display,
    }

    // Create Database Connection
    let pool = &db_connection_async().await.unwrap();

    // Sql Line
    //let sql = "SELECT table_name, column_name, data_type FROM information_schema.columns WHERE table_name = userprofile;";

    // Databse Query
    let userprofile = anything_schema("userprofile", pool).await;

    // Sql Line
    //let sql = "SELECT * FROM userprofile;"

    // Database Query
    let userprofile_cvs = anything_table("userprofile", pool).await;

    let payload = Payload {
        userprofile,
        userprofile_cvs,
    };
    let payload = serde_json::to_value(payload).unwrap();
    Json(payload)
}

// async fn testx(pool: &sqlx::Pool<sqlx::Postgres>) {
//     let sql = "INSERT INTO kingtest(id, somdata) VALUES(5, 'five');";
//     sqlx::query(sql).execute(pool).await.unwrap();
// }

//async fn nps_tables_columns_values(pool: &Pool<Postgres>) -> Vec<BTreeMap<String, String>> {
//    // Create SQL statement to get all crap from (Table netherportals)
//    let table_cvs = format!("SELECT * from netherportals;");
//
//    // Query the database
//    let rows = sqlx::query(&table_cvs).fetch_all(pool).await.unwrap();
//
//    // Create a container to hold allll the things
//    let mut vec = Vec::new();
//
//    // For each Row in the database query of Rows
//    rows.into_iter().for_each(|ref row| {
//        // Create a container for its Column Names & Values
//        let mut sub_btm = BTreeMap::new();
//
//        // For each value in the Row
//        (0..row.len()).into_iter().for_each(|index| {
//            // Get the name of the column
//            let column_name = row.column(index).name().to_string();
//
//            // Get the value of column
//            let column_value = nps_vc_to_string(row, index).unwrap();
//
//            // Shove it into the btreemap
//            sub_btm.insert(column_name, column_value);
//        });
//
//        // Shove it into the mass vec collection
//        vec.push(sub_btm);
//    });
//
//    // Return all the goody goody gumdrop data
//    vec
//}

//async fn nether_portals_schema(pool: &Pool<Postgres>) -> BTreeMap<String, String> {
//    // Create SQL statement to get poor man's table schema
//    let table_info_sql = format!("SELECT table_name, column_name, data_type FROM information_schema.columns WHERE table_name = 'netherportals';");
//
//    // Query the database
//    let rows = sqlx::query(&table_info_sql).fetch_all(pool).await.unwrap();
//
//    // Convert database query into a BTreeMap (mostly for prettyness)
//    let table = rows_to_btm(rows);
//
//    table
//}
