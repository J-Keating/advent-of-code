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

  add(other: Point) {
    this.row += other.row;
    this.col += other.col;
  }
}

export class PointXY {
  x: number;
  y: number;

  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }

  add(other: PointXY) {
    this.x += other.x;
    this.y += other.y;
  }
}

export class Point3 {
  x: number;
  y: number;
  z: number;

  constructor(x: number, y: number, z: number) {
    this.x = x;
    this.y = y;
    this.z = z;
  }

  add(other: Point3) {
    this.x += other.x;
    this.y += other.y;
    this.z += other.z;
  }

  min(other: Point3): Point3 {
    return new Point3(Math.min(this.x, other.x), Math.min(this.y, other.y), Math.min(this.z, other.z));
  }

  max(other: Point3): Point3 {
    return new Point3(Math.max(this.x, other.x), Math.max(this.y, other.y), Math.max(this.z, other.z));
  }
}
