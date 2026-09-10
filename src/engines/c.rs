use crate::domain::error::DomainError;
use crate::domain::language::Language;
use crate::domain::payload::Payload;
use crate::engines::QuineEngine;

pub struct CEngine;

impl QuineEngine for CEngine {
    fn language(&self) -> Language {
        Language::C
    }

    fn generate(&self, payload: Option<&Payload>) -> Result<String, DomainError> {
        let code = match payload {
            None => {
                let tmpl = "#include <stdio.h>%cint main(){char*s=%c%s%c;printf(s,10,34,s,34,10);return 0;}%c";
                format!(
                    "#include <stdio.h>\nint main(){{char*s=\"{}\";printf(s,10,34,s,34,10);return 0;}}\n",
                    tmpl
                )
            }
            Some(p) => {
                let sanitized = p.content.replace("*/", "* /");
                let comment_block = format!("/* {} */", sanitized.trim());
                let tmpl = format!(
                    "{}%c#include <stdio.h>%cint main(){{char*s=%c%s%c;printf(s,10,10,34,s,34,10);return 0;}}%c",
                    comment_block
                );
                format!(
                    "{}\n#include <stdio.h>\nint main(){{char*s=\"{}\";printf(s,10,10,34,s,34,10);return 0;}}\n",
                    comment_block, tmpl
                )
            }
        };

        Ok(code)
    }
}
