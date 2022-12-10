import d13Data from "./data/d13_data.js";
import { PointXY } from "./data/dataHelpers.js";

namespace d13 {
  function fold(points: PointXY[], mappingFunc: (_: PointXY) => PointXY): PointXY[] {
    let returnPoints: PointXY[] = new Array<PointXY>();
    points.forEach((p) => {
      let newPoint: PointXY = mappingFunc(p);
      if (returnPoints.find((p) => p.x == newPoint.x && p.y == newPoint.y) == undefined) {
        returnPoints.push(newPoint);
      }
    });
    return returnPoints;
  }

  function foldUp(points: PointXY[], yVal: number): PointXY[] {
    return fold(points, (p) => {
      if (p.y > yVal) {
        p.y = yVal - (p.y - yVal);
      }
      return p;
    });
  }

  function foldLeft(points: PointXY[], xVal: number): PointXY[] {
    return fold(points, (p) => {
      if (p.x > xVal) {
        p.x = xVal - (p.x - xVal);
      }
      return p;
    });
  }

  export function part1(desc: string, pointsIn: PointXY[], folds: string[]): PointXY[] {
    let points: PointXY[] = [...pointsIn];
    let foldPoints: { dir:string, value:number }[] = folds.map((s) => {
      let toks: string[] = s.split("=");
      return { dir:toks[0] , value:+toks[1] };
    });

    foldPoints.forEach(fp => {
      if (fp.dir == "y") {
        points = foldUp(points, fp.value);
      }
      if (fp.dir == "x") {
        points = foldLeft(points, fp.value);
      }
    })

    console.log(`P1 Score (${desc}) = ${points.length}`);
    return points;
  }

  export function part2(desc: string, pointsIn: PointXY[], folds: string[]) {
    let points: PointXY[] = part1("", pointsIn, folds);
    let maxX:number = Math.max(...points.map(p => p.x));
    let maxY:number = Math.max(...points.map(p => p.y));
    let board:string[][] = new Array<string[]>(maxY + 1).fill([]).map(row => new Array<string>(maxX + 1).fill(" "));
    points.forEach(p => board[p.y][p.x] = "#");
    board.forEach(line => console.log(line.reduce((p, c) => p + c), ""));
  }
}

// d13.part1("test", d13Data.testData, d13Data.testDataFolds);
// d13.part1("real", d13Data.realData, d13Data.realDataFolds);
d13.part2("test", d13Data.testData, d13Data.testDataFolds);
d13.part2("real", d13Data.realData, d13Data.realDataFolds);
