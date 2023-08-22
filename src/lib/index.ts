type Game = {
  target: number;
  inputs: number[];
};

const defaultSmalls = [1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9];
const defaultLarges = [25, 50, 75, 100];

function generateGame(
  numLarges: number,
  smalls: number[] = defaultSmalls,
  larges: number[] = defaultLarges,
  numInputs: number = 6
): Game {
  let target = Math.floor(Math.random() * 900) + 100;
  let inLarges = choose(larges, numLarges);
  inLarges.sort((a, b) => a - b);
  let inSmalls = choose(smalls, numInputs - numLarges);
  inSmalls.sort((a, b) => a - b);
  return {
    target,
    inputs: [...inLarges, ...inSmalls],
  };
}

function choose(from: number[], k: number): number[] {
  let fromCopied = [...from]; // shallow copy would not work for complex types, but works for number[]
  let result = [];
  for (let i = 0; i < k; i++) {
    if (fromCopied.length == 0) break;
    let randomIndex = Math.floor(Math.random() * fromCopied.length);
    result.push(fromCopied[randomIndex]);
    console.log(randomIndex);
    fromCopied.splice(randomIndex, 1);
  }
  return result;
}

export type { Game, defaultLarges, defaultSmalls };
export { generateGame, choose };
