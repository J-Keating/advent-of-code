import { ifError } from "assert";
import { assert } from "console";
import d20Data from "./data/d20_data.js";
import DataHelpers from "./data/dataHelpers.js";

namespace d20 {
  function charToBit(char: string): number {
    switch (char) {
      case "#":
        return 1;
      case ".":
        return 0;
      default:
        assert(false);
    }
    return 0;
  }

  export function part1(desc: string, algo: string, imageStrings: string[]) {
    let image: number[][] = new Array<number[]>();
    imageStrings.forEach((inRow) => {
      [...inRow];
    });
    console.log(`P1 Score (${desc}) = ?`);
  }

  export function part2(desc: string, filename: string) {
    console.log(`P2 Score (${desc}) = ?`);
  }
}

export function solve() {
  d20.part1("test", d20Data.testAlgo, d20Data.testImage);
  d20.part1("real", d20Data.realAlgo, d20Data.realImage);
  // d20.part2("test", './data/d20_test.txt');
  // d20.part2("real", './data/d20_real.txt');
}
