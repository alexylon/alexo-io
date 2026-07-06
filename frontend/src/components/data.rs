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
            "Slint",
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
        skills: &["Next.js", "Node.js", "Dioxus"],
    },
    SkillCategory {
        name: "AI & ML",
        skills: &["LLM integration", "MCP", "Vercel AI SDK"],
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
            "Build an AI MCP server in Rust for accessing the company's APIs.",
            "Develop frontend applications for a digital publishing suite — including Publications Editor, InDesign Plugin, Reader, Configurator, Assets Drive, and a cross-platform mobile application — using React, TypeScript, JavaScript, ExtendScript, Redux, and SCSS.",
            "Maintain Java backend APIs with a focus on coding standards, test coverage, and reliability.",
            "Work closely with UI/UX teams to streamline releases and reduce feature turnaround time.",
        ],
    },
    ExperienceEntry {
        title: "Full-Stack Developer",
        company: "DXC Technology",
        period: "Apr 2020 – Aug 2023",
        responsibilities: &[
            "Modernised the Next Gen Product Accelerator by rewriting critical components from Java to Rust.",
            "Contributed to building an LSP server in Rust for the VP/MS modelling language.",
            "Developed VP/MS (Visual Product Modelling System) with React, TypeScript, Material UI, and Java, improving product visualisation and user workflows.",
            "Built a full-stack service with the AWS SDK, React, TypeScript, and Terraform to integrate cloud infrastructure into customer-facing solutions.",
            "Developed features for the BMW Group Vulnerability Tool and optimised PostgreSQL schemas and iBATIS data layers, reducing query times by 90% (from 30 seconds to 3 seconds).",
        ],
    },
    ExperienceEntry {
        title: "Web Developer Intern",
        company: "Camplight",
        period: "Sep 2019 – Apr 2020",
        responsibilities: &[
            "Built a property management web application with React, Next.js, and GraphQL.",
            "Wrote reusable UI components in TypeScript and Material UI.",
            "Collaborated with mentors on architecture reviews and code quality to deliver production-ready components.",
        ],
    },
];

pub struct ProjectEntry<'a> {
    pub name: &'a str,
    /// Short classification shown in the margin apparatus, e.g. "Rust · CLI".
    pub kind: &'a str,
    pub description: &'a str,
    pub url: &'a str,
    pub homepage: Option<&'a str>,
}

pub const PROJECTS: &[ProjectEntry] = &[
    ProjectEntry {
        name: "FerroCrypt",
        kind: "Rust · library, CLI & desktop",
        description: "A pure Rust library, CLI, and desktop application for encrypting and decrypting files and directories with password-based or key-pair encryption.",
        url: "https://github.com/alexylon/ferrocrypt",
        homepage: Some("https://www.ferrocrypt.app"),
    },
    ProjectEntry {
        name: "Sofos Code",
        kind: "Rust · CLI",
        description: "A terminal-based AI coding assistant built in Rust, with Claude and GPT support, file editing, and MCP integration. Published on crates.io.",
        url: "https://github.com/alexylon/sofos-code",
        homepage: None,
    },
    ProjectEntry {
        name: "Sofos Web",
        kind: "TypeScript · web app",
        description: "A multi-model AI chatbot built with React, Next.js, and Material UI, with image analysis and speech-to-text support for OpenAI, Anthropic, and Google models.",
        url: "https://github.com/alexylon/sofos-web",
        homepage: None,
    },
    ProjectEntry {
        name: "clavirio",
        kind: "Rust · terminal app",
        description: "A terminal-based typing tutor built with Rust and ratatui. Practise with built-in lessons or any text file while a virtual keyboard tracks each keystroke.",
        url: "https://github.com/alexylon/clavirio",
        homepage: Some("https://www.clavir.io"),
    },
    ProjectEntry {
        name: "alexo.io",
        kind: "Rust · this site",
        description: "This website — a full-stack Rust application with a Dioxus/WASM frontend and an axum server, hosted on a Raspberry Pi.",
        url: "https://github.com/alexylon/alexo-io",
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
        title: "PhD in Theology",
        institution: "Sofia University “St. Kliment Ohridski”",
    },
    EducationEntry {
        title: "MEng in Engineering",
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
        level: "Native",
    },
    Language {
        name: "English",
        level: "C1",
    },
    Language {
        name: "Italian",
        level: "B2",
    },
    Language {
        name: "Russian",
        level: "B1",
    },
    Language {
        name: "Greek",
        level: "A2",
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
        label: "Resume PDF",
        href: "",
        target: None,
        rel: None,
        download: Some("Resume_Alexander_Alexandrov.pdf"),
    },
];
