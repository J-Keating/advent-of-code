import d9Data from "./data/d9_data.js";

namespace d9 {
  class Point {
    row: number;
    col: number;

    constructor(row: number, col: number) {
      this.row = row;
      this.col = col;
    }
  }

  function toNumArray(stringArray: string[]): number[][] {
    let width: number = stringArray[0].length;
    let height: number = stringArray.length;

    let board: number[][] = new Array<number[]>(height).fill([]).map((row) => new Array<number>(width).fill(0));

    for (var col: number = 0; col < height; col++) {
      let stringRow: string = stringArray[col];
      for (var row: number = 0; row < width; row++) {
        board[col][row] = +stringRow.charAt(row);
      }
    }
    return board;
  }

  function findLowPoints(board: number[][]): Point[] {
    let height: number = board.length;
    let width: number = board[0].length;

    let lowPoints: Point[] = new Array();
    for (var row: number = 0; row < height; row++) {
      for (var col: number = 0; col < width; col++) {
        var base: number = board[row][col];
        var rLess: number = row - 1;
        var rMore: number = row + 1;
        var cLess: number = col - 1;
        var cMore: number = col + 1;
        
        var isMin: boolean = true;
        if (rLess >= 0 && board[rLess][col] <= base) {
          isMin = false;
        }
        if (rMore < height && board[rMore][col] <= base) {
          isMin = false;
        }
        if (cLess >= 0 && board[row][cLess] <= base) {
          isMin = false;
        }
        if (cMore < width && board[row][cMore] <= base) {
          isMin = false;
        }

        if (isMin) {
          lowPoints.push(new Point(row, col));
        }
      }
    }
    return lowPoints;
  }

  export function part1(desc: string, stringArray: string[]) {
    let board: number[][] = toNumArray(stringArray);
    let lowPoints: Point[] = findLowPoints(board);

    let riskLevel: number = 0;
    lowPoints.forEach((point) => {
      riskLevel += board[point.row][point.col] + 1;
    });

    console.log(`Total (${desc}) = ${riskLevel}`);
  }

  function addIfBasin(board: number[][], visited: boolean[][], point: Point, basinPoints: Point[]) {
    let height: number = board.length;
    let width: number = board[0].length;

    if (point.row < 0 || point.row >= height || point.col < 0 || point.col >= width) {
      return;
    }
    if ((board[point.row][point.col] == 9)) {
      return;
    }
    if (visited[point.row][point.col]) {
      return;
    }
    visited[point.row][point.col] = true;
    basinPoints.push(new Point(point.row, point.col));

    addIfBasin(board, visited, new Point(point.row - 1, point.col), basinPoints);
    addIfBasin(board, visited, new Point(point.row + 1, point.col), basinPoints);
    addIfBasin(board, visited, new Point(point.row, point.col - 1), basinPoints);
    addIfBasin(board, visited, new Point(point.row, point.col + 1), basinPoints);
  }

  function findBasinSize(board: number[][], point: Point): number {
    let visited: boolean[][] = new Array<boolean[]>(board.length).fill([]).map((row) => new Array<boolean>(board[0].length).fill(false));
    var basinPoints: Point[] = new Array<Point>();
    addIfBasin(board, visited, point, basinPoints);
    return basinPoints.length;
  }

  export function part2(desc: string, stringArray: string[]) {
    let board: number[][] = toNumArray(stringArray);
    let lowPoints: Point[] = findLowPoints(board);

    let basinSizes: number[] = new Array<number>(lowPoints.length);
    for (var i = 0; i < lowPoints.length; i++) {
      basinSizes[i] = findBasinSize(board, lowPoints[i]);
    }

    let maxSizes:number[] = basinSizes.sort((a,b) => a - b).slice(-3);
    let ret:number = maxSizes.reduce((p, c) => p * c);
    console.log(`Total (${desc}) = ${ret}`);
  }
}

d9.part1("test", d9Data.testData);
d9.part1("real", d9Data.realData);
d9.part2("test", d9Data.testData);
d9.part2("real", d9Data.realData);
