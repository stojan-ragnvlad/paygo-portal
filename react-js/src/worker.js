import init, { create_sql_schema_from_csv } from '../lib/compile_time_tools';

onmessage = function (event) {
  const reader = new FileReader();

  reader.readAsArrayBuffer(event.data);

  reader.onload = () => {
    init().then(() => {
      const sql_query = create_sql_schema_from_csv(
        event.data.name,
        new Uint8Array(reader.result)
      );

      postMessage(sql_query);
    });
  }

  reader.onerror = (error) => postMessage(error);
}

