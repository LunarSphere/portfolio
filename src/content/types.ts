export type PanelId =
  | "welcome"
  | "help"
  | "about"
  | "projects"
  | "socials"
  | "resume";

export interface Project {
  title: string;
  slug: string;
  blurb: string;
  description: string;
  technologies: string[];
  githubUrl?: string;
  liveUrl?: string;
  featured: boolean;
}

export interface SocialLink {
  label: string;
  handle?: string;
  url: string;
  blurb: string;
}

export interface SiteContent {
  seo: {
    title: string;
    description: string;
    siteUrl: string;
  };
  identity: {
    fullName: string;
    shortName: string;
    shellUser: string;
    tagline: string;
  };
  welcome: {
    heading: string;
    introduction: string;
  };
  about: {
    heading: string;
    paragraphs: string[];
  };
  resume: {
    heading: string;
    path: string;
    description: string;
    editorHint: string;
  };
  projects: Project[];
  socials: SocialLink[];
}
