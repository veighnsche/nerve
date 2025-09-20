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
struct CliWorld {
    binary: Option<PathBuf>,
    last_status: Option<i32>,
    stdout: String,
    stderr: String,
}

impl CliWorld {
    fn ensure_binary(&self) -> PathBuf {
        self.binary
            .clone()
            .expect("CLI binary was not initialised by the given step")
    }
}

#[given("the nrv CLI binary is available")]
async fn cli_binary_available(world: &mut CliWorld) {
    if let Ok(path) = std::env::var("CARGO_BIN_EXE_nrv") {
        world.binary = Some(PathBuf::from(path));
        return;
    }

    let workspace_dir = workspace_dir();
    let target_dir = std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| workspace_dir.join("target"));
    let binary_name = format!("nrv{}", std::env::consts::EXE_SUFFIX);
    let binary_path = target_dir.join("debug").join(binary_name);

    if !binary_path.exists() {
        let status = Command::new("cargo")
            .args(["build", "-p", "nrv", "--bin", "nrv"])
            .current_dir(&workspace_dir)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("failed to build nrv CLI binary");
        assert!(status.success(), "cargo build failed for nrv CLI");
    }

    assert!(
        binary_path.exists(),
        "nrv binary missing at {}",
        binary_path.display()
    );
    world.binary = Some(binary_path);
}

#[when(regex = r#"the CLI is invoked with arguments \"([^\"]*)\""#)]
async fn cli_invoked(world: &mut CliWorld, args: String) {
    let binary = world.ensure_binary();
    let mut cmd = Command::new(binary);
    if !args.trim().is_empty() {
        cmd.args(args.split_whitespace());
    }
    let output = cmd.output().expect("failed to run nrv CLI");
    world.last_status = output.status.code();
    world.stdout = String::from_utf8_lossy(&output.stdout).to_string();
    world.stderr = String::from_utf8_lossy(&output.stderr).to_string();
}

#[then("the CLI responds without crashing")]
async fn cli_responds(world: &mut CliWorld) {
    assert!(
        world.last_status.is_some(),
        "expected CLI to produce an exit code"
    );
}

#[then(regex = r#"the exit code is (\d+)"#)]
async fn exit_code_is(world: &mut CliWorld, expected: i32) {
    let actual = world
        .last_status
        .expect("expected CLI to produce an exit code");
    assert_eq!(actual, expected);
}

#[then(regex = r#"stdout contains \"([^\"]+)\""#)]
async fn stdout_contains(world: &mut CliWorld, expected: String) {
    assert!(
        world.stdout.contains(&expected),
        "stdout `{}` did not contain `{}`",
        world.stdout,
        expected
    );
}

#[then("stdout prints the CLI version")]
async fn stdout_matches_version(world: &mut CliWorld) {
    let expected = format!("nrv {}\n", env!("CARGO_PKG_VERSION"));
    assert_eq!(world.stdout, expected);
}

#[then(regex = r#"stderr mentions \"([^\"]+)\""#)]
async fn stderr_mentions(world: &mut CliWorld, fragment: String) {
    assert!(
        world.stderr.contains(&fragment),
        "stderr `{}` did not contain `{}`",
        world.stderr,
        fragment
    );
}

#[tokio::test]
async fn run_bdd() {
    if std::env::var_os(BDD_ENV_FLAG).is_none() {
        eprintln!(
            "skipping nrv CLI BDD scaffolding; set {BDD_ENV_FLAG}=1 to execute the feature suite"
        );
        return;
    }

    let features = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/features");

    CliWorld::cucumber()
        .with_writer(Basic::stdout())
        .run(features)
        .await;
}
