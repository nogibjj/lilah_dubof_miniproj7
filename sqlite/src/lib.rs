use csv::ReaderBuilder; //for loading from csv
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fs::File; //for loading csv //for capturing errors from loading
                   // Here we will have a function for each of the commands

// Create a table
pub fn create_table(conn: &Connection, table_name: &str) -> Result<()> {
    let create_query = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            year INTEGER PRIMARY KEY,
            less_than_hs INTEGER NOT NULL,
            high_school INTEGER NOT NULL,
            some_college INTEGER NOT NULL,
            bachelors_degree INTEGER NOT NULL,
            advanced_degree INTEGER NOT NULL
        )",
        table_name
    );
    conn.execute(&create_query, [])?;
    println!("Table '{}' created successfully.", table_name);
    Ok(()) //returns nothing except an error if it occurs
}

//Read
pub fn query_exec(conn: &Connection, query_string: &str) -> Result<()> {
    // Prepare the query and iterate over the rows returned
    let mut stmt = conn.prepare(query_string)?;

    // Use query_map to handle multiple rows
    let rows = stmt.query_map([], |row| {
        // Assuming the `users` table has an `id` and `name` column
        let year: i32 = row.get(0)?;
        let less_than_hs: i32 = row.get(1)?;
        let high_school: i32 = row.get(2)?;
        let some_college: i32 = row.get(3)?;
        let bachelors_degree: i32 = row.get(4)?;
        let advanced_degree: i32 = row.get(5)?;
        Ok((
            year,
            less_than_hs,
            high_school,
            some_college,
            bachelors_degree,
            advanced_degree,
        ))
    })?;

    // Iterate over the rows and print the results
    for row in rows {
        let (year, less_than_hs, high_school, some_college, bachelors_degree, advanced_degree) =
            row?;
        println!("year: {}, Less than HS: {}, High School: {}, Some College: {}, Bachelors Degree: {}, Advanced Degree: {}", year, less_than_hs, high_school, some_college, bachelors_degree, advanced_degree);
    }

    Ok(())
}

//delete
pub fn drop_table(conn: &Connection, table_name: &str) -> Result<()> {
    let drop_query = format!("DROP TABLE IF EXISTS {}", table_name);
    conn.execute(&drop_query, [])?;
    println!("Table '{}' dropped successfully.", table_name);
    Ok(())
}

//load data from a file path to a table
pub fn load_data_from_csv(
    conn: &Connection,
    table_name: &str,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    //Box<dyn Error> is a trait object that can represent any error type
    let file = File::open(file_path).expect("failed to open the file path");
    let mut rdr = ReaderBuilder::new().from_reader(file);

    let insert_query = format!(
        "INSERT INTO {} (year, less_than_hs, high_school, some_college, bachelors_degree, advanced_degree) VALUES (?, ?, ?, ?, ?, ?)",
        table_name
    );
    //this is a loop that expects a specific schema, you will need to change this if you have a different schema
    for result in rdr.records() {
        let record = result.expect("failed to parse a record");
        //let year: i32 = record[0].trim().parse().expect("failed to parse year"); //.parse() is a method that converts a string into a number
        let year_str = record[0].trim();
        let year: i32 = match year_str.parse() {
            Ok(year) => year,
            Err(_) => {
                println!("Failed to parse year '{}'", year_str);
                // You can either return an error, panic, or set a default value
                // For example, you can set a default value like this:
                0
            }
        };
        println!("year: {}", year);
        let less_than_hs: f32 = record[1].trim().parse()?;
        let high_school: f32 = record[2].trim().parse()?;
        let some_college: f32 = record[3].trim().parse()?;
        let bachelors_degree: f32 = record[4].trim().parse()?;
        let advanced_degree: f32 = record[5].trim().parse().expect("failed to parse advanced degree");
        println!("year: {}, Less than HS: {}, High School: {}, Some College: {}, Bachelors Degree: {}, Advanced Degree: {}", year, less_than_hs, high_school, some_college, bachelors_degree, advanced_degree);
        
        conn.execute(
            &insert_query,
            params![
                year,
                less_than_hs,
                high_school,
                some_college,
                bachelors_degree,
                advanced_degree
            ],
        ).expect("failed to execute data into db table");
    }
    println!(
        "Data loaded successfully from '{}' into table '{}'.",
        file_path, table_name
    );
    Ok(())
}
pub fn update_table(
    conn: &Connection,
    table_name: &str,
    set_clause: &str,
    condition: &str,
) -> Result<()> {
    // Construct the SQL UPDATE query using the provided table name, set clause, and condition
    let update_query = format!(
        "UPDATE {} SET {} WHERE {};",
        table_name, set_clause, condition
    );

    // Execute the update query
    let affected_rows = conn.execute(&update_query, [])?;

    // Output the number of rows updated
    println!(
        "Successfully updated {} row(s) in table '{}'.",
        affected_rows, table_name
    );

    Ok(())
}
