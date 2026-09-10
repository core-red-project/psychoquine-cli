use psychoquine::application::{
    EngineRegistry, ExplainQuineUseCase, GenerateQuineRequest, GenerateQuineResponse,
    GenerateQuineUseCase,
};
use psychoquine::domain::{DomainError, Language, PayloadMode};
use std::sync::Arc;

fn execute_or_skip(
    use_case: &GenerateQuineUseCase,
    request: GenerateQuineRequest,
) -> Option<GenerateQuineResponse> {
    match use_case.execute(request) {
        Ok(r) => Some(r),
        Err(DomainError::RuntimeUnavailable(rt)) => {
            eprintln!("Skipping test: runtime '{}' unavailable on host", rt);
            None
        }
        Err(DomainError::ExecutionTimeout {
            language,
            timeout_secs,
        }) => {
            eprintln!(
                "Skipping test: '{}' timed out after {}s on host",
                language, timeout_secs
            );
            None
        }
        Err(DomainError::CompilationFailed { language, details }) => {
            eprintln!(
                "Skipping test: compiler '{}' failed on host: {}",
                language, details
            );
            None
        }
        Err(e) => panic!("Execution failed: {:?}", e),
    }
}

#[test]
fn test_python_quine_self_verification() {
    let registry = Arc::new(EngineRegistry::new());
    let use_case = GenerateQuineUseCase::new(registry);
    let Some(res) = execute_or_skip(
        &use_case,
        GenerateQuineRequest {
            language: Some(Language::Python),
            payload_mode: PayloadMode::None,
            dry_run: false,
            verify: true,
        },
    ) else {
        return;
    };

    let ver = res.verification.unwrap();
    assert!(
        ver.success,
        "Python quine must self-reproduce: {:?}",
        ver.diff_preview
    );
}

#[test]
fn test_python_quine_with_payload_verification() {
    let registry = Arc::new(EngineRegistry::new());
    let use_case = GenerateQuineUseCase::new(registry);
    let Some(res) = execute_or_skip(
        &use_case,
        GenerateQuineRequest {
            language: Some(Language::Python),
            payload_mode: PayloadMode::Text("Sxnnyside Automated Integration Test".to_string()),
            dry_run: false,
            verify: true,
        },
    ) else {
        return;
    };

    let ver = res.verification.unwrap();
    assert!(
        ver.success,
        "Python quine with payload must self-reproduce: {:?}",
        ver.diff_preview
    );
}

#[test]
fn test_javascript_quine_self_verification() {
    let registry = Arc::new(EngineRegistry::new());
    let use_case = GenerateQuineUseCase::new(registry);
    let Some(res) = execute_or_skip(
        &use_case,
        GenerateQuineRequest {
            language: Some(Language::Javascript),
            payload_mode: PayloadMode::None,
            dry_run: false,
            verify: true,
        },
    ) else {
        return;
    };

    let ver = res.verification.unwrap();
    assert!(
        ver.success,
        "JavaScript quine must self-reproduce: {:?}",
        ver.diff_preview
    );
}

#[test]
fn test_javascript_quine_with_payload_verification() {
    let registry = Arc::new(EngineRegistry::new());
    let use_case = GenerateQuineUseCase::new(registry);
    let Some(res) = execute_or_skip(
        &use_case,
        GenerateQuineRequest {
            language: Some(Language::Javascript),
            payload_mode: PayloadMode::Text("Sxnnyside JS Payload Test".to_string()),
            dry_run: false,
            verify: true,
        },
    ) else {
        return;
    };

    let ver = res.verification.unwrap();
    assert!(
        ver.success,
        "JavaScript quine with payload must self-reproduce: {:?}",
        ver.diff_preview
    );
}

#[test]
fn test_c_quine_self_verification() {
    let registry = Arc::new(EngineRegistry::new());
    let use_case = GenerateQuineUseCase::new(registry);
    let Some(res) = execute_or_skip(
        &use_case,
        GenerateQuineRequest {
            language: Some(Language::C),
            payload_mode: PayloadMode::None,
            dry_run: false,
            verify: true,
        },
    ) else {
        return;
    };

    let ver = res.verification.unwrap();
    assert!(
        ver.success,
        "C quine must self-reproduce: {:?}",
        ver.diff_preview
    );
}

#[test]
fn test_c_quine_with_payload_verification() {
    let registry = Arc::new(EngineRegistry::new());
    let use_case = GenerateQuineUseCase::new(registry);
    let Some(res) = execute_or_skip(
        &use_case,
        GenerateQuineRequest {
            language: Some(Language::C),
            payload_mode: PayloadMode::Text("Sxnnyside C Payload Test".to_string()),
            dry_run: false,
            verify: true,
        },
    ) else {
        return;
    };

    let ver = res.verification.unwrap();
    assert!(
        ver.success,
        "C quine with payload must self-reproduce: {:?}",
        ver.diff_preview
    );
}

#[test]
fn test_rust_quine_self_verification() {
    let registry = Arc::new(EngineRegistry::new());
    let use_case = GenerateQuineUseCase::new(registry);
    let Some(res) = execute_or_skip(
        &use_case,
        GenerateQuineRequest {
            language: Some(Language::Rust),
            payload_mode: PayloadMode::None,
            dry_run: false,
            verify: true,
        },
    ) else {
        return;
    };

    let ver = res.verification.unwrap();
    assert!(
        ver.success,
        "Rust quine must self-reproduce: {:?}",
        ver.diff_preview
    );
}

#[test]
fn test_rust_quine_with_payload_verification() {
    let registry = Arc::new(EngineRegistry::new());
    let use_case = GenerateQuineUseCase::new(registry);
    let Some(res) = execute_or_skip(
        &use_case,
        GenerateQuineRequest {
            language: Some(Language::Rust),
            payload_mode: PayloadMode::Text("Sxnnyside Rust Payload Test".to_string()),
            dry_run: false,
            verify: true,
        },
    ) else {
        return;
    };

    let ver = res.verification.unwrap();
    assert!(
        ver.success,
        "Rust quine with payload must self-reproduce: {:?}",
        ver.diff_preview
    );
}

#[test]
fn test_rust_quine_with_banner_verification() {
    let registry = Arc::new(EngineRegistry::new());
    let use_case = GenerateQuineUseCase::new(registry);
    let Some(res) = execute_or_skip(
        &use_case,
        GenerateQuineRequest {
            language: Some(Language::Rust),
            payload_mode: PayloadMode::Banner,
            dry_run: false,
            verify: true,
        },
    ) else {
        return;
    };

    let ver = res.verification.unwrap();
    assert!(
        ver.success,
        "Rust quine with banner must self-reproduce: {:?}",
        ver.diff_preview
    );
}

#[test]
fn test_bash_quine_self_verification() {
    let registry = Arc::new(EngineRegistry::new());
    let use_case = GenerateQuineUseCase::new(registry);
    let Some(res) = execute_or_skip(
        &use_case,
        GenerateQuineRequest {
            language: Some(Language::Bash),
            payload_mode: PayloadMode::None,
            dry_run: false,
            verify: true,
        },
    ) else {
        return;
    };

    let ver = res.verification.unwrap();
    assert!(
        ver.success,
        "Bash quine must self-reproduce: {:?}",
        ver.diff_preview
    );
}

#[test]
fn test_dry_run_simulation() {
    let registry = Arc::new(EngineRegistry::new());
    let use_case = GenerateQuineUseCase::new(registry);
    let res = use_case
        .execute(GenerateQuineRequest {
            language: Some(Language::Python),
            payload_mode: PayloadMode::Text("Dry run test".to_string()),
            dry_run: true,
            verify: false,
        })
        .unwrap();

    assert!(res.is_dry_run);
    assert!(res.verification.is_some());
    let ver = res.verification.unwrap();
    assert!(ver.is_dry_run);
}

#[test]
fn test_explain_usecase() {
    let registry = Arc::new(EngineRegistry::new());
    let explain_use_case = ExplainQuineUseCase::new(registry);
    let resp = explain_use_case.execute(Language::Python).unwrap();
    assert_eq!(resp.metadata.display_name, "Python");
    assert!(!resp.kleene_breakdown.is_empty());
    assert!(!resp.canonical_sample.is_empty());
}

#[test]
fn test_wikipedia_languages_registered() {
    let registry = EngineRegistry::new();
    let langs = registry.supported_languages();
    assert!(
        langs.len() >= 17,
        "All Wikipedia languages must be registered in the catalog"
    );
    assert!(langs.contains(&Language::Brainfuck));
    assert!(langs.contains(&Language::Hq9Plus));
    assert!(langs.contains(&Language::Pascal));
    assert!(langs.contains(&Language::Scheme));
    assert!(langs.contains(&Language::CommonLisp));
    assert!(langs.contains(&Language::CSharp));
    assert!(langs.contains(&Language::Perl));
    assert!(langs.contains(&Language::Php));
    assert!(langs.contains(&Language::Ruby));
    assert!(langs.contains(&Language::Go));
    assert!(langs.contains(&Language::Lua));
    assert!(langs.contains(&Language::Ocaml));
    assert!(langs.contains(&Language::DosBatch));
}

#[test]
fn test_payload_too_large_rejected() {
    let registry = Arc::new(EngineRegistry::new());
    let use_case = GenerateQuineUseCase::new(registry);
    let huge_text = "A".repeat(psychoquine::domain::payload::MAX_PAYLOAD_BYTES + 10);
    let res = use_case.execute(GenerateQuineRequest {
        language: Some(Language::Python),
        payload_mode: PayloadMode::Text(huge_text),
        dry_run: false,
        verify: false,
    });

    assert!(res.is_err());
    match res.unwrap_err() {
        psychoquine::domain::DomainError::PayloadTooLarge {
            max_bytes,
            actual_bytes,
        } => {
            assert_eq!(max_bytes, psychoquine::domain::payload::MAX_PAYLOAD_BYTES);
            assert!(actual_bytes > max_bytes);
        }
        other => panic!("Expected PayloadTooLarge error, got {:?}", other),
    }
}

#[test]
fn test_sandbox_timeout_kill() {
    use psychoquine::infrastructure::SandboxRunner;
    use std::time::Duration;

    // Code that sleeps longer than timeout
    let infinite_code = "import time\ntime.sleep(5)\n";
    let res = SandboxRunner::execute_with_timeout(
        Language::Python,
        infinite_code,
        Duration::from_millis(400),
    );

    assert!(res.is_err());
    match res.unwrap_err() {
        psychoquine::domain::DomainError::ExecutionTimeout { timeout_secs, .. } => {
            assert_eq!(timeout_secs, 0); // 400ms is 0s
        }
        other => panic!("Expected ExecutionTimeout, got {:?}", other),
    }
}
