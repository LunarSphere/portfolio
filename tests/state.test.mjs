import assert from "node:assert/strict";
import test from "node:test";

import {
  clampIndex,
  initialPortfolioState,
  MAX_OUTPUT_HISTORY,
  portfolioReducer,
} from "../src/core/state.ts";

test("selection remains in bounds", () => {
  assert.equal(clampIndex(-1, 3), 0);
  assert.equal(clampIndex(9, 3), 2);
  assert.equal(clampIndex(1, 0), 0);
});

test("output history remains bounded", () => {
  let state = initialPortfolioState;
  for (let index = 0; index < MAX_OUTPUT_HISTORY + 4; index += 1) {
    state = portfolioReducer(state, {
      type: "output",
      entry: { command: `/test-${index}`, lines: [String(index)] },
      record: true,
    });
  }
  assert.equal(state.outputHistory.length, MAX_OUTPUT_HISTORY);
  assert.equal(state.commandHistory.length, MAX_OUTPUT_HISTORY + 4);
});

test("clear returns the shell to an empty output state", () => {
  const state = portfolioReducer(initialPortfolioState, { type: "clear" });
  assert.equal(state.commandInput, "");
  assert.deepEqual(state.commandHistory, []);
  assert.deepEqual(state.outputHistory, []);
  assert.equal(state.lastStatus, "cleared | /help");
});
