use wasm_bindgen::prelude::*;
use csv::Reader;

#[derive(Clone)]
enum CsvCellDataType {
  Int,
  Float,
  Text,
  Empty
}

#[wasm_bindgen]
pub fn create_sql_schema_from_csv(
  csv_filename: &str,
  csv_contents: &[u8]
) -> String {
  let mut reader =
    ReaderBuilder::new().delimiter(b',').from_reader(csv_contents);

  let headers = reader.headers().unwrap();

  let (record_data_types, record_max_text_lengths) =
    create_csv_record_data_types_and_max_text_lengths(
      headers.len(),
      reader
    );

  let mut columns: Vec<String> = Vec::new();

  for (index, header) in headers.iter().enumerate() {
    let mut column = format!("VARCHAR({})", record_max_text_lengths[index]);

    match record_data_types[index] {
      crate::CsvCellDataType::Float => column = "FLOAT".to_string(),
      crate::CsvCellDataType::Int => column = "INTEGER".to_string()
    }

    columns.push(column);
  }

  let table_name = csv_filename.strip_suffix(".csv").unwrap();

  let final_statement =
    format!("CREATE TABLE {} ({});", table_name, columns.join(", "));

  let mut sql_statements: Vec<String> = Vec::new();

  sql_statements.push(final_statement);

  return sql_statements.join("\n\n");
}

fn create_csv_record_data_types_and_max_text_lengths(
  number_of_columns: usize,
  reader: Reader
) -> (Vec<crate::CsvCellDataType>, Vec<usize>) {
  let record_data_types: Vec<crate::CsvCellDataType> =
    vec![crate::CsvCellDataType::Empty; number_of_columns];

  let record_max_text_lengths: Vec<u32> = vec![0, number_of_columns];

  for record in reader.records().unwrap() {
    for (index, value) in record.iter().enumerate() {
      if value.len() > record_max_text_lengths[index] {
        record_max_text_lengths[index] = value.len();
      }

      let mut data_type = crate::CsvCellDataType::Empty;

      let mut parsed_value = value.parse::<u32>();

      if Err(parsed_value) {
        parsed_value = value.parse::<f64>();
      } else {
        data_type = crate::CsvCellDataType::Int;
      }

      if Err(parsed_value) {
        data_type = crate::CsvCellDataType::Text;
      } else {
        data_type = crate::CsvCellDataType::Float;
      }

      match data_type {
        crate::CsvCellDataType::Float => {
          if
            record_data_types[index] == crate::CsvCellDataType::Empty ||
            record_data_types[index] == crate::CsvCellDataType::Int
          {
            record_data_types[index] = crate::CsvCellDataType::Float;
          }
        },
        crate::CsvCellDataType::Int => {
          if record_data_types[index] == crate::CsvCellDataType::Empty {
            record_data_types[index] = crate::CsvCellDataType::Int;
          }
        },
        crate::CsvCellDataType::Text => {
          if record_data_types[index] != crate::CsvCellDataType::Text {
            record_data_types[index] = crate::CsvCellDataType::Text;
          }
        }
      }
    }
  }

  return (record_data_types, record_max_text_lengths);
}

