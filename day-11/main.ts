import * as fs from "fs";

function main() {
  stage1();
}

const stage1 = () => {
  let input = readFile("input/input1.txt");
  for (let i = 0; i < 25; i++) {
    input = blink(input);
  }
  console.log(input.length);
};

const readFile = (filename: string): number[] => {
  let fileContent = "";
  try {
    fileContent = fs.readFileSync(filename, "utf8");
  } catch (err) {
    console.error(err);
  }
  const numbers = fileContent.split(" ").map((word) => parseInt(word));
  return numbers;
};

const blink = (numbers: number[]): number[] => {
  const result: number[] = [];

  numbers.forEach((number) => {
    if (number === 0) {
      // Rule 1: Replace 0 with 1
      result.push(1);
    } else if (number.toString().length % 2 === 0) {
      // Rule 2: Split numbers with even digits
      const numberString = number.toString();
      const half = numberString.length / 2;
      const left = parseInt(numberString.substring(0, half));
      const right = parseInt(numberString.substring(half));
      result.push(left);
      result.push(right);
    } else {
      // Rule 3: Multiply by 2024 for all other cases
      result.push(number * 2024);
    }
  });

  return result;
};

main();
