//type MagicError = Box<dyn std::error::Error>;
use std::collections::BTreeMap;

pub type Display = Vec<BTreeMap<String, String>>;
pub type DisplaySchema = BTreeMap<String, String>;

pub fn capitalize(mut cap_me: String) -> String {
    //! Takes a mutable string and capitalizes the first character and returns it
    cap_me.replace_range(..1, &cap_me.get(..1).unwrap().to_string().to_uppercase());
    cap_me
}
pub mod db_tools {
    use super::*;
    use sqlx::postgres::{PgPoolOptions, PgRow, Postgres};
    use sqlx::{Column, Pool, Row};
    use std::collections::BTreeMap;
    //use sqlx::{FromRow, Pool, Row};

    pub async fn db_connection_async() -> Result<Pool<Postgres>, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://peony:free144@localhost/breaker")
            .await
    }

    fn rows_to_btm(rows: Vec<PgRow>) -> BTreeMap<String, String> {
        let mut table_info = BTreeMap::new();
        rows.into_iter().for_each(|row| {
            let key = capitalize(row.get(1));
            let value = capitalize(row.get(2));
            table_info.insert(key, value);
        });

        table_info
    }

    fn nps_vc_to_string(row: &PgRow, index: usize) -> Result<String, sqlx::Error> {
        let string = match row.try_get(index) {
            Ok(string) => string,
            Err(_) => {
                let int: i32 = row.try_get(index)?;
                int.to_string()
            }
        };

        Ok(string)
    }

    pub async fn anything_table(
        table: &str,
        pool: &Pool<Postgres>,
    ) -> Vec<BTreeMap<String, String>> {
        // SQL
        let sql = &format!("SELECT * FROM {};", table);

        // Query the database
        let rows = sqlx::query(&sql).fetch_all(pool).await.unwrap();

        // Create a container to hold allll the things
        let mut vec = Vec::new();

        // For each Row in the database query of Rows
        rows.into_iter().for_each(|ref row| {
            // Create a container for its Column Names & Values
            let mut sub_btm = BTreeMap::new();

            // For each value in the Row
            (0..row.len()).into_iter().for_each(|index| {
                // Get the name of the column
                let column_name = row.column(index).name().to_string();

                // Get the value of column
                let column_value = nps_vc_to_string(row, index).unwrap();

                // Shove it into the btreemap
                sub_btm.insert(column_name, column_value);
            });

            // Shove it into the mass vec collection
            vec.push(sub_btm);
        });

        // Return all the goody goody gumdrop data
        vec
    }

    pub async fn anything_schema(table: &str, pool: &Pool<Postgres>) -> DisplaySchema {
        let sql = format!("SELECT table_name, column_name, data_type FROM information_schema.columns WHERE table_name = '{}';", table);

        let rows = sqlx::query(&sql).fetch_all(pool).await.unwrap();

        // Convert database query into a BTreeMap (mostly for prettyness)
        let table = rows_to_btm(rows);

        table
    }
}
