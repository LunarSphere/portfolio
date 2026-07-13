import type { SiteContent } from "./types";

/**
 * Edit this file to update the portfolio. The TypeScript `satisfies` check
 * catches missing fields and misspelled properties during `npm run typecheck`.
 */
export const site = {
  seo: {
    title: "James Kevius Tribble | Portfolio",
    description:
      "Portfolio website of James Kevius Tribble, presented as an interactive terminal.",
    siteUrl: "https://keviustribble.dev",
  },
  identity: {
    fullName: "James Kevius Tribble",
    shortName: "Kevius Tribble",
    shellUser: "guest@portfolio",
    tagline: "Kevius really likes terminal user interfaces",
  },
  welcome: {
    heading: "James Kevius Tribble",
    introduction: "A cross-platform portfolio rendered as a browser TUI.",
  },
  about: {
    heading: "About",
    paragraphs: [
      "Kevius Tribble is a Computer Science Student @ Clemson University.",
      "Passionate about solving Computer Vision, Cloud, and ML Problems.",
    ],
  },
  resume: {
    heading: "Resume",
    path: "/resume.pdf",
    description: "Open the current resume as a PDF in a new tab.",
    editorHint: "Replace public/resume.pdf whenever the resume changes.",
  },
  // Keep featured work at the top while preserving the authored order within
  // the featured and non-featured groups.
  projects: [
    {
      title: "Terminal Portfolio",
      slug: "terminal-portfolio",
      blurb:
        "This portfolio: a cross-platform terminal experience built with Expo and React Native.",
      description:
        "A keyboard-first personal site with slash commands, selectable project and social panes, theme toggling, and static deployment support.",
      technologies: ["React Native", "Expo", "TypeScript", "React Native Web"],
      githubUrl: "https://github.com/LunarSphere/portfolio",
      liveUrl: "https://keviustribble.dev/",
      featured: false,
    },
    {
      title: "Arxivist",
      slug: "arxivist",
      blurb: "Toy agentic search engine deployed on AWS.",
      description:
        "I wanted to learn how search engines work under the hood, so I built a mini Perplexity.",
      technologies: ["Rust", "CLI", "Tooling"],
      githubUrl: "https://github.com/LunarSphere/arxivist",
      liveUrl: "https://arxivist-swart.vercel.app/",
      featured: true,
    },
    {
      title: "Policy Document Validator",
      slug: "policy-document-validator",
      blurb: "Won Most Viable at LPL Financials 2026 university hackathon",
      description: "A document processing and validation system that extracts data from documents using AWS Textract and validates them against company and state policies using AWS Bedrock Knowledge Bases",
      technologies: ["OCR", "ReactJS", "AWS"],
      githubUrl: "https://github.com/LunarSphere/team-7",
      featured: true,
    },
    {
      title: "2025 Computer Science Capstone",
      slug: "2025-computer-science-capstone",
      blurb: "The core component I developed for my teams capstone project",
      description: "Implemented the Nightshade research methodology to protect publicly shared images on our social media platform from unauthorized scraping and use. Deployed the solution on an AWS EC@ server and ran a weekly recurring job to continuously poison newly uploaded content.",
      technologies: ["Computer Vision", "Data Poisoning", "Diffusion"],
      githubUrl: "https://github.com/LunarSphere/nightshade-release",
      featured: false,
    },
    {
      title: "Honors Thesis",
      slug: "honors-thesis",
      blurb: "Experiments I'm running to graduate with honors distinction.",
      description: "I'm investigating what causes MLLMs to hallucinate.",
      technologies: ["Computer Vision", "Multimodal LLMs", "Research"],
      githubUrl: "https://github.com/LunarSphere/LLM_Hallucinations",
      featured: false,
    },
  ].sort((a, b) => Number(b.featured) - Number(a.featured)),
  socials: [
    {
      label: "GitHub",
      handle: "@LunarSphere",
      url: "https://github.com/LunarSphere",
      blurb: "Code, experiments, and project history.",
    },
    {
      label: "LinkedIn",
      handle: "Kevius Tribble",
      url: "https://www.linkedin.com/in/jamestribble/",
      blurb: "Professional background and contact path.",
    },
    {
      label: "Substack",
      handle: "Kevius Thinks",
      url: "https://substack.com/@keviusthinks",
      blurb:
        "A tortured poet",
    },
    {
      label: "Email",
      handle: "My Email",
      url: "mailto:jjtribb05@gmail.com",
      blurb: "The slowest way to connect with me.",
    },
  ],
} satisfies SiteContent;

export function primaryProjectUrl(project: SiteContent["projects"][number]) {
  return project.liveUrl || project.githubUrl;
}
