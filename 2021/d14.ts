import d14Data from "./data/d14_data.js";

namespace d14 {
  class insertionData {
    index: number = 0;
    insertionChar: string = "";
  }

  export function part1(desc: string, start: string, insertionStrings: string[]) {
    let insMap = new Map<string, string>();
    insertionStrings.forEach((i) => {
      let sp = i.split(" -> ");
      insMap.set(sp[0], sp[1]);
    });

    let step: number = 0;
    while (++step <= 40) {
      let allInsertions = new Array<insertionData>();
      insMap.forEach((value, key) => {
        let pos: number = start.indexOf(key);
        while (pos >= 0) {
          allInsertions.push({ index: pos, insertionChar: value });
          pos = start.indexOf(key, pos + 1);
        }
      });
      allInsertions
        .sort((a, b) => a.index - b.index)
        .reverse()
        .forEach((id) => {
          //console.log(`inserting ${id.insertionChar} at ${id.index}`);
          start = start.slice(0, id.index + 1) + id.insertionChar + start.slice(id.index + 1);
        });
      //console.log(start);
    }
    let countMap = new Map<string, number>();
    [...start].forEach((c) => {
      if (!countMap.has(c)) {
        countMap.set(c, [...start].filter((t) => t == c).length);
      }
    });
    let diff = Math.max(...countMap.values()) - Math.min(...countMap.values());
    console.log(`P1 Score (${desc}) = ${diff}`);
  }

  function addCount(map: Map<string, number>, pair: string, count: number) {
    if (map.has(pair)) {
      count += map.get(pair) ?? 0;
    }
    map.set(pair, count);
  }

  export function part2(desc: string, start: string, insertionStrings: string[]) {
    let insMap = new Map<string, string>();
    insertionStrings.forEach((i) => {
      let sp = i.split(" -> ");
      insMap.set(sp[0], sp[1]);
    });

    let countMap = new Map<string, number>();
    for (let i = 0; i < start.length - 1; ++i) {
      addCount(countMap, start.slice(i, i + 2), 1);
    }

    let iteration: number = 0;
    while (++iteration <= 40) {
      let newMap = new Map<string, number>();
      countMap.forEach((count, pair) => {
        if (insMap.has(pair)) {
          let l: string = pair.charAt(0) + insMap.get(pair);
          let r: string = insMap.get(pair) + pair.charAt(1);
          addCount(newMap, l, count);
          addCount(newMap, r, count);
        } else {
          addCount(newMap, pair, count);
        }
      });

      countMap = newMap;
    }

    let letterCounts = new Map<string, number>();
    countMap.forEach((count, pair) => {
      addCount(letterCounts, pair.charAt(0), count);
      addCount(letterCounts, pair.charAt(1), count);
    });
    addCount(letterCounts, start.charAt(0), 1);
    addCount(letterCounts, start.charAt(start.length - 1), 1);

    let diff = Math.max(...letterCounts.values()) / 2 - Math.min(...letterCounts.values()) / 2;
    console.log(`P2 Score (${desc}) = ${diff}`);
  }
}

//d14.part1("test", d14Data.testStart, d14Data.testInsertions);
//d14.part1("real", d14Data.realStart, d14Data.realInsertions);
d14.part2("test", d14Data.testStart, d14Data.testInsertions);
d14.part2("real", d14Data.realStart, d14Data.realInsertions);
