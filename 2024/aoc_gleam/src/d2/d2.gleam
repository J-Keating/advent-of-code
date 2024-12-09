import gleam/int
import gleam/io
import gleam/list
import gleam/option.{type Option, None, Some}
import gleam/result
import gleam/string
import utils/file_utils

fn check_next_two(
  numbers: List(Int),
  want_ascending_option: Option(Bool),
  allow_bad_level_in: Bool,
) -> Bool {
  case numbers {
    [first, second, ..rest] -> {
      let diff = second - first
      let abs_diff = int.absolute_value(diff)
      let step_size_ok = abs_diff >= 1 && abs_diff <= 3
      let ascending = diff > 0
      let ascending_ok = case want_ascending_option {
        Some(want_ascending) -> {
          ascending == want_ascending
        }
        None -> {
          True
        }
      }
      let should_recurse = ascending_ok && step_size_ok
      case should_recurse, allow_bad_level_in {
        True, _ -> {
          check_next_two([second, ..rest], Some(ascending), allow_bad_level_in)
        }
        False, True -> {
          check_next_two([first, ..rest], Some(ascending), False)
        }
        False, False -> {
          False
        }
      }
      // should_recurse
      // && check_next_two([second, ..rest], Some(ascending), allow_bad_level_out)
    }
    [_] -> True
    [] -> panic as "Shouldn't get here"
  }
}

fn check_report(numbers: List(Int), allow_bad_level_in: Bool) -> Bool {
  check_next_two(numbers, None, allow_bad_level_in)
}

pub fn count_good_reports(filename: String, allow_bad_level_in: Bool) {
  // Read file
  let lines = file_utils.read_file_as_string_list(filename)
  // Convert to list of list of numbers
  let reports =
    list.map(lines, fn(line) {
      let split_line: List(String) =
        line
        |> string.trim()
        |> string.split(" ")
      //io.debug(split_line)
      // let numbers =
      split_line
      |> list.map(fn(number_as_string) {
        number_as_string
        |> int.parse
        |> result.unwrap(0)
      })
    })
  //list.each(reports, fn(report) { io.debug(report) })
  let good_reports = list.filter(reports, check_report(_, allow_bad_level_in))
  //io.debug(good_reports)
  //io.debug(list.length(good_reports))
  io.debug(list.length(good_reports))
}

const day = "d2"

fn part1() {
  count_good_reports("src\\" <> day <> "\\test_data.txt", False)
  count_good_reports("src\\" <> day <> "\\data.txt", False)
  //2
  //269
}

// fn part2() {
//   //count_good_reports("src\\" <> day <> "\\test_data.txt", True)
//   //count_good_reports("src\\" <> day <> "\\data.txt", True)
//   //4
//   //???
// }

pub fn solve() {
  io.debug("Solving day 2 (1)")
  part1()
  //part2()
}
