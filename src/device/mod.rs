use ringbuf::HeapRb;

pub trait Device {
  fn input_stream() -> HeapRb::<i16>;
  fn output_stream() -> HeapRb::<i16>;
  fn next_input() -> i16;
  fn next_output() -> i16;
}
