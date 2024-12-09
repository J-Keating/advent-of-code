import gleam/bool
import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import simplifile

pub fn is_safe(line: List(Int)) -> Bool {
  let windows =
    line
    |> list.window_by_2
    |> list.map(fn(nums) { nums.0 - nums.1 })
  windows |> list.all(fn(n) { n > 0 && n < 4 })
  || windows |> list.all(fn(n) { n < 0 && n > -4 })
}

fn part_1(input: List(List(Int))) -> Int {
  list.count(input, is_safe)
}

fn part_2(input: List(List(Int))) -> Int {
  list.count(input, fn(line) {
    use <- bool.guard(is_safe(line), True)
    list.combinations(line, list.length(line) - 1)
    |> list.find(is_safe)
    |> result.is_ok
  })
}

pub fn solve() {
  let assert Ok(input) = simplifile.read("src\\d2\\data.txt")
  let lines =
    input
    |> string.split("\r\n")
    |> list.filter_map(fn(line) {
      string.split(line, " ") |> list.map(int.parse) |> result.all
    })

  io.debug("Solving day 2 (2)")
  part_1(lines) |> io.debug
  part_2(lines) |> io.debug
}
