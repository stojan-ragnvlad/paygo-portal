use wasm_bindgen::prelude::*;
use csv::ReaderBuilder;
use csv::Reader;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

macro_rules! console_log {
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Debug)]
#[derive(PartialEq)]
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

  let (record_data_types, record_max_text_lengths) =
    create_csv_record_data_types_and_max_text_lengths(
      reader.headers().unwrap().len(),
      &mut reader
    );

  let mut columns: Vec<String> = Vec::new();

  for (index, header) in reader.headers().unwrap().iter().enumerate() {
    let mut column =
      format!("{} VARCHAR({})", header, record_max_text_lengths[index]);

    match record_data_types[index] {
      crate::CsvCellDataType::Float => {
        column = header.to_string() + " FLOAT";
      },
      crate::CsvCellDataType::Int => {
        column = header.to_string() + " INTEGER";
      },
      _ => ()
    }

    columns.push(column);
  }

  let table_name = csv_filename.strip_suffix(".csv").unwrap();

  let mut final_statement =
    format!("CREATE TABLE {} ({});\n", table_name, columns.join(", "));

  final_statement.push_str(&format!("INSERT INTO {} VALUES ", table_name));

  reader = ReaderBuilder::new().delimiter(b',').from_reader(csv_contents);

  let records_count = reader.records().count();

  reader = ReaderBuilder::new().delimiter(b',').from_reader(csv_contents);

  for (record_index, record) in reader.records().enumerate() {
    let mut insert_string = "(".to_string();

    let values = record.unwrap();

    for (value_index, value) in values.iter().enumerate() {
      match record_data_types[value_index] {
        crate::CsvCellDataType::Int => {
          insert_string.push_str(&format!("{}", value));
        },
        crate::CsvCellDataType::Float => {
          insert_string.push_str(&format!("{}", value));
        },
        crate::CsvCellDataType::Text => {
          insert_string.push_str(&format!("'{}'", value));
        },
        crate::CsvCellDataType::Empty => {
          insert_string.push_str("NULL");
        }
      }

      if value_index < values.len() - 1 {
        insert_string.push_str(", ");
      } else {
        insert_string.push_str(")");
      }
    }

    if record_index < records_count - 1 {
      insert_string.push_str(",\n");
    } else {
      insert_string.push_str(";");
    }

    final_statement.push_str(&insert_string);
  }

  let mut sql_statements: Vec<String> = Vec::new();

  sql_statements.push(final_statement);

  return sql_statements.join("\n\n");
}

fn create_csv_record_data_types_and_max_text_lengths(
  number_of_columns: usize,
  reader: &mut Reader<&[u8]>
) -> (Vec<crate::CsvCellDataType>, Vec<usize>) {
  let mut record_data_types: Vec<crate::CsvCellDataType> =
    vec![crate::CsvCellDataType::Empty; number_of_columns];

  let mut record_max_text_lengths: Vec<usize> = vec![0; number_of_columns];

  for record in reader.records() {
    for (index, value) in record.unwrap().iter().enumerate() {
      if value.len() > record_max_text_lengths[index] {
        record_max_text_lengths[index] = value.len();
      }

      let mut data_type = crate::CsvCellDataType::Empty;

      let parsed_int = value.parse::<i32>();

      match parsed_int {
        Ok(_) => data_type = crate::CsvCellDataType::Int,
        Err(_) => {
          let parsed_float = value.parse::<f64>();

          match parsed_float {
            Ok(_) => data_type = crate::CsvCellDataType::Float,
            Err(_) => data_type = crate::CsvCellDataType::Text
          }
        }
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
        },
        _ => ()
      }
    }
  }

  return (record_data_types, record_max_text_lengths);
}

