use wasm_bindgen::prelude::*;
use regex::Regex;

#[wasm_bindgen]
pub fn create_sql_schema_from_csv(
  csv_filename: &str,
  csv_contents: &[u8]
) -> String {
  let mut sql_statements: Vec<String> = Vec::new();

  let table_name = csv_filename.strip_suffix(".csv").unwrap();

  let mut still_processing_header: bool = true;

  let mut columns: Vec<String> = Vec::new();

  let mut column: Vec<String> = Vec::new();

  let sql_column_name_expression = Regex::new(r"[a-zA-Z_]").unwrap();

  for character in csv_contents.into_iter() {
    let string_character =
      std::str::from_utf8(&[character.clone()]).unwrap().to_string();

    if still_processing_header == true {
      if sql_column_name_expression.is_match(&string_character) {
        column.push(string_character);
      } else if &string_character == "," {
        columns.push(column.join(""));

        column.clear();
      } else if &string_character == "\n" {
        columns.push(column.join(""));

        column.clear();

        still_processing_header = false;

        break;
      }
    }
  }

  let final_statement =
    format!("CREATE TABLE {} ({});", table_name, columns.join(", "));

  sql_statements.push(final_statement);

  return sql_statements.join("\n\n");
}

