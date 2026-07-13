import assert from "node:assert/strict";
import test from "node:test";

import { panelPath, parseCommand } from "../src/core/command.ts";

test("parses canonical commands", () => {
  assert.equal(parseCommand("/help").id, "help");
  assert.equal(parseCommand("/projects").id, "projects");
  assert.equal(parseCommand("/toggle").id, "toggle");
});

test("tolerates whitespace, casing, and a missing slash", () => {
  assert.equal(parseCommand("  socials  ").id, "socials");
  assert.equal(parseCommand("ABOUT now").id, "about");
  assert.equal(parseCommand("").id, "noop");
});

test("preserves unknown command text", () => {
  assert.deepEqual(parseCommand("  /wat is this  "), {
    id: "unknown",
    raw: "/wat is this",
  });
});

test("maps panels to shareable static paths", () => {
  assert.equal(panelPath("welcome"), "/");
  assert.equal(panelPath("resume"), "/resume");
});
