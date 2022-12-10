import d11Data from "./data/d11_data.js";
import DataHelpers from "./data/dataHelpers.js";
import { Point } from "./data/dataHelpers.js";

namespace d11 {
  function simStep(levels: number[][]): number {
    let flashes: Point[] = new Array<Point>();
    levels.forEach((r, rowIdx) =>
      r.forEach((_, colIdx) => {
        if (++levels[rowIdx][colIdx] > 9) {
          flashes.push(new Point(rowIdx, colIdx));
        }
      })
    );

    let width: number = levels[0].length;
    let height: number = levels.length;
    let hasFlashed: boolean[][] = new Array(height).fill([]).map((row) => new Array(width).fill(false));

    let flashCount: number = flashes.length;
    while (flashes.length > 0) {
      let flashPos = flashes.pop() ?? new Point(-1, -1);
      //hasFlashed[flashPos.row][flashPos.col] = true;
      levels[flashPos.row][flashPos.col] = 0;
      for (let r = Math.max(flashPos.row - 1, 0); r < Math.min(flashPos.row + 2, height); r++) {
        for (let c = Math.max(flashPos.col - 1, 0); c < Math.min(flashPos.col + 2, width); c++) {
          if (!(flashPos.col == c && flashPos.row == r)) {
            if (levels[r][c] > 0 && levels[r][c] <= 9) {
              if (++levels[r][c] > 9) {
                flashCount++;
                flashes.push(new Point(r, c));
              }
            }
          }
        }
      }
    }
    return flashCount;
  }

  export function part1(desc: string, lines: string[], iterations: number) {
    let levels: number[][] = DataHelpers.stringArrayToNumArray(lines);
    let stepCount = 0;
    let flashCount: number = 0;
    while (stepCount++ < iterations) {
      flashCount += simStep(levels);
    }

    let score: number = 0;
    console.log(`P1 Total (${desc}) = ${flashCount}`);
  }

  export function part2(desc: string, lines: string[]) {
    let levels: number[][] = DataHelpers.stringArrayToNumArray(lines);
    let stepCount = 0;
    while (++stepCount < 2000) {
      if (simStep(levels) == levels.length * levels[0].length) {
        break;
      }
    }

    let ret: number = 0;
    console.log(`P2 Score (${desc}) = ${stepCount}`);
  }
}

d11.part1("test", d11Data.testData, 100);
d11.part1("real", d11Data.realData, 100);
d11.part2("test", d11Data.testData);
d11.part2("real", d11Data.realData);
