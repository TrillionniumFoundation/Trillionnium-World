#!/usr/bin/env node
import fs from "node:fs/promises";
import { dirname } from "node:path";
import { chromium } from "../web4-frontend/node_modules/playwright/index.mjs";

const CONTRACT = "trillionnium_world_standalone_browser_parity_shell_v1";

function argValue(name, fallback) {
  const index = process.argv.indexOf(name);
  if (index >= 0 && process.argv[index + 1]) return process.argv[index + 1];
  return fallback;
}

const baseUrl = argValue("--base-url", "http://127.0.0.1:28792").replace(/\/$/, "");
const summaryFile = argValue(
  "--summary-file",
  "acceptance/S3_browser_parity/latest/browser-parity.json",
);
const screenshotFile = argValue(
  "--screenshot-file",
  "acceptance/S3_browser_parity/latest/browser-parity.png",
);
const executablePath =
  process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE || "/usr/bin/google-chrome";

const checks = [];
const pageErrors = [];
const consoleErrors = [];
const requestFailures = [];

function recordCheck(name, ok, details = {}) {
  checks.push({ name, ok, ...details });
}

function requireOk(name, condition, details = {}) {
  recordCheck(name, Boolean(condition), details);
  if (!condition) throw new Error(name + " failed");
}

await fs.mkdir(dirname(summaryFile), { recursive: true });
await fs.mkdir(dirname(screenshotFile), { recursive: true });

let browser;
let status = "standalone_browser_parity_failed";
let shellState = null;
let finalState = null;
let finalNode = "";
let finalActionCount = 0;

try {
  browser = await chromium.launch({
    headless: true,
    executablePath,
    args: ["--no-sandbox", "--disable-dev-shm-usage"],
  });
  const page = await browser.newPage({ viewport: { width: 390, height: 844 } });
  page.on("pageerror", (error) => pageErrors.push(error.message));
  page.on("console", (message) => {
    if (message.type() === "error") consoleErrors.push(message.text());
  });
  page.on("requestfailed", (request) => {
    requestFailures.push({
      url: request.url(),
      failure: request.failure()?.errorText || "unknown",
    });
  });

  await page.goto(baseUrl + "/world/play", { waitUntil: "domcontentloaded" });
  await page.waitForSelector(
    '#world-browser-parity-shell[data-contract="' + CONTRACT + '"]',
  );
  recordCheck("shell_contract_visible", true, { contract: CONTRACT });

  await page.waitForFunction(() => {
    const text = document.querySelector("#player-node")?.textContent || "";
    return text.length > 0 && text !== "loading";
  });
  const initialNode = await page.locator("#player-node").textContent();
  requireOk("initial_world_loaded", initialNode === "mirror-city-square", {
    initialNode,
  });

  await page.click("#move-east");
  await page.waitForFunction(() => {
    return document.querySelector("#player-node")?.textContent === "starter-studio";
  });
  const movedNode = await page.locator("#player-node").textContent();
  requireOk("move_east_updates_browser_and_server_state", movedNode === "starter-studio", {
    movedNode,
  });

  const actionExpectations = [
    ["#train-skill", "skill_training_recorded"],
    ["#attack", "tactics_combat_resolved"],
    ["#offer-task", "task_offer_recorded"],
    ["#complete-task", "task_completion_validated"],
  ];
  for (const [selector, result] of actionExpectations) {
    await page.click(selector);
    await page.waitForFunction((expected) => {
      return document.querySelector("#last-result")?.textContent === expected;
    }, result);
    const actual = await page.locator("#last-result").textContent();
    requireOk("browser_action_" + result, actual === result, { selector, actual });
  }

  finalState = await page.evaluate(async () => {
    const response = await fetch("/world/state", {
      headers: { Accept: "application/json" },
    });
    if (!response.ok) throw new Error("/world/state returned " + response.status);
    return response.json();
  });
  const position = (finalState.positions || []).find(
    (entry) => entry.actor_id === "local-player",
  );
  finalNode = position?.node_id || "";
  requireOk("state_readback_after_browser_flow", finalNode === "starter-studio", {
    finalNode,
  });

  shellState = await page.evaluate(() => window.__trnmWorldBrowserParity);
  finalActionCount = shellState?.actions?.length || 0;
  requireOk("browser_recorded_full_action_sequence", finalActionCount >= 6, {
    finalActionCount,
  });
  requireOk("no_page_errors", pageErrors.length === 0, { pageErrors });
  requireOk("no_request_failures", requestFailures.length === 0, { requestFailures });

  await page.screenshot({ path: screenshotFile, fullPage: true });
  status = "standalone_browser_parity_green";
} catch (error) {
  recordCheck("script_error", false, { error: error.message });
} finally {
  if (browser) await browser.close();
}

const summary = {
  contract_version: "trillionnium_world_standalone_browser_parity_gate_v1",
  status,
  generated_at: new Date().toISOString(),
  source_of_truth: "trnm_world_server_browser_parity_gate",
  base_url: baseUrl,
  shell_contract: CONTRACT,
  browser: {
    engine: "chromium",
    executable_path: executablePath,
    viewport: { width: 390, height: 844 },
  },
  screenshot: screenshotFile,
  checks,
  page_errors: pageErrors,
  console_errors: consoleErrors,
  request_failures: requestFailures,
  final: {
    player_node_id: finalNode,
    action_count: finalActionCount,
    shell_state: shellState,
    state_node_count: finalState?.nodes?.length || 0,
    state_task_count: finalState?.tasks?.length || 0,
  },
};

await fs.writeFile(summaryFile, JSON.stringify(summary, null, 2));

if (status !== "standalone_browser_parity_green") {
  console.error("TRILLIONNIUM_WORLD_BROWSER_PARITY_BLOCKED " + status + " " + summaryFile);
  process.exit(1);
}

console.log("TRILLIONNIUM_WORLD_BROWSER_PARITY_READY " + summaryFile);
