use wasm_bindgen::prelude::*;
use csv::ReaderBuilder;
use csv::StringRecordsIter;

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
      reader.records().unwrap()
    );

  let mut columns: Vec<String> = Vec::new();

  for (index, header) in headers.iter().enumerate() {
    let mut column = format!("VARCHAR({})", record_max_text_lengths[index]);

    match record_data_types[index] {
      CsvCellDataType::Float => column = "FLOAT",
      CsvCellDataType::Int => column = "INTEGER"
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
  number_of_columns: u32,
  records: StringRecordsIter
) -> (Vec<CsvCellDataType>, Vec<u32>) {
  let record_data_types: Vec<CsvCellDataType> =
    vec![CsvCellDataType::Empty; number_of_columns];

  let record_max_text_lengths: Vec<u32> = vec![0, number_of_columns];

  for record in records {
    for (index, value) in record.iter().enumerate() {
      if value.len() > record_max_text_lengths[index] {
        record_max_text_lengths[index] = value.len();
      }

      let mut data_type = CsvCellDataType::Empty;

      let mut parsed_value = value.parse::<CsvCellDataType::Int>();

      if Err(parsed_value) {
        parsed_value = value.parse::<CsvCellDataType::Float>();
      } else {
        data_type = CsvCellDataType::Int;
      }

      if Err(parsed_value) {
        data_type = CsvCellDataType::Text;
      } else {
        data_type = CsvCellDataType::Float;
      }

      match data_type {
        CsvCellDataType::Float => {
          if (
            record_data_types[index] == CsvCellDataType::Empty ||
            record_data_types[index] == CsvCellDataType::Int
          ) {
            record_data_types[index] = CsvCellDataType::Float;
          }
        },
        CsvCellDataType::Int => {
          if record_data_types[index] == CsvCellDataType::Empty {
            record_data_types[index] = CsvCellDataType::Int;
          }
        },
        CsvCellDataType::Text => {
          if record_data_types[index] != CsvCellDataType::Text {
            record_data_types[index] = CsvCellDataType::Text;
          }
        }
      }
    }
  }

  return (record_data_types, record_max_text_lengths);
}

