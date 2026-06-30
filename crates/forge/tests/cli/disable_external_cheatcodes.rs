//! Tests for the `FOUNDRY_DISABLE_EXTERNAL_CHEATCODES` environment variable, which forces
//! external cheatcodes (the `Filesystem` and `Environment` groups) to revert.

const EXTERNAL_CHEATS_CONTRACT: &str = r#"
import "./test.sol";
interface Vm {
    function projectRoot() external view returns (string memory);
    function setEnv(string calldata name, string calldata value) external;
    function ffi(string[] calldata commandInput) external returns (bytes memory);
}

contract ExternalCheatsTest is DSTest {
    Vm constant vm = Vm(HEVM_ADDRESS);

    // `projectRoot` is in the `Filesystem` group and needs no fs permissions.
    function testProjectRoot() external view {
        vm.projectRoot();
    }

    // `setEnv` is in the `Environment` group.
    function testSetEnv() external {
        vm.setEnv("FOUNDRY_DISABLE_EXTERNAL_CHEATCODES_PROBE", "1");
    }

    function testFfi() external {
        string[] memory inputs = new string[](2);
        inputs[0] = "echo";
        inputs[1] = "hi";
        vm.ffi(inputs);
    }
}
"#;

// External cheatcodes execute normally when the env var is unset.
forgetest!(external_cheatcodes_allowed_when_env_unset, |prj, cmd| {
    prj.insert_ds_test();
    prj.add_source("ExternalCheats.t.sol", EXTERNAL_CHEATS_CONTRACT);
    cmd.args(["test", "--match-test", "testProjectRoot|testSetEnv"]).assert_success();
});

// Filesystem-group cheatcodes revert when the env var is set.
forgetest!(disable_external_cheatcodes_blocks_filesystem, |prj, cmd| {
    prj.insert_ds_test();
    prj.add_source("ExternalCheats.t.sol", EXTERNAL_CHEATS_CONTRACT);
    cmd.env("FOUNDRY_DISABLE_EXTERNAL_CHEATCODES", "true");
    cmd.args(["test", "--match-test", "testProjectRoot"])
        .assert_failure()
        .stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful!

Ran 1 test for src/ExternalCheats.t.sol:ExternalCheatsTest
[FAIL: external cheatcodes are disabled by `FOUNDRY_DISABLE_EXTERNAL_CHEATCODES`: `vm.projectRoot` accesses the filesystem] testProjectRoot() ([GAS])
Suite result: FAILED. 0 passed; 1 failed; 0 skipped; [ELAPSED]

Ran 1 test suite [ELAPSED]: 0 tests passed, 1 failed, 0 skipped (1 total tests)

Failing tests:
Encountered 1 failing test in src/ExternalCheats.t.sol:ExternalCheatsTest
[FAIL: external cheatcodes are disabled by `FOUNDRY_DISABLE_EXTERNAL_CHEATCODES`: `vm.projectRoot` accesses the filesystem] testProjectRoot() ([GAS])

Encountered a total of 1 failing tests, 0 tests succeeded

Tip: Run `forge test --rerun` to retry only the 1 failed test
Tip: Run `forge test --debug --match-test <TEST_NAME>` to inspect one failing test in the debugger

"#]]);
});

// Environment-group cheatcodes revert when the env var is set.
forgetest!(disable_external_cheatcodes_blocks_environment, |prj, cmd| {
    prj.insert_ds_test();
    prj.add_source("ExternalCheats.t.sol", EXTERNAL_CHEATS_CONTRACT);
    cmd.env("FOUNDRY_DISABLE_EXTERNAL_CHEATCODES", "true");
    cmd.args(["test", "--match-test", "testSetEnv"])
        .assert_failure()
        .stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful!

Ran 1 test for src/ExternalCheats.t.sol:ExternalCheatsTest
[FAIL: external cheatcodes are disabled by `FOUNDRY_DISABLE_EXTERNAL_CHEATCODES`: `vm.setEnv` accesses the environment] testSetEnv() ([GAS])
Suite result: FAILED. 0 passed; 1 failed; 0 skipped; [ELAPSED]

Ran 1 test suite [ELAPSED]: 0 tests passed, 1 failed, 0 skipped (1 total tests)

Failing tests:
Encountered 1 failing test in src/ExternalCheats.t.sol:ExternalCheatsTest
[FAIL: external cheatcodes are disabled by `FOUNDRY_DISABLE_EXTERNAL_CHEATCODES`: `vm.setEnv` accesses the environment] testSetEnv() ([GAS])

Encountered a total of 1 failing tests, 0 tests succeeded

Tip: Run `forge test --rerun` to retry only the 1 failed test
Tip: Run `forge test --debug --match-test <TEST_NAME>` to inspect one failing test in the debugger

"#]]);
});

// The env var overrides `--ffi` (and, by the same mechanism, a `foundry.toml` `ffi = true`).
forgetest!(disable_external_cheatcodes_overrides_ffi_flag, |prj, cmd| {
    prj.insert_ds_test();
    prj.add_source("ExternalCheats.t.sol", EXTERNAL_CHEATS_CONTRACT);
    cmd.env("FOUNDRY_DISABLE_EXTERNAL_CHEATCODES", "true");
    cmd.args(["test", "--ffi", "--match-test", "testFfi"])
        .assert_failure()
        .stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful!

Ran 1 test for src/ExternalCheats.t.sol:ExternalCheatsTest
[FAIL: external cheatcodes are disabled by `FOUNDRY_DISABLE_EXTERNAL_CHEATCODES`: `vm.ffi` accesses the filesystem] testFfi() ([GAS])
Suite result: FAILED. 0 passed; 1 failed; 0 skipped; [ELAPSED]

Ran 1 test suite [ELAPSED]: 0 tests passed, 1 failed, 0 skipped (1 total tests)

Failing tests:
Encountered 1 failing test in src/ExternalCheats.t.sol:ExternalCheatsTest
[FAIL: external cheatcodes are disabled by `FOUNDRY_DISABLE_EXTERNAL_CHEATCODES`: `vm.ffi` accesses the filesystem] testFfi() ([GAS])

Encountered a total of 1 failing tests, 0 tests succeeded

Tip: Run `forge test --rerun` to retry only the 1 failed test
Tip: Run `forge test --debug --match-test <TEST_NAME>` to inspect one failing test in the debugger

"#]]);
});
