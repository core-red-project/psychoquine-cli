use crate::domain::error::DomainError;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LanguageFamily {
    Compiled,
    Interpreted,
    Shell,
    Functional,
    Esoteric,
}

impl fmt::Display for LanguageFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LanguageFamily::Compiled => write!(f, "Compiled"),
            LanguageFamily::Interpreted => write!(f, "Interpreted"),
            LanguageFamily::Shell => write!(f, "Shell"),
            LanguageFamily::Functional => write!(f, "Functional"),
            LanguageFamily::Esoteric => write!(f, "Esoteric"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[value(alias = "py")]
    Python,
    #[value(alias = "js", alias = "node")]
    Javascript,
    #[value(alias = "c")]
    C,
    #[value(alias = "cs", alias = "csharp")]
    CSharp,
    #[value(alias = "rs")]
    Rust,
    #[value(alias = "sh")]
    Bash,
    #[value(alias = "pl")]
    Perl,
    #[value(alias = "php")]
    Php,
    #[value(alias = "rb")]
    Ruby,
    #[value(alias = "golang")]
    Go,
    #[value(alias = "lua")]
    Lua,
    #[value(alias = "ml")]
    Ocaml,
    #[value(alias = "pas")]
    Pascal,
    #[value(alias = "scm")]
    Scheme,
    #[value(alias = "lisp", alias = "cl")]
    CommonLisp,
    #[value(alias = "bat", alias = "cmd")]
    DosBatch,
    #[value(alias = "bf")]
    Brainfuck,
    #[value(alias = "hq9")]
    Hq9Plus,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.slug())
    }
}

impl FromStr for Language {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "python" | "py" => Ok(Language::Python),
            "javascript" | "js" | "node" => Ok(Language::Javascript),
            "c" => Ok(Language::C),
            "csharp" | "c#" | "cs" => Ok(Language::CSharp),
            "rust" | "rs" => Ok(Language::Rust),
            "bash" | "sh" => Ok(Language::Bash),
            "perl" | "pl" => Ok(Language::Perl),
            "php" => Ok(Language::Php),
            "ruby" | "rb" => Ok(Language::Ruby),
            "go" | "golang" => Ok(Language::Go),
            "lua" => Ok(Language::Lua),
            "ocaml" | "ml" => Ok(Language::Ocaml),
            "pascal" | "pas" => Ok(Language::Pascal),
            "scheme" | "scm" => Ok(Language::Scheme),
            "commonlisp" | "common-lisp" | "lisp" | "cl" => Ok(Language::CommonLisp),
            "dosbatch" | "batch" | "bat" | "cmd" => Ok(Language::DosBatch),
            "brainfuck" | "bf" => Ok(Language::Brainfuck),
            "hq9plus" | "hq9+" | "hq9" => Ok(Language::Hq9Plus),
            other => Err(DomainError::UnsupportedLanguage(other.to_string())),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct LanguageMetadata {
    pub language: Language,
    pub display_name: &'static str,
    pub family: LanguageFamily,
    pub extension: &'static str,
    pub default_binary: &'static str,
    pub comment_prefix: &'static str,
    pub description: &'static str,
    pub kleene_principle: &'static str,
}

impl Language {
    pub const ALL: &'static [Language] = &[
        Language::Python,
        Language::Javascript,
        Language::C,
        Language::CSharp,
        Language::Rust,
        Language::Bash,
        Language::Perl,
        Language::Php,
        Language::Ruby,
        Language::Go,
        Language::Lua,
        Language::Ocaml,
        Language::Pascal,
        Language::Scheme,
        Language::CommonLisp,
        Language::DosBatch,
        Language::Brainfuck,
        Language::Hq9Plus,
    ];

    pub fn slug(&self) -> &'static str {
        match self {
            Language::Python => "python",
            Language::Javascript => "javascript",
            Language::C => "c",
            Language::CSharp => "csharp",
            Language::Rust => "rust",
            Language::Bash => "bash",
            Language::Perl => "perl",
            Language::Php => "php",
            Language::Ruby => "ruby",
            Language::Go => "go",
            Language::Lua => "lua",
            Language::Ocaml => "ocaml",
            Language::Pascal => "pascal",
            Language::Scheme => "scheme",
            Language::CommonLisp => "commonlisp",
            Language::DosBatch => "dosbatch",
            Language::Brainfuck => "brainfuck",
            Language::Hq9Plus => "hq9plus",
        }
    }

    pub fn metadata(&self) -> LanguageMetadata {
        match self {
            Language::Python => LanguageMetadata {
                language: *self,
                display_name: "Python",
                family: LanguageFamily::Interpreted,
                extension: "py",
                default_binary: "python3",
                comment_prefix: "# ",
                description: "Python 3 reflexive quine using exact string formatting and repr() serialization.",
                kleene_principle: "Part A (Data) is stored in variable s. Part B (Logic) invokes s.format(s), where {!r} safely expands to repr(s) without quote collisions.",
            },
            Language::Javascript => LanguageMetadata {
                language: *self,
                display_name: "JavaScript (Node.js)",
                family: LanguageFamily::Interpreted,
                extension: "js",
                default_binary: "node",
                comment_prefix: "// ",
                description: "JavaScript reflexive quine using console.log %j JSON serialization.",
                kleene_principle: "Part A is a string literal containing format specifier %j. Part B passes s to console.log(s, s) which stringifies the template into itself.",
            },
            Language::C => LanguageMetadata {
                language: *self,
                display_name: "ANSI C",
                family: LanguageFamily::Compiled,
                extension: "c",
                default_binary: "cc",
                comment_prefix: "// ",
                description: "ANSI C canonical quine using printf with ASCII codes 10 (\\n) and 34 (\").",
                kleene_principle: "Bypasses quote-escaping by using ASCII codes 10 (newline) and 34 (quote) with %c specifiers in printf to generate delimiters at runtime.",
            },
            Language::CSharp => LanguageMetadata {
                language: *self,
                display_name: "C# (.NET)",
                family: LanguageFamily::Compiled,
                extension: "cs",
                default_binary: "dotnet",
                comment_prefix: "// ",
                description: "C# canonical quine using Console.Write composite formatting.",
                kleene_principle: "Uses indexed composite formatting {0}{1}{0} with ASCII quote char (char)34 to reconstruct enclosing double quotes around the code block.",
            },
            Language::Rust => LanguageMetadata {
                language: *self,
                display_name: "Rust",
                family: LanguageFamily::Compiled,
                extension: "rs",
                default_binary: "rustc",
                comment_prefix: "// ",
                description: "Rust canonical quine utilizing positional macro formatting with {0:?}.",
                kleene_principle: "The {0:?} debug formatter automatically injects surrounding quotes and escapes characters, enabling clean self-replication in compiled Rust.",
            },
            Language::Bash => LanguageMetadata {
                language: *self,
                display_name: "POSIX Bash",
                family: LanguageFamily::Shell,
                extension: "sh",
                default_binary: "bash",
                comment_prefix: "# ",
                description: "Portable shell quine using octal escape \\47 (single quote) interpolation in printf.",
                kleene_principle: "Octal sequence \\47 emits a single quote without colliding with the shell quote tokenizer, allowing the string variable to print itself cleanly.",
            },
            Language::Perl => LanguageMetadata {
                language: *self,
                display_name: "Perl",
                family: LanguageFamily::Interpreted,
                extension: "pl",
                default_binary: "perl",
                comment_prefix: "# ",
                description: "Canonical Perl quine using q() quoting operator and eval execution.",
                kleene_principle: "Perl's q() construct acts as custom literal quote delimiters, allowing $_ to hold the code body and print itself using eval.",
            },
            Language::Php => LanguageMetadata {
                language: *self,
                display_name: "PHP",
                family: LanguageFamily::Interpreted,
                extension: "php",
                default_binary: "php",
                comment_prefix: "// ",
                description: "PHP CLI quine with printf and ASCII character 39 (single quote).",
                kleene_principle: "Injects ASCII 39 through %c to wrap the data variable within single quotes during printf execution.",
            },
            Language::Ruby => LanguageMetadata {
                language: *self,
                display_name: "Ruby",
                family: LanguageFamily::Interpreted,
                extension: "rb",
                default_binary: "ruby",
                comment_prefix: "# ",
                description: "Classic 1-line Ruby quine using eval and Kernel#p inspection.",
                kleene_principle: "Kernel#p prints the inspected (quoted and escaped) string representation, while print outputs the raw code prefix.",
            },
            Language::Go => LanguageMetadata {
                language: *self,
                display_name: "Go",
                family: LanguageFamily::Compiled,
                extension: "go",
                default_binary: "go",
                comment_prefix: "// ",
                description: "Canonical Go quine using fmt.Printf with %q string quote specifier.",
                kleene_principle: "fmt.Printf %q prints a double-quoted string safely escaped, matching the Go compiler's string literal expectations.",
            },
            Language::Lua => LanguageMetadata {
                language: *self,
                display_name: "Lua",
                family: LanguageFamily::Interpreted,
                extension: "lua",
                default_binary: "lua",
                comment_prefix: "-- ",
                description: "Lua quine using string.format with %q specifier.",
                kleene_principle: "Lua string.format %q formats strings with enclosing quotes and escapes, perfectly mirroring the input string.",
            },
            Language::Ocaml => LanguageMetadata {
                language: *self,
                display_name: "OCaml",
                family: LanguageFamily::Functional,
                extension: "ml",
                default_binary: "ocaml",
                comment_prefix: "(* ",
                description: "Canonical functional OCaml quine using higher-order function and %S.",
                kleene_principle: "Printf %S formats strings with surrounding quotes, allowing a self-applied lambda to print both its parameter and its source.",
            },
            Language::Pascal => LanguageMetadata {
                language: *self,
                display_name: "Pascal (FreePascal)",
                family: LanguageFamily::Compiled,
                extension: "pas",
                default_binary: "fpc",
                comment_prefix: "// ",
                description: "Classic Pascal quine with constant string replacement and #39 character code.",
                kleene_principle: "Pascal represents ASCII characters using #CODE (#39 for single quote), bypassing quote escaping in the print stream.",
            },
            Language::Scheme => LanguageMetadata {
                language: *self,
                display_name: "Scheme (Lisp)",
                family: LanguageFamily::Functional,
                extension: "scm",
                default_binary: "scheme",
                comment_prefix: ";; ",
                description: "Canonical Lisp/Scheme self-evaluating lambda with quote reflection.",
                kleene_principle: "((lambda (x) (list x (list 'quote x))) '(lambda (x) (list x (list 'quote x)))) embodies Kleene's Second Recursion Theorem directly in functional form.",
            },
            Language::CommonLisp => LanguageMetadata {
                language: *self,
                display_name: "Common Lisp",
                family: LanguageFamily::Functional,
                extension: "lisp",
                default_binary: "sbcl",
                comment_prefix: ";; ",
                description: "Common Lisp canonical S-expression quine using format or lambda quote.",
                kleene_principle: "Common Lisp format directive ~S prints a readable (quoted) S-expression, matching code to data.",
            },
            Language::DosBatch => LanguageMetadata {
                language: *self,
                display_name: "DOS / Windows Batch",
                family: LanguageFamily::Shell,
                extension: "bat",
                default_binary: "cmd",
                comment_prefix: ":: ",
                description: "Windows CMD batch quine using variable expansion.",
                kleene_principle: "Batch variables expand dynamically during echo commands to output the script's lines.",
            },
            Language::Brainfuck => LanguageMetadata {
                language: *self,
                display_name: "Brainfuck",
                family: LanguageFamily::Esoteric,
                extension: "bf",
                default_binary: "brainfuck",
                comment_prefix: "",
                description: "Classic Brainfuck self-replicator operating on cell memory tape.",
                kleene_principle: "A memory tape decoder loop reconstructs the instructions by shifting between data storage cells and output loops.",
            },
            Language::Hq9Plus => LanguageMetadata {
                language: *self,
                display_name: "HQ9+",
                family: LanguageFamily::Esoteric,
                extension: "hq9",
                default_binary: "hq9plus",
                comment_prefix: "",
                description: "Trivial esoteric quine language where instruction 'Q' outputs the entire program source code.",
                kleene_principle: "By language specification, the Q command is a built-in primitive quine.",
            },
        }
    }
}
