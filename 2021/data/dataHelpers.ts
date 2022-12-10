import * as fs from "fs";

export default class DataHelpers {
  public static stringArrayToNumArray(stringArray: string[]): number[][] {
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

  public static fileToNumArray(filename: string): number[][] {
    return this.stringArrayToNumArray(
      fs
        .readFileSync(filename, "utf8")
        .split("\n")
        .map((l) => l.trim())
    );
  }

  public static fileToStringArray(filename: string): string[] {
    return fs
      .readFileSync(filename, "utf8")
      .split("\n")
      .map((l) => l.trim());
  }
}

export class Point {
  row: number;
  col: number;

  constructor(row: number, col: number) {
    this.row = row;
    this.col = col;
  }
}

export class PointXY {
  x: number;
  y: number;

  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }
}
