import gleam/int
import gleam/io
import gleam/list
import gleam/option.{Some}
import gleam/regexp
import gleam/result
import utils/file_utils

fn match_muls(filename: String) {
  let lines = file_utils.read_file_as_single_string(filename)
  let options = regexp.Options(case_insensitive: False, multi_line: True)
  let assert Ok(re) = regexp.compile("mul\\((\\d+),(\\d+)\\)", options)
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

pub type Instruction {
  Do
  Dont
  Mul(a: Int, b: Int)
}

pub type Pt2State {
  Pt2State(doing: Bool, sum: Int)
}

type Input =
  List(Instruction)

pub fn parse(input: String) -> Input {
  let assert Ok(r) =
    regexp.compile(
      "do\\(\\)|don't\\(\\)|mul\\((\\d{1,3}),(\\d{1,3})\\)",
      regexp.Options(False, False),
    )
  let matches = regexp.scan(r, input)
  list.map(matches, fn(match) {
    case match.content {
      "do()" -> Do
      "don't()" -> Dont
      _ -> {
        let assert [option.Some(a), option.Some(b)] = match.submatches
        let assert Ok(a) = int.parse(a)
        let assert Ok(b) = int.parse(b)
        Mul(a, b)
      }
    }
  })
}

pub fn pt_1(input: Input) -> Int {
  list.fold(input, 0, fn(acc, ins) {
    case ins {
      Mul(a, b) -> acc + { a * b }
      _ -> acc
    }
  })
}

pub fn pt_2(input: Input) -> Int {
  list.fold(input, Pt2State(True, 0), fn(acc, ins) {
    case ins {
      Do -> Pt2State(..acc, doing: True)
      Dont -> Pt2State(..acc, doing: False)
      Mul(a, b) ->
        Pt2State(
          ..acc,
          sum: acc.sum
            + case acc.doing {
              True -> a * b
              False -> 0
            },
        )
    }
  }).sum
}

const day = "d3"

fn part1() {
  match_muls("src\\" <> day <> "\\test_data.txt")
  match_muls("src\\" <> day <> "\\data.txt")
  // 161
  // 173731097
}

fn part2() {
  let input =
    parse(file_utils.read_file_as_single_string(
      "src\\" <> day <> "\\test_data2.txt",
    ))
  io.debug(pt_2(input))
  let input =
    parse(file_utils.read_file_as_single_string("src\\" <> day <> "\\data.txt"))
  io.debug(pt_2(input))
  // 48
  // 93729253
}

pub fn solve() {
  io.debug("Solving day 3")
  io.debug("Part1:")
  part1()
  io.debug("Part2:")
  part2()
}
