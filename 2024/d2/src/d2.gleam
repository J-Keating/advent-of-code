// 
import file_utils
import gleam/int
import gleam/io
import gleam/list
import gleam/option.{type Option, None, Some}
import gleam/regexp
import gleam/result
import gleam/string

fn check_next_two(
  numbers: List(Int),
  want_ascending_option: Option(Bool),
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
      ascending_ok
      && step_size_ok
      && check_next_two([second, ..rest], Some(ascending))
    }
    [_] -> True
    [] -> panic as "Shouldn't get here"
  }
}

fn check_report(numbers: List(Int)) {
  check_next_two(numbers, None)
}

pub fn count_good_reports(filename: String) {
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
  let good_reports = list.filter(reports, check_report)
  //io.debug(good_reports)
  io.debug(list.count(good_reports, fn(_) { True }))
}

fn d2_part1() {
  count_good_reports("test_data_d2.txt")
  count_good_reports("data_d2.txt")
}

// fn d2_part2() {
//   todo
// }

// fn d2() {
//   d2_part1()
//   //269
//   //d2_part2()
// }

fn match_muls(filename: String) {
  let options = regexp.Options(case_insensitive: False, multi_line: True)
  let assert Ok(re) = regexp.compile("mul\\((\\d+),(\\d+)\\)", options)
  let lines = file_utils.read_file_as_single_string(filename)
  //io.debug(lines)
  let matches: List(regexp.Match) = regexp.scan(re, lines)
  //io.debug(matches)
  let results =
    list.map(matches, fn(match: regexp.Match) {
      case match.submatches {
        [Some(x), Some(y)] -> {
          result.unwrap(int.parse(x), 0) * result.unwrap(int.parse(y), 0)
        }
        _ -> panic as "Not exactly two matches"
      }
    })
  io.debug(list.fold(results, 0, int.add))
}

fn d3_part1() {
  // 161
  // 173731097
  match_muls("test_data_d3.txt")
  match_muls("data_d3.txt")
}

fn d3() {
  d3_part1()
}

// fn d5() {
//   let lines = file_utils.read_file_as_string_list(filename)
// }

pub fn main() {
  d3()
}
