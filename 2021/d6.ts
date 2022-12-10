namespace d6 {
  let testData = [3, 4, 3, 1, 2];
  let realData = [
    1, 1, 1, 2, 1, 5, 1, 1, 2, 1, 4, 1, 4, 1, 1, 1, 1, 1, 1, 4, 1, 1, 1, 1, 4, 1, 1, 5, 1, 3, 1, 2, 1, 1, 1, 2, 1, 1, 1,
    4, 1, 1, 3, 1, 5, 1, 1, 1, 1, 3, 5, 5, 2, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 5, 4, 1, 1, 1, 1, 1, 3, 1, 1, 2, 4, 4,
    1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 5, 1, 3, 1, 5, 1, 2, 1, 1, 5, 1, 1, 1, 5, 3, 3, 1, 4, 1, 3, 1, 3, 1, 1, 1, 1, 3, 1,
    4, 1, 1, 1, 1, 1, 2, 1, 1, 1, 4, 2, 1, 1, 5, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 5, 1, 1, 1, 1, 3,
    1, 1, 1, 1, 1, 3, 4, 1, 2, 1, 3, 2, 1, 1, 2, 1, 1, 1, 1, 4, 1, 1, 1, 1, 4, 1, 1, 1, 1, 1, 2, 1, 1, 4, 1, 1, 1, 5, 3,
    2, 2, 1, 1, 3, 1, 5, 1, 5, 1, 1, 1, 1, 1, 5, 1, 4, 1, 2, 1, 1, 1, 1, 2, 1, 3, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 3, 1, 4,
    3, 1, 4, 1, 3, 2, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 5, 1, 1, 1, 1, 2, 1, 1, 1, 3, 5, 1, 1, 1, 1,
    5, 1, 1, 2, 1, 2, 4, 2, 2, 1, 1, 1, 5, 2, 1, 1, 5, 1, 1, 1, 1, 5, 1, 1, 1, 2, 1,
  ];

  function simulateDays(ages_in: number[], days: number): number {
    var ages: number[] = new Array<number>(...ages_in);
    while (days-- > 0) {
      let birthCount: number = 0;
      for (var i = 0; i < ages.length; ++i) {
        if (--ages[i] < 0) {
          ages[i] = 6;
          birthCount++;
        }
      }
      ages.push(...Array(birthCount).fill(8));
      // console.log(ages);
    }
    return ages.length;
  }

  export function part1() {
    let days: number = 80;
    console.log("After " + days + " days count = " + simulateDays(realData, days));
  }

  function simulateDays2(ages: number[], days: number): number {
    let counts: number[] = new Array(9).fill(0);
    ages.forEach((a) => counts[a]++);

    // console.log(ages);
    // console.log(counts);

    while (days-- > 0) {
      let spawning = counts[0];
      counts = counts.slice(1);
      counts[6] += spawning;
      counts.push(spawning);

      //console.log(counts);
    }
    return counts.reduce((prev: number, curr: number) => prev + curr);
  }

  export function part2() {
    let days: number = 256;
    console.log("After " + days + " days count = " + simulateDays2(realData, days));
  }
}

d6.part1();
d6.part2();
