import file_streams/file_stream.{type FileStream}
import file_streams/file_stream_error
import gleam/list
import gleam/string

fn append_next_line_to_array(
  stream: FileStream,
  accumlator: List(String),
) -> List(String) {
  case file_stream.read_line(stream) {
    Error(file_stream_error.Eof) -> accumlator
    Ok(next_line) ->
      append_next_line_to_array(stream, list.append(accumlator, [next_line]))
    _ -> panic as "Unexpected line parsing"
  }
}

fn read_buffer_as_string_list(stream: FileStream) -> List(String) {
  append_next_line_to_array(stream, [])
}

pub fn read_file_as_string_list(filename: String) -> List(String) {
  let assert Ok(stream) = file_stream.open_read(filename)
  read_buffer_as_string_list(stream)
}

fn append_next_line_to_string(stream: FileStream, accumlator: String) -> String {
  case file_stream.read_line(stream) {
    Error(file_stream_error.Eof) -> accumlator
    Ok(next_line) ->
      append_next_line_to_string(stream, string.append(accumlator, next_line))
    _ -> panic as "Unexpected line parsing"
  }
}

fn read_buffer_as_single_string(stream: FileStream) -> String {
  append_next_line_to_string(stream, "")
}

pub fn read_file_as_single_string(filename: String) -> String {
  let assert Ok(stream) = file_stream.open_read(filename)
  read_buffer_as_single_string(stream)
}
