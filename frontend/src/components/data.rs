pub struct ProfileInfo<'a> {
    pub name: &'a str,
    pub title: &'a str,
}

pub const PROFILE: ProfileInfo = ProfileInfo {
    name: "Alexander Alexandrov",
    title: "Senior Full-Stack Developer",
};

pub struct SkillCategory<'a> {
    pub name: &'a str,
    pub skills: &'a [&'a str],
}

pub const SKILL_CATEGORIES: &[SkillCategory] = &[
    SkillCategory {
        name: "Languages",
        skills: &["Rust", "TypeScript", "JavaScript", "Java", "ExtendScript"],
    },
    SkillCategory {
        name: "Frontend",
        skills: &[
            "React",
            "Dioxus",
            "Redux",
            "Material UI",
            "HTML",
            "CSS",
            "WebAssembly",
        ],
    },
    SkillCategory {
        name: "Backend",
        skills: &["Spring MVC", "PostgreSQL", "iBATIS"],
    },
    SkillCategory {
        name: "Full-Stack",
        skills: &["Next.js", "Node.js"],
    },
    SkillCategory {
        name: "AI / ML",
        skills: &["LLM Integration", "MCP", "Vercel AI SDK"],
    },
    SkillCategory {
        name: "DevOps",
        skills: &["AWS", "Terraform", "Docker", "Vercel", "Git"],
    },
];

pub struct ExperienceEntry<'a> {
    pub title: &'a str,
    pub company: &'a str,
    pub period: &'a str,
    pub responsibilities: &'a [&'a str],
}

pub const EXPERIENCE_ENTRIES: &[ExperienceEntry] = &[
    ExperienceEntry {
        title: "Senior Full-Stack Web Developer",
        company: "Proxiad",
        period: "Aug 2023 – Present",
        responsibilities: &[
            "Develop front-end applications for a digital publishing suite — Publications Editor, InDesign Plugin, Reader, Configurator, Assets Drive, and a cross-platform mobile app — using React, TypeScript, JavaScript, ExtendScript, Redux, and SCSS.",
            "Contribute to Java back-end APIs, maintaining coding standards and test coverage.",
            "Collaborate with UI/UX teams to streamline releases and improve feature turnaround time.",
        ],
    },
    ExperienceEntry {
        title: "Full-Stack Developer",
        company: "DXC Technology",
        period: "Apr 2020 - Aug 2023",
        responsibilities: &[
            "Modernized an enterprise solution by migrating critical components from Java to Rust.",
            "Developed VP/MS (Visual Product Modeling System) using React, TypeScript, Material UI, and Java, improving product visualization and user workflows.",
            "Built a full-stack service using AWS SDK, React, TypeScript, and Terraform to integrate cloud infrastructure into customer-facing solutions.",
            "Developed features for the BMW Group Vulnerability Tool; optimized PostgreSQL schemas and iBATIS data layers, reducing query time by 90% (from 30s to 3s).",
        ],
    },
    ExperienceEntry {
        title: "Web Developer Intern",
        company: "Camplight",
        period: "Sep 2019 - Apr 2020",
        responsibilities: &[
            "Developed a property management web application using React, Next.js, and GraphQL.",
            "Wrote reusable UI components in TypeScript and Material UI.",
            "Collaborated with mentors on architecture reviews and code quality, delivering production-ready components.",
        ],
    },
];

pub struct ProjectEntry<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub url: &'a str,
    pub homepage: Option<&'a str>,
}

pub const PROJECTS: &[ProjectEntry] = &[
    ProjectEntry {
        name: "FerroCrypt",
        description: "A pure Rust library, CLI, and desktop application for encrypting and decrypting files and directories using password-based or key-pair-based encryption.",
        url: "https://github.com/alexylon/ferrocrypt",
        homepage: Some("https://www.ferrocrypt.app"),
    },
    ProjectEntry {
        name: "Sofos Code",
        description: "AI coding assistant for the terminal built in pure Rust, with Claude/GPT support, file editing, and MCP integration. Published on crates.io.",
        url: "https://github.com/alexylon/sofos-code",
        homepage: None,
    },
    ProjectEntry {
        name: "Sofos Web",
        description: "Multi-model AI chatbot built with React, Next.js, and Material UI, with image analysis and speech-to-text. Supports OpenAI, Anthropic, and Google.",
        url: "https://github.com/alexylon/sofos-web",
        homepage: None,
    },
    ProjectEntry {
        name: "clavirio",
        description: "A terminal typing tutor built with Rust and ratatui. Practice with built-in lessons or any text file while a virtual keyboard tracks your keystrokes in real time.",
        url: "https://github.com/alexylon/clavirio",
        homepage: Some("https://www.clavir.io"),
    },
    ProjectEntry {
        name: "alexo.io",
        description: "This website — a full-stack Rust app with a Dioxus/WASM frontend and an axum server. Hosted on a Raspberry Pi.",
        url: "https://github.com/alexylon/alexo.io",
        homepage: None,
    },
];

pub struct EducationEntry<'a> {
    pub title: &'a str,
    pub institution: &'a str,
}

pub const EDUCATION: &[EducationEntry] = &[
    EducationEntry {
        title: "React & JavaScript",
        institution: "Camplight Academy",
    },
    EducationEntry {
        title: "Java Fundamentals",
        institution: "MaxPlus",
    },
    EducationEntry {
        title: "PhD, Theology",
        institution: "Sofia University “St. Kliment Ohridski”",
    },
    EducationEntry {
        title: "M.Eng., Engineering",
        institution: "University of Forestry",
    },
    EducationEntry {
        title: "Mathematics",
        institution: "High School of Mathematics",
    },
];

pub struct CertificationEntry<'a> {
    pub url: &'a str,
    pub title: &'a str,
    pub meta: &'a str,
}

pub const CERTIFICATIONS: &[CertificationEntry] = &[CertificationEntry {
    url: "https://www.credly.com/badges/13918dd1-e5ad-4e81-96c6-95fcb6fb8b3c",
    title: "Oracle Certified Associate, Java SE 8 Programmer",
    meta: "Jan 2019",
}];

pub struct Language<'a> {
    pub name: &'a str,
    pub level: &'a str,
}

pub const LANGUAGES: &[Language] = &[
    Language {
        name: "Bulgarian",
        level: "(Native)",
    },
    Language {
        name: "English",
        level: "(C1)",
    },
    Language {
        name: "Italian",
        level: "(B2)",
    },
    Language {
        name: "Russian",
        level: "(B1)",
    },
    Language {
        name: "Greek",
        level: "(A2)",
    },
];

pub struct ContactLink<'a> {
    pub label: &'a str,
    pub href: &'a str,
    pub target: Option<&'a str>,
    pub rel: Option<&'a str>,
    pub download: Option<&'a str>,
}

pub const CONTACT_LINKS: &[ContactLink] = &[
    ContactLink {
        label: "Email",
        href: "mailto:hi@alexo.io",
        target: None,
        rel: None,
        download: None,
    },
    ContactLink {
        label: "LinkedIn",
        href: "https://www.linkedin.com/in/alexandrovalexander/",
        target: Some("_blank"),
        rel: Some("noopener noreferrer"),
        download: None,
    },
    ContactLink {
        label: "GitHub",
        href: "https://github.com/alexylon",
        target: Some("_blank"),
        rel: Some("noopener noreferrer"),
        download: None,
    },
    ContactLink {
        label: "Mastodon",
        href: "https://fosstodon.org/@lexer",
        target: Some("_blank"),
        rel: Some("noopener noreferrer"),
        download: None,
    },
    ContactLink {
        label: "Resume \u{2913}\u{FE0E}",
        href: "",
        target: None,
        rel: None,
        download: Some("Resume_Alexander_Alexandrov.pdf"),
    },
];
