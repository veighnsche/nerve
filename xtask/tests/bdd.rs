use cucumber::{given, then, when, writer::Basic, World};
use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
};

fn workspace_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("CARGO_WORKSPACE_DIR") {
        return PathBuf::from(dir);
    }

    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        let manifest = dir.join("Cargo.toml");
        if manifest.exists() {
            if let Ok(contents) = fs::read_to_string(&manifest) {
                if contents.contains("[workspace]") {
                    return dir;
                }
            }
        }
        if !dir.pop() {
            panic!("unable to locate workspace root starting from tests");
        }
    }
}

const BDD_ENV_FLAG: &str = "NRV_RUN_BDD";

#[derive(Debug, Default, World)]
#[world(init = Self::default)]
struct XtaskWorld {
    binary: Option<PathBuf>,
    last_status: Option<i32>,
    output: String,
}

impl XtaskWorld {
    fn ensure_binary(&self) -> PathBuf {
        self.binary
            .clone()
            .expect("xtask binary was not initialised by the given step")
    }
}

#[given("the xtask binary is scaffolded")]
async fn xtask_binary(world: &mut XtaskWorld) {
    if let Ok(path) = std::env::var("CARGO_BIN_EXE_xtask") {
        world.binary = Some(PathBuf::from(path));
        return;
    }

    let workspace_dir = workspace_dir();
    let target_dir = std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| workspace_dir.join("target"));
    let binary_name = format!("xtask{}", std::env::consts::EXE_SUFFIX);
    let binary_path = target_dir.join("debug").join(binary_name);

    if !binary_path.exists() {
        let status = Command::new("cargo")
            .args(["build", "-p", "xtask", "--bin", "xtask"])
            .current_dir(&workspace_dir)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("failed to build xtask binary");
        assert!(status.success(), "cargo build failed for xtask binary");
    }

    assert!(
        binary_path.exists(),
        "xtask binary missing at {}",
        binary_path.display()
    );
    world.binary = Some(binary_path);
}

#[when(regex = r#"xtask runs with arguments \"([^\"]*)\""#)]
async fn xtask_runs(world: &mut XtaskWorld, args: String) {
    let binary = world.ensure_binary();
    let mut cmd = Command::new(binary);
    if !args.trim().is_empty() {
        cmd.args(args.split_whitespace());
    }
    let output = cmd.output().expect("failed to run xtask");
    world.last_status = output.status.code();
    world.output = String::from_utf8_lossy(&output.stdout).to_string();
}

#[then("xtask emits scaffolding guidance")]
async fn xtask_emits(world: &mut XtaskWorld) {
    assert!(
        world.output.contains("placeholder"),
        "expected xtask placeholder output, got `{}`",
        world.output
    );
}

#[then("xtask exits cleanly")]
async fn xtask_exit(world: &mut XtaskWorld) {
    let code = world.last_status.expect("missing xtask exit code");
    assert_eq!(code, 0);
}

#[tokio::test]
async fn run_bdd() {
    if std::env::var_os(BDD_ENV_FLAG).is_none() {
        eprintln!(
            "skipping xtask BDD scaffolding; set {BDD_ENV_FLAG}=1 to execute the feature suite"
        );
        return;
    }

    let features = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/features");

    XtaskWorld::cucumber()
        .with_writer(Basic::stdout())
        .run(features)
        .await;
}
