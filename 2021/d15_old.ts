import * as fs from "fs";

//import d15Data from "./data/d15_data.js";
import DataHelpers from "./data/dataHelpers.js";
import { Point } from "./data/dataHelpers.js";

namespace d15 {
  const directions: { rowStep: number; colStep: number }[] = [
    { rowStep: -1, colStep: 0 },
    { rowStep: 1, colStep: 0 },
    { rowStep: 0, colStep: -1 },
    { rowStep: 0, colStep: 1 },
  ];

  function processNeighbor(levels: number[][], bestScores: number[][], loc: Point, score: number, toProcess: Point[]) {
    if (loc.row < 0 || loc.col < 0 || loc.row >= levels.length || loc.col >= levels[0].length) {
      return;
    }
    let newScore: number = score + levels[loc.row][loc.col];
    if (newScore < bestScores[loc.row][loc.col]) {
      bestScores[loc.row][loc.col] = newScore;
      toProcess.push(loc);
    }
  }

  function solve(desc: string, levels: number[][]) {
    let width: number = levels[0].length;
    let height: number = levels.length;
    let bestScores: number[][] = new Array<number[]>(height)
      .fill([])
      .map((row) => new Array<number>(width).fill(width * height));

    let toProcess: Point[] = new Array<Point>();
    toProcess.push(new Point(0, 0));
    bestScores[0][0] = 0;
    while (toProcess.length > 0) {
      let p: Point = toProcess.shift() ?? new Point(-1, -1);
      let score = bestScores[p.row][p.col];
      directions.forEach((d) => {
        processNeighbor(levels, bestScores, new Point(p.row + d.rowStep, p.col + d.colStep), score, toProcess);
      });
    }

    console.log(`P2 Score (${desc}) = ${bestScores[height - 1][width - 1]}`);
  }

  export function part1(desc: string, filename: string) {
    var levels: number[][] = DataHelpers.fileToNumArray(filename);
    solve(desc, levels);
  }

  export function part2(desc: string, filename: string) {
    let origLevels: number[][] = DataHelpers.fileToNumArray(filename);
    let origWidth: number = origLevels[0].length;
    let origHeight: number = origLevels.length;

    var levels: number[][] = new Array<number[]>(origHeight * 5)
      .fill([])
      .map((row) => new Array<number>(origWidth * 5).fill(0));
    for (var cy = 0; cy < 5; cy++) {
      for (var cx = 0; cx < 5; cx++) {
        origLevels.forEach((row, rIdx) => {
          row.forEach((val, cIdx) => {
            levels[origHeight * cy + rIdx][origWidth * cx + cIdx] = ((val + cx + cy - 1) % 9) + 1;
          });
        });
      }
    }
    solve(desc, levels);
  }
}

d15.part1("test", "./data/d15_test.txt");
d15.part1("real", "./data/d15_real.txt");
d15.part2("test", "./data/d15_test.txt");
d15.part2("real", "./data/d15_real.txt");
