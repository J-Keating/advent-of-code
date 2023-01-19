import { ifError } from "assert";
import { assert } from "console";
import d20Data from "./data/d20_data.js";
import DataHelpers from "./data/dataHelpers.js";

namespace d20 {
  let BUFFER_SIZE = 60;

  function charToBit(char: string): number {
    switch (char) {
      case "#":
        return 1;
      case ".":
        return 0;
      default:
        assert(false);
    }
    return 0;
  }

  function loadImage(imageStrings: string[]): number[][] {
    let height: number = imageStrings.length + 2 * BUFFER_SIZE;
    let width: number = imageStrings[0].length + 2 * BUFFER_SIZE;
    let image: number[][] = new Array<number[]>(height);
    for (let i = 0; i < BUFFER_SIZE; i++) {
      image[i] = new Array<number>(width).fill(0);
      image[height - i - 1] = new Array<number>(width).fill(0);
    }
    imageStrings.forEach((inRow, y) => {
      let imageLine = new Array<number>(width).fill(0);
      [...inRow].forEach((char, x) => {
        imageLine[BUFFER_SIZE + x] = charToBit(char);
      });
      image[BUFFER_SIZE + y] = imageLine;
    });
    return image;
  }

  function printImage(image: number[][]) {
    image.forEach((row) => {
      console.log(row.map((bit) => (bit == 1 ? "#" : ".")).join(""));
    });
  }

  function intFromPatch(image: number[][], x: number, y: number): number {
    let ret = 0;
    for (let dy = -1; dy <= 1; dy++) {
      for (let dx = -1; dx <= 1; dx++) {
        ret = ret * 2 + image[y + dy][x + dx];
      }
    }
    return ret;
  }

  function runEnhancement(image: number[][], algorithm: string): number[][] {
    let height = image.length;
    let width = image[0].length;
    let backgroundIndex = intFromPatch(image, 1, 1);
    let newImage: number[][] = new Array<number[]>(height);
    for (let y = 0; y < height; y++) {
      newImage[y] = new Array<number>(width).fill(charToBit(algorithm[backgroundIndex]));
    }
    for (let y = 1; y < height - 1; y++) {
      for (let x = 1; x < width - 1; x++) {
        let index = intFromPatch(image, x, y);
        newImage[y][x] = charToBit(algorithm[index]);
      }
    }
    return newImage;
  }

  export function part1(desc: string, algo: string, imageStrings: string[]) {
    let image = loadImage(imageStrings);
    //printImage(image);
    image = runEnhancement(image, algo);
    image = runEnhancement(image, algo);
    //printImage(image);
    let count = image.reduce((acc, row) => {
      return acc + row.reduce((acc, bit) => acc + bit, 0);
    }, 0);
    console.log(`${desc}: ${count}`);
  }

  export function part2(desc: string, algo: string, imageStrings: string[]) {
    let image = loadImage(imageStrings);
    //printImage(image);
    for (let i = 0; i < 50; i++) {
      image = runEnhancement(image, algo);
    }
    //printImage(image);
    let count = image.reduce((acc, row) => {
      return acc + row.reduce((acc, bit) => acc + bit, 0);
    }, 0);
    console.log(`${desc}: ${count}`);
  }
}

export function solve() {
  console.log(`Day 20:`);
  d20.part1("P1 (test)", d20Data.testAlgo, d20Data.testImage);
  d20.part1("P1 (real)", d20Data.realAlgo, d20Data.realImage);
  d20.part2("P2 (test)", d20Data.testAlgo, d20Data.testImage);
  d20.part2("P2 (real)", d20Data.realAlgo, d20Data.realImage);
}

// P1 (test): 35
// P1 (real): 5065
// P2 (test): 3351
// P2 (real): 14790
