import d15Data from "./data/d15_data.js";
import DataHelpers from "./data/dataHelpers.js";
import { Point } from "./data/dataHelpers.js";

namespace d15 {
  var levels: number[][];
  var visited: boolean[][];
  var width: number;
  var height: number;
  var bestScoreXXX: number;
  var bestPath: Point[];
  var bestScores: number[][];

  function moveTo(visitPath: Point[], loc: Point, score: number) {
    if (loc.row < 0 || loc.col < 0 || loc.row >= height || loc.col >= width) {
      return;
    }
    if (visited[loc.row][loc.col] || visitPath.length > 1 * (height + width)) {
      return;
    }
    let newScore: number = score + levels[loc.row][loc.col];
    if (loc.row == height - 1 && loc.col == width - 1) {
      if (newScore < bestScoreXXX) {
        bestPath = visitPath.map((p) => p);
        bestScoreXXX = newScore;
      }
      return;
    }
    if (newScore > bestScoreXXX) {
      return;
    }
    // Recurse
    visitPath.push(loc);
    visited[loc.row][loc.col] = true;
    moveTo(visitPath, new Point(loc.row + 1, loc.col), newScore);
    moveTo(visitPath, new Point(loc.row, loc.col + 1), newScore);
    moveTo(visitPath, new Point(loc.row, loc.col - 1), newScore);
    moveTo(visitPath, new Point(loc.row - 1, loc.col), newScore);
    visited[loc.row][loc.col] = false;
    visitPath.pop();
  }

  export function BADpart1(desc: string, lines: string[]) {
    levels = DataHelpers.stringArrayToNumArray(lines);
    width = levels[0].length;
    height = levels.length;
    bestScoreXXX = 0;
    for (var r = 0; r < height; r++) {
      bestScoreXXX += levels[r][0];
    }
    for (var c = 0; c < width; c++) {
      bestScoreXXX += levels[height - 1][c];
    }
    visited = new Array<boolean[]>(height).fill([]).map((row) => new Array<boolean>(width).fill(false));

    let visitPath: Point[] = new Array<Point>();
    moveTo(visitPath, new Point(0, 0), -levels[0][0]);

    console.log(`P2 Score (${desc}) = ` + bestScoreXXX + " " + bestPath.reduce((p, c) => p + levels[c.row][c.col], 0));
  }

  function stepOut(loc: Point, score: number) {
    if (loc.row < 0 || loc.col < 0 || loc.row >= height || loc.col >= width) {
      return;
    }
    let newScore: number = score + levels[loc.row][loc.col];
    if (newScore > bestScoreXXX) {
      return;
    }
    if (loc.row == height - 1 && loc.col == width - 1) {
      if (newScore < bestScoreXXX) {
        bestScoreXXX = newScore;
      }
      return;
    }
    if (newScore >= bestScoreXXX) {
      return;
    }

    if (newScore <= bestScores[loc.row][loc.col]) {
      bestScores[loc.row][loc.col] = newScore;
      stepOut(new Point(loc.row + 1, loc.col), newScore);
      stepOut(new Point(loc.row, loc.col + 1), newScore);
      stepOut(new Point(loc.row, loc.col - 1), newScore);
      stepOut(new Point(loc.row - 1, loc.col), newScore);
    }
  }

  export function part1SuperSlow(desc: string, lines: string[]) {
    levels = DataHelpers.stringArrayToNumArray(lines);
    width = levels[0].length;
    height = levels.length;
    bestScoreXXX = width * height;
    bestScores = new Array<number[]>(height).fill([]).map((row) => new Array<number>(width).fill(bestScoreXXX));
    visited = new Array<boolean[]>(height).fill([]).map((row) => new Array<boolean>(width).fill(false));

    stepOut(new Point(0, 0), -levels[0][0]);

    console.log(`P2 Score (${desc}) = ${bestScores[height - 1][width - 1]}`);
  }

  function processNeighbor(loc: Point, score: number, toProcess: Point[]) {
    if (loc.row < 0 || loc.col < 0 || loc.row >= height || loc.col >= width) {
      return;
    }
    let newScore: number = score + levels[loc.row][loc.col];
    if (newScore < bestScores[loc.row][loc.col]) {
      bestScores[loc.row][loc.col] = newScore;
      toProcess.push(loc);
    }
  }

  function solve(desc: string) {
    width = levels[0].length;
    height = levels.length;
    bestScores = new Array<number[]>(height).fill([]).map((row) => new Array<number>(width).fill(width * height));

    let toProcess: Point[] = new Array<Point>();
    toProcess.push(new Point(0, 0));
    bestScores[0][0] = 0;
    while (toProcess.length > 0) {
      let p: Point = toProcess.shift() ?? new Point(-1, -1);
      let score: number = bestScores[p.row][p.col];
      processNeighbor(new Point(p.row - 1, p.col), score, toProcess);
      processNeighbor(new Point(p.row + 1, p.col), score, toProcess);
      processNeighbor(new Point(p.row, p.col - 1), score, toProcess);
      processNeighbor(new Point(p.row, p.col + 1), score, toProcess);
    }

    console.log(`P2 Score (${desc}) = ${bestScores[height - 1][width - 1]}`);
  }

  export function part1(desc: string, lines: string[]) {
    levels = DataHelpers.stringArrayToNumArray(lines);
    solve(desc);
  }

  export function part2(desc: string, lines: string[]) {
    let origLevels: number[][] = DataHelpers.stringArrayToNumArray(lines);
    let origWidth: number = origLevels[0].length;
    let origHeight: number = origLevels.length;

    width = origWidth * 5;
    height = origHeight * 5;
    levels = new Array<number[]>(height).fill([]).map((row) => new Array<number>(width).fill(0));
    for (var cy = 0; cy < 5; cy++) {
      for (var cx = 0; cx < 5; cx++) {
        origLevels.forEach((row, rIdx) => {
          row.forEach((val, cIdx) => {
            levels[origHeight * cy + rIdx][origWidth * cx + cIdx] = ((val + cx + cy - 1) % 9) + 1;
          });
        });
      }
    }
    solve(desc);
  }
}

//d15.part1("test", d15Data.testGrid);
//d15.part1("real", d15Data.realGrid);
d15.part2("test", d15Data.testGrid);
d15.part2("real", d15Data.realGrid);
