use cucumber::{given, then, when, writer::Basic, World};
use nrv_ui::{step, StepEvent};
use std::path::PathBuf;

const BDD_ENV_FLAG: &str = "NRV_RUN_BDD";

#[derive(Debug, Default, World)]
#[world(init = Self::default)]
struct UiWorld {
    module_name: Option<&'static str>,
    step: Option<nrv_ui::Step>,
}

impl UiWorld {
    fn ensure_step(&self) -> nrv_ui::Step {
        self.step
            .clone()
            .expect("step has not been created in this scenario")
    }

    fn replace_step(&mut self, new_step: nrv_ui::Step) {
        self.step = Some(new_step);
    }
}

#[given("the ui module sentinel is registered")]
async fn module_sentinel(world: &mut UiWorld) {
    world.module_name = Some(nrv_ui::module_name());
}

#[when(regex = r#"a narration step is created with label \"([^\"]+)\""#)]
async fn narration_step_created(world: &mut UiWorld, label: String) {
    world.step = Some(step(label));
}

#[when(regex = r#"the narration step records info \"([^\"]+)\""#)]
async fn narration_step_info(world: &mut UiWorld, msg: String) {
    let step = world.ensure_step().info(msg);
    world.replace_step(step);
}

#[when(regex = r#"the narration step records ok \"([^\"]*)\""#)]
async fn narration_step_ok(world: &mut UiWorld, msg: String) {
    let step = world.ensure_step().ok(Some(msg));
    world.replace_step(step);
}

#[when(regex = r#"the narration step records fail \"([^\"]*)\""#)]
async fn narration_step_fail(world: &mut UiWorld, msg: String) {
    let step = world.ensure_step().fail(Some(msg));
    world.replace_step(step);
}

#[then(regex = r#"the module name reports \"([^\"]+)\""#)]
async fn module_name_reports(world: &mut UiWorld, expected: String) {
    let actual = world.module_name.expect("module sentinel not registered");
    assert_eq!(actual, expected);
}

#[then(regex = r#"the narration events include info \"([^\"]+)\""#)]
async fn events_include_info(world: &mut UiWorld, expected: String) {
    let step = world.ensure_step();
    assert!(
        step.events().iter().any(|event| matches!(
            event,
            StepEvent::Info(msg) if msg == &expected
        )),
        "expected info event `{expected}` not found"
    );
}

#[then(regex = r#"the narration events include ok \"([^\"]*)\""#)]
async fn events_include_ok(world: &mut UiWorld, expected: String) {
    let step = world.ensure_step();
    assert!(
        step.events().iter().any(|event| matches!(
            event,
            StepEvent::Ok(Some(msg)) if msg == &expected
        )),
        "expected ok event `{expected}` not found"
    );
}

#[then(regex = r#"the narration events include fail \"([^\"]*)\""#)]
async fn events_include_fail(world: &mut UiWorld, expected: String) {
    let step = world.ensure_step();
    assert!(
        step.events().iter().any(|event| matches!(
            event,
            StepEvent::Fail(Some(msg)) if msg == &expected
        )),
        "expected fail event `{expected}` not found"
    );
}

#[tokio::test]
async fn run_bdd() {
    if std::env::var_os(BDD_ENV_FLAG).is_none() {
        eprintln!(
            "skipping nrv-ui BDD scaffolding; set {BDD_ENV_FLAG}=1 to execute the feature suite"
        );
        return;
    }

    let features = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/features");

    UiWorld::cucumber()
        .with_writer(Basic::stdout())
        .run(features)
        .await;
}
