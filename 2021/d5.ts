import d5Data from "./data/d5_data.js";

namespace d5 {
  function computeMaxes(points: Array<Array<number>>): Array<number> {
    let maxX: number = Math.max(...points.map((p) => p[0]));
    let maxY: number = Math.max(...points.map((p) => p[1]));
    return [maxX, maxY];
  }

  function computeBounds(): Array<number> {
    var fromMax: Array<number> = computeMaxes(d5Data.from);
    var toMax: Array<number> = computeMaxes(d5Data.to);

    var maxX: number = Math.max(fromMax[0], toMax[0]);
    var maxY: number = Math.max(fromMax[1], toMax[1]);

    return [maxX + 1, maxY + 1];
  }

  export function countOverlap(description:string, allowDiagonal:boolean): void {
    var maxX: number;
    var maxY: number;
    [maxX, maxY] = computeBounds();
    console.log(maxX, maxY);

    var counts: number[][] = new Array(maxX).fill(0).map(() => new Array(maxY).fill(0));

    for (var i = 0; i < d5Data.from.length; ++i) {
      var from: Array<number> = d5Data.from[i];
      var to: Array<number> = d5Data.to[i];

      var diffX: number = to[0] - from[0];
      var diffY: number = to[1] - from[1];

      var horizontalOrVertical: boolean = (diffX == 0 && diffY != 0) || (diffX != 0 && diffY == 0);
      var diagnonal:boolean = (Math.abs(diffX) == Math.abs(diffY));
      var good:boolean = (horizontalOrVertical || (diagnonal && allowDiagonal));
      if (!good) {
        continue;
      }

      var stepX: number = diffX == 0 ? 0 : diffX > 0 ? 1 : -1;
      var stepY: number = diffY == 0 ? 0 : diffY > 0 ? 1 : -1;

      var X: number = from[0];
      var Y: number = from[1];

      while (X != to[0] || Y != to[1]) {
        counts[X][Y]++;
        X += stepX;
        Y += stepY;
      }
      counts[X][Y]++;
    }

    // Compute number of spaces where lines overlap
    var overlapCount: number = 0;
    counts.forEach((col) => {
      col.filter((count) => count > 1).forEach(() => overlapCount++);
    });

    console.log(description, overlapCount);
  }
}

d5.countOverlap("Part1: ", false);
d5.countOverlap("Part2: ", true);
