import assert from "node:assert/strict";
import test from "node:test";

import { primaryProjectUrl, site } from "../src/content/site.ts";

test("site content has publishable identity and entries", () => {
  assert.ok(site.identity.fullName);
  assert.ok(site.projects.length > 0);
  assert.ok(site.socials.length > 0);
  assert.ok(site.socials.every((social) => /^(https?:|mailto:)/u.test(social.url)));
});

test("featured projects are listed first", () => {
  const firstNonFeatured = site.projects.findIndex((project) => !project.featured);

  assert.ok(firstNonFeatured >= 0);
  assert.ok(site.projects.slice(0, firstNonFeatured).every((project) => project.featured));
  assert.ok(site.projects.slice(firstNonFeatured).every((project) => !project.featured));
});

test("project live URL takes precedence over GitHub", () => {
  const project = {
    title: "Test",
    slug: "test",
    blurb: "Test project",
    description: "Test project",
    technologies: [],
    featured: false,
    liveUrl: "https://example.com/live",
    githubUrl: "https://example.com/source",
  };

  assert.equal(primaryProjectUrl(project), project.liveUrl);
  assert.equal(primaryProjectUrl({ ...project, liveUrl: undefined }), project.githubUrl);
  assert.equal(
    primaryProjectUrl({ ...project, liveUrl: undefined, githubUrl: undefined }),
    undefined,
  );
});
