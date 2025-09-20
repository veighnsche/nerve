#![allow(clippy::module_name_repetitions)]

use cucumber::{given, then, when, writer::Basic, World};
use std::{collections::BTreeMap, path::PathBuf};

const BDD_ENV_FLAG: &str = "NRV_RUN_BDD";

#[derive(Debug, Default, World)]
#[world(init = Self::default)]
struct NrvRsWorld {
    modules: BTreeMap<&'static str, &'static str>,
    apply_result: Option<Result<nrv_rs::apply::ApplyOutcome, nrv_rs::apply::ApplyError>>,
}

#[given("the nrv-rs primitive scaffold is loaded")]
async fn primitive_scaffold_loaded(world: &mut NrvRsWorld) {
    assert!(world.modules.is_empty(), "world should start empty");
}

#[when("a contributor enumerates module guardrails")]
async fn contributor_enumerates_modules(world: &mut NrvRsWorld) {
    let modules: [(&str, fn() -> &'static str); 8] = [
        ("file", nrv_rs::file::module_name),
        ("dir", nrv_rs::dir::module_name),
        ("apply", nrv_rs::apply::module_name),
        ("llm", nrv_rs::llm::module_name),
        ("ctx", nrv_rs::ctx::module_name),
        ("match", nrv_rs::r#match::module_name),
        ("proof", nrv_rs::proof::module_name),
        ("ui", nrv_rs::ui::module_name),
    ];

    for (expected, module_fn) in modules {
        let observed = module_fn();
        world.modules.insert(expected, observed);
    }
}

#[then("each primitive category is tracked for future behavior specs")]
async fn each_category_is_tracked(world: &mut NrvRsWorld) {
    let expected = ["file", "dir", "apply", "llm", "ctx", "match", "proof", "ui"];
    assert_eq!(world.modules.len(), expected.len());

    for category in expected {
        let Some(observed) = world.modules.get(category) else {
            panic!("missing placeholder coverage for {}", category);
        };
        assert_eq!(observed, &category);
    }
}

#[then("the orchestrator bindings remain a separate concern")]
async fn orchestrator_remains_separate(world: &mut NrvRsWorld) {
    assert!(world.modules.get("orch").is_none());
}

#[given("the apply diff scaffold is loaded")]
async fn apply_diff_scaffold(world: &mut NrvRsWorld) {
    assert!(world.apply_result.is_none());
}

#[when(regex = r#"apply\.diff is invoked against \"([^\"]+)\" with a unified diff"#)]
async fn apply_diff_invoked(world: &mut NrvRsWorld, file: String) {
    let diff = format!("--- {0}\n+++ {0}\n@@ -0,0 +1,2 @@\n+example\n+diff\n", file);
    let options = nrv_rs::apply::ApplyOptions {
        path: PathBuf::from(&file),
        diff,
        strategy: nrv_rs::apply::ApplyStrategy::Write,
        checksum: None,
    };
    let result = nrv_rs::apply::diff(options);
    world.apply_result = Some(result);
}

#[then("the diff operation reports an unimplemented placeholder")]
async fn diff_reports_unimplemented(world: &mut NrvRsWorld) {
    let Some(result) = world.apply_result.take() else {
        panic!("apply.diff was never invoked in this scenario");
    };
    match result {
        Ok(outcome) => panic!("expected an error, got {:?}", outcome),
        Err(nrv_rs::apply::ApplyError::Unimplemented { .. }) => {}
        Err(other) => panic!("unexpected error {:?}", other),
    }
}

#[tokio::test]
async fn run_bdd() {
    if std::env::var_os(BDD_ENV_FLAG).is_none() {
        eprintln!(
            "skipping nrv-rs BDD scaffolding; set {BDD_ENV_FLAG}=1 to execute the feature suite"
        );
        return;
    }

    let features = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/features");

    NrvRsWorld::cucumber()
        .with_writer(Basic::stdout())
        .run(features)
        .await;
}
