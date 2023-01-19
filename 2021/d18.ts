import { assert } from "console";
import DataHelpers from "./data/dataHelpers.js";

namespace d18 {
  class Digit {
    constructor(value: number, depth: number) {
      this.value = value;
      this.depth = depth;
    }
    equals(other: Digit): boolean {
      return this.value == other.value && this.depth == other.depth;
    }
    value: number = 0;
    depth: number = 0;
  }

  type SnailFishNumber = Digit[];

  function addSnailFish(a: SnailFishNumber, b: SnailFishNumber): SnailFishNumber {
    return a.concat(b).map((d) => new Digit(d.value, d.depth + 1));
  }

  function snailFishNumberFromLine(line: string) {
    let depth = 0;
    let number: SnailFishNumber = [];
    for (let char of line) {
      if (char == "[") {
        depth++;
      } else if (char == "]") {
        depth--;
      } else if (char == ",") {
      } else {
        number.push(new Digit(+char, depth));
      }
    }
    return number;
  }

  function loadData(filename: string): SnailFishNumber[] {
    var lines: string[] = DataHelpers.fileToStringArray(filename);
    return lines.map((line) => snailFishNumberFromLine(line));
  }

  function printSnailFishNumber(sfn: SnailFishNumber) {
    let max_height: number = sfn.map((d) => d.depth).reduce((a, b) => Math.max(a, b));
    var lines = new Array<string>(max_height + 1).fill(" ".repeat(sfn.length));
    sfn.forEach((d, i) => {
      lines[d.depth] = lines[d.depth].slice(0, i) + d.value + lines[d.depth].slice(i + 1);
    });
    lines.forEach((l) => console.log(l));
  }

  function findExplodeLocation(sfn: SnailFishNumber): number {
    let ret = sfn.findIndex((d) => d.depth >= 5);
    if (ret >= 0) {
      assert(sfn[ret].depth == sfn[ret + 1].depth);
    }
    return ret;
  }

  function explode(sfn: SnailFishNumber, explodeLocation: number): SnailFishNumber {
    // 2 numbers to one number.  Depth is one less than before.
    let left = sfn[explodeLocation].value;
    let right = sfn[explodeLocation + 1].value;
    if (explodeLocation > 0) {
      sfn[explodeLocation - 1].value += left;
    }
    if (explodeLocation + 2 < sfn.length) {
      sfn[explodeLocation + 2].value += right;
    }
    let newDepth = sfn[explodeLocation].depth - 1;
    return sfn
      .slice(0, explodeLocation)
      .concat(new Digit(0, newDepth))
      .concat(sfn.slice(explodeLocation + 2));
  }

  export function testExplode() {
    let tests = [
      "[[[[[9,8],1],2],3],4]",
      "[[[[0,9],2],3],4]",
      "[7,[6,[5,[4,[3,2]]]]]",
      "[7,[6,[5,[7,0]]]]",
      "[[6,[5,[4,[3,2]]]],1]",
      "[[6,[5,[7,0]]],3]",
      "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
      "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
      "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
      "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
    ];

    for (let i = 0; i < tests.length; i += 2) {
      let sfn = snailFishNumberFromLine(tests[i]);
      let newSfn = explode(sfn, findExplodeLocation(sfn));
      let testSfn = snailFishNumberFromLine(tests[i + 1]);
      assert(newSfn.length == testSfn.length);
      newSfn.forEach((d, i) => assert(d.equals(testSfn[i])));
    }
  }

  function findSplitLocation(sfn: SnailFishNumber): number {
    let ret = sfn.findIndex((d) => d.value >= 10);
    return ret;
  }

  function split(sfn: SnailFishNumber, explodeLocation: number): SnailFishNumber {
    // One number to two numbers.  Depth is one greater than before.
    let current = sfn[explodeLocation].value;
    let left = Math.floor(current / 2);
    let right = Math.floor(current / 2 + 0.5);
    let newDepth = sfn[explodeLocation].depth + 1;
    return sfn
      .slice(0, explodeLocation)
      .concat(new Digit(left, newDepth))
      .concat(new Digit(right, newDepth))
      .concat(sfn.slice(explodeLocation + 1));
  }

  export function testSplit() {
    let tests = ["[[[[1,1],2],3],4]", "[[[[[5,5],1],2],3],4]"];

    for (let i = 0; i < tests.length; i += 2) {
      let sfn = snailFishNumberFromLine(tests[i]);
      sfn[0].value = 10;
      let newSfn = split(sfn, findSplitLocation(sfn));
      let testSfn = snailFishNumberFromLine(tests[i + 1]);
      assert(newSfn.length == testSfn.length);
      newSfn.forEach((d, i) => assert(d.equals(testSfn[i])));
    }
  }

  function reduce(sfn: SnailFishNumber): SnailFishNumber {
    while (true) {
      let explodeLocation = findExplodeLocation(sfn);
      if (explodeLocation >= 0) {
        sfn = explode(sfn, explodeLocation);
        continue;
      }
      let splitLocation = findSplitLocation(sfn);
      if (splitLocation >= 0) {
        sfn = split(sfn, splitLocation);
        continue;
      }
      break;
    }
    return sfn;
  }

  export function testReduce() {
    let added = addSnailFish(
      snailFishNumberFromLine("[[[[4,3],4],4],[7,[[8,4],9]]]"),
      snailFishNumberFromLine("[1,1]")
    );
    let reduced = reduce(added);
    console.log(reduced);
  }

  function snailFishMagnitude(sfn: SnailFishNumber): number {
    let max_depth: number = 0;
    while (sfn.length > 1) {
      let max_depth = sfn.map((d) => d.depth).reduce((a, b) => Math.max(a, b));
      let deepest_index_first_digit = sfn.findIndex((d) => d.depth == max_depth);
      let magnitude = 3 * sfn[deepest_index_first_digit].value + 2 * sfn[deepest_index_first_digit + 1].value;
      sfn = sfn
        .slice(0, deepest_index_first_digit)
        .concat(new Digit(magnitude, max_depth - 1))
        .concat(sfn.slice(deepest_index_first_digit + 2));
    }
    return sfn[0].value;
  }

  export function testMagnitude() {
    assert(snailFishMagnitude(snailFishNumberFromLine("[9,1]")) == 29);
    assert(snailFishMagnitude(snailFishNumberFromLine("[1,9]")) == 21);
    assert(snailFishMagnitude(snailFishNumberFromLine("[[9,1],[1,9]]")) == 129);
    assert(
      snailFishMagnitude(snailFishNumberFromLine("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")) == 3488
    );
  }

  export function part1(desc: string, filename: string) {
    var numbers = loadData(filename);
    let sfn: SnailFishNumber = numbers[0];
    for (let i = 1; i < numbers.length; i++) {
      sfn = addSnailFish(sfn, numbers[i]);
      sfn = reduce(sfn);
    }
    //printSnailFishNumber(sfn);
    console.log(`${desc}: ${snailFishMagnitude(sfn)}`);
  }

  export function part2(desc: string, filename: string) {
    var numbers = loadData(filename);
    let max = 0;
    for (let i = 0; i < numbers.length; i++) {
      for (let j = 0; j < numbers.length; j++) {
        if (i == j) continue;
        let sfn: SnailFishNumber = addSnailFish(numbers[i], numbers[j]);
        sfn = reduce(sfn);
        let mag = snailFishMagnitude(sfn);
        if (mag > max) max = mag;
      }
    }
    console.log(`${desc}: ${max}`);
  }
}

export function solve() {
  console.log(`Day 18:`);
  // d18.testExplode();
  // d18.testSplit();
  // d18.testReduce();
  // d18.testMagnitude();
  d18.part1("P1 (test)", "./data/d18_test.txt");
  d18.part1("P1 (test2)", "./data/d18_test2.txt");
  d18.part1("P1 (real)", "./data/d18_real.txt");
  d18.part2("P2 (test2)", "./data/d18_test2.txt");
  d18.part2("P2 (real)", "./data/d18_real.txt");
}

// Day 18:
// P1 (test): 3488
// P1 (test2): 4140
// P1 (real): 3816
// P2 (test2): 3993
// P2 (real): 4819
