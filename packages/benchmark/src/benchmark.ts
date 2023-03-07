import { Candidate } from "./candidates";

export function benchmark(samples: number, candidates: Candidate[], getSample: () => [string, string]) {
  console.log(`Running ---> ${samples} iterations <--- `);
  const inputs: [string, string][] = Array(samples).fill(0).map(getSample);

  const executions = candidates.map((candidate) => {
    const results = execute(inputs, candidate.execute);
    console.log(
      `${candidate.name.padEnd(30, " ")} took ${results.time.toLocaleString().padStart(6, " ")} ms. (${getHashPerSec(
        samples,
        results.time,
      )})`,
    );

    return results;
  });

  verifyResults(executions.map((e) => e.results));
  console.log("\r\n");
}

function execute(
  inputs: [string, string][],
  fn: (input: [string, string]) => string,
): { results: string[]; time: number } {
  const start = Date.now();
  const results = inputs.map(fn);

  const time = Date.now() - start;

  return { results, time };
}

function verifyResults(results: string[][]) {
  for (let i = 0; i < results.length; i++) {
    const resultsNumber = results[0].length;

    for (let j = 0; j < resultsNumber; j++) {
      const baseline = results[0][j];
      const actual = results[i][j];
      if (baseline != actual) {
        throw new Error(`Result doesnt match. Baseline: ${baseline}, actual: ${actual}`);
      }
    }
  }
}

function getHashPerSec(samples: number, ms: number): string {
  return `${((samples / ms) * 1000).toLocaleString()} hash/s`;
}
