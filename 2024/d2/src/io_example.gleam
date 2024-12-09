// 
import file_streams/file_stream
import file_streams/file_stream_error

pub fn main() {
  let filename = "test.txt"

  // Write file
  let assert Ok(stream) = file_stream.open_write(filename)
  let assert Ok(Nil) = file_stream.write_bytes(stream, <<"Hello!\n":utf8>>)
  let assert Ok(Nil) = file_stream.write_chars(stream, "12")
  let assert Ok(Nil) = file_stream.close(stream)

  // Read file
  let assert Ok(stream) = file_stream.open_read(filename)
  let assert Ok(<<"Hello!\n":utf8>>) = file_stream.read_bytes(stream, 7)
  let assert Ok([49, 50]) =
    file_stream.read_list(stream, file_stream.read_uint8, 2)
  let assert Error(file_stream_error.Eof) = file_stream.read_bytes(stream, 1)

  // Reset file position to the start and read line by line (not currently
  // supported on JavaScript)
  let assert Ok(0) =
    file_stream.position(stream, file_stream.BeginningOfFile(0))
  let assert Ok("Hello!\n") = file_stream.read_line(stream)
  let assert Ok("12") = file_stream.read_line(stream)
  let assert Error(file_stream_error.Eof) = file_stream.read_line(stream)
  let assert Ok(Nil) = file_stream.close(stream)
}
