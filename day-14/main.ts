import * as fs from "fs";

const MAP_SIZE_X = 101;
const MAP_SIZE_Y = 103;

type Coordinate = {
  x: number;
  y: number;
};

type Robot = {
  position: Coordinate;
  velocity: Coordinate;
};

const main = () => {
  stage1();
};

const stage1 = () => {
  let robotPositions = readFile("input/input1.txt");
  moveRobots(100, robotPositions);
  const robotsMap = getMap(robotPositions);
  printMap(robotsMap);
  const robotCountInQuadrants = countRobotsInQuadrants(robotsMap);
  console.log(robotCountInQuadrants);
  const factor = robotCountInQuadrants.reduce((a, b) => a * b, 1);
  console.log(factor);
};

const moveRobots = (seconds: number, robots: Robot[]) => {
  for (let i = 0; i < seconds; i++) {
    robots.forEach((robot) => {
      robot.position.x = mod(robot.position.x + robot.velocity.x, MAP_SIZE_X);
      robot.position.y = mod(robot.position.y + robot.velocity.y, MAP_SIZE_Y);
    });
  }
};

const getMap = (robots: Robot[]): string[][] => {
  const map = Array.from({ length: MAP_SIZE_Y }, () =>
    Array(MAP_SIZE_X).fill(".")
  );
  robots.forEach((robot) => {
    if (map[robot.position.y][robot.position.x] === ".") {
      map[robot.position.y][robot.position.x] = "1";
    } else {
      map[robot.position.y][robot.position.x] =
        parseInt(map[robot.position.y][robot.position.x]) + 1;
    }
  });
  return map;
};

const readFile = (filename: string): Robot[] => {
  let robots: Robot[] = [];
  try {
    const lines = fs
      .readFileSync(filename, "utf8")
      .split("\n")
      .map((line) => line.trim());

    for (let line of lines) {
      const parts = line.split(" ");
      const position = parts[0].split("=")[1].split(",");
      const velocity = parts[1].split("=")[1].split(",");

      const positionCoord: Coordinate = {
        x: parseInt(position[0]),
        y: parseInt(position[1]),
      };
      const velocityCoord: Coordinate = {
        x: parseInt(velocity[0]),
        y: parseInt(velocity[1]),
      };

      const robot: Robot = {
        position: positionCoord,
        velocity: velocityCoord,
      };
      robots.push(robot);
    }
  } catch (err) {
    console.error(err);
  }

  return robots;
};

const mod = (n: number, m: number): number => {
  return ((n % m) + m) % m;
};

const countRobotsInQuadrants = (robotsMap: string[][]): number[] => {
  const robotCountInQuadrants = [0, 0, 0, 0];

  for (let y = 0; y < MAP_SIZE_Y; y++) {
    for (let x = 0; x < MAP_SIZE_X; x++) {
      if (robotsMap[y][x] === ".") continue;

      const count = parseInt(robotsMap[y][x]);

      if (
        x === Math.floor(MAP_SIZE_X / 2) ||
        y === Math.floor(MAP_SIZE_Y / 2)
      ) {
        continue;
      }

      if (x < MAP_SIZE_X / 2 && y < MAP_SIZE_Y / 2) {
        robotCountInQuadrants[0] += count; // Top-left
      } else if (x >= MAP_SIZE_X / 2 && y < MAP_SIZE_Y / 2) {
        robotCountInQuadrants[1] += count; // Top-right
      } else if (x < MAP_SIZE_X / 2 && y >= MAP_SIZE_Y / 2) {
        robotCountInQuadrants[2] += count; // Bottom-left
      } else {
        robotCountInQuadrants[3] += count; // Bottom-right
      }
    }
  }

  return robotCountInQuadrants;
};

const printMap = (map: string[][]) => {
  for (let y = 0; y < MAP_SIZE_Y; y++) {
    let line = "";
    for (let x = 0; x < MAP_SIZE_X; x++) {
      line += map[y][x];
    }
    console.log(line);
  }
};

main();
