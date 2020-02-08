mod source {
  struct CharLocation {
    line: i32,
    column: i32,
  }

  struct Source {
    path: String,
    content: String,
  }
}