import { expect, test } from "@playwright/test";

test.describe.configure({ mode: "serial" });

test("theme defaults to dark and icon matches", async ({ page }) => {
  await page.goto("http://127.0.0.1:3000/");

  await expect.poll(async () => {
    return page.locator("html").getAttribute("data-theme");
  }).toBe("dark");

  await expect(page.locator("#theme-icon")).toHaveText("☀");
});

test("saved theme restores after refresh", async ({ page }) => {
  await page.goto("http://127.0.0.1:3000/");

  await page.evaluate(() => localStorage.setItem("theme", "light"));
  await page.reload();

  await expect.poll(async () => {
    return page.locator("html").getAttribute("data-theme");
  }).toBe("light");

  await expect(page.locator("#theme-icon")).toHaveText("☾");
});

test("mobile menu toggles and closes on link click", async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto("http://127.0.0.1:3000/");

  const mobileNav = page.locator("#nav-mobile");
  const hamburger = page.locator("#nav-hamburger");

  await hamburger.click();
  await expect(mobileNav).toHaveClass(/open/);

  await page.locator("#nav-mobile a[href=\"#skills\"]").click();
  await expect(mobileNav).not.toHaveClass(/open/);
});

test("active nav changes on scroll", async ({ page }) => {
  await page.goto("http://127.0.0.1:3000/");

  const skillsSection = page.locator("#skills");
  await skillsSection.scrollIntoViewIfNeeded();

  await expect(page.locator('.nav-links a[href="#skills"]')).toHaveClass(/active/);
});
