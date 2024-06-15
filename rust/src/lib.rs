use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn create_sql_schema_from_csv(
  csv_filename: &str,
  csv_contents: &[u8]
) -> String {
  let mut sql_statements: Vec<String> = Vec::new();

  sql_statements.push(format!("CREATE TABLE {};", csv_filename));

  return sql_statements.join("\n\n");
}

