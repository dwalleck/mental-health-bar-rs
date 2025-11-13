import { test, expect } from './tauri.fixture';

// Minimal example that exercises the same SvelteKit UI via Playwright
// while the real Tauri app is running (see tauri.fixture + launch-tauri).
test('tauri app boots and main UI is reachable', async ({ page }) => {
  // Adjust this path/selector to something stable in your app.
  await page.goto('http://127.0.0.1:5173/');

  await expect(page.locator('body')).toBeVisible();
});
