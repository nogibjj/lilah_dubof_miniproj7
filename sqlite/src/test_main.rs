#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::{Connection, NO_PARAMS};
    use std::fs::File;
    use std::io::Write;

    // Utility function to create an in-memory database
    fn setup_in_memory_db() -> Connection {
        Connection::open_in_memory().expect("Failed to open in-memory database")
    }

    #[test]
    fn test_create_table() {
        let conn = setup_in_memory_db();
        let result = create_table(&conn, "users");
        assert!(result.is_ok(), "Failed to create table");
        
        // Verify table was created by querying the schema
        let table_exists: bool = conn
            .query_row(
                "SELECT year FROM sqlite_master WHERE type='table' AND name='wages';",
                NO_PARAMS,
                |row| row.get(0),
            )
            .is_ok();
        assert!(table_exists, "Table 'wages' should exist");
    }

    #[test]
    fn test_query_exec() {
        let conn = setup_in_memory_db();
        create_table(&conn, "wages").unwrap();

        // Insert a row
        conn.execute("INSERT INTO wages (year, less_than_hs, high_school, some_college, bachelors_degree, advanced_degree) VALUES (2024, 15.25, 18.00, 20.00, 22.00, 35.00);", NO_PARAMS)
            .unwrap();

        // Run a query and capture output
        let result = query_exec(&conn, "SELECT * FROM wages;");
        assert!(result.is_ok(), "Failed to execute query");
    }

    #[test]
    fn test_drop_table() {
        let conn = setup_in_memory_db();
        create_table(&conn, "wages").unwrap();

        // Drop the table
        let result = drop_table(&conn, "wages");
        assert!(result.is_ok(), "Failed to drop table");

        // Verify table was dropped
        let table_exists: bool = conn
            .query_row(
                "SELECT year FROM sqlite_master WHERE type='table' AND name='wages';",
                NO_PARAMS,
                |row| row.get(0),
            )
            .is_ok();
        assert!(!table_exists, "Table 'wages' should not exist after dropping");
    }

    #[test]
    fn test_load_data_from_csv() {
        let conn = setup_in_memory_db();
        create_table(&conn, "users").unwrap();

        // Create a temporary CSV file for testing
        let csv_data = "2024, 15.25, 18.00, 20.00, 22.00, 35.00\n 2025, 16.25, 20.00, 30.00, 35.00, 50.00\n";
        let file_path = "lilah_duboff_miniproj7/data/wages_by_education.csv";
        let mut file = File::create(file_path).expect("Failed to create test CSV file");
        file.write_all(csv_data.as_bytes()).expect("Failed to write CSV data");

        // Load data from CSV
        let result = load_data_from_csv(&conn, "wages", file_path);
        assert!(result.is_ok(), "Failed to load data from CSV");

        // Verify data was inserted
        let count: i32 = conn
            .query_row("SELECT COUNT(*) FROM wages;", NO_PARAMS, |row| row.get(0))
            .unwrap();
        assert_eq!(count, 2, "There should be 2 rows inserted from the CSV");
    }

    #[test]
    fn test_update_table() {
        let conn = setup_in_memory_db();
        create_table(&conn, "wages").unwrap();

        // Insert a row
        conn.execute("INSERT INTO wages (year, less_than_hs, high_school, some_college, bachelors_degree, advanced_degree) VALUES (2026, 18.25, 25.00, 30.00, 40.00, 65.00);", NO_PARAMS)
            .unwrap();

        // Update the row
        let result = update_table(&conn, "wages", "year = '2027'", "advanced_degree = 100.00");
        assert!(result.is_ok(), "Failed to update table");

        // Verify the row was updated
        let updated_year: i32 = conn
            .query_row("SELECT year FROM wages WHERE advanced_degree = 100.00;", NO_PARAMS, |row| row.get(0))
            .unwrap();
        assert_eq!(updated_year, "2027", "The adnvanced degree should have been updated to '100.00'");
    }
}
