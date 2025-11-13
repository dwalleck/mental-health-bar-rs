import { test, expect } from '@playwright/test';

test('home page renders main UI', async ({ page }) => {
  await page.goto('/');
  // Adjust this selector/text to something stable in your actual landing page.
  await expect(page.locator('body')).toBeVisible();
});
