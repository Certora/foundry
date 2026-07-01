//! Tests for the `FOUNDRY_FFI` environment variable, which authoritatively controls the FFI
//! cheatcode, overriding both `foundry.toml` (`ffi = ...`) and the `--ffi` flag.

const FFI_ECHO_CONTRACT: &str = r#"
import "./test.sol";
interface Vm {
    function ffi(string[] calldata commandInput) external returns (bytes memory);
}

contract FfiEnvTest is DSTest {
    Vm constant vm = Vm(HEVM_ADDRESS);

    function testFfiEcho() external {
        string[] memory inputs = new string[](3);
        inputs[0] = "echo";
        inputs[1] = "-n";
        inputs[2] = "gm";
        bytes memory res = vm.ffi(inputs);
        require(keccak256(res) == keccak256(bytes("gm")), "ffi output mismatch");
    }
}
"#;

// `FOUNDRY_FFI=true` enables FFI without the `--ffi` flag (and with `ffi` unset in config).
forgetest!(foundry_ffi_env_enables_without_flag, |prj, cmd| {
    prj.insert_ds_test();
    prj.add_source("FfiEnv.t.sol", FFI_ECHO_CONTRACT);
    cmd.env("FOUNDRY_FFI", "true");
    cmd.args(["test", "--match-test", "testFfiEcho"]).assert_success();
});

// `FOUNDRY_FFI=false` disables FFI even when `--ffi` is passed.
forgetest!(foundry_ffi_env_disables_over_flag, |prj, cmd| {
    prj.insert_ds_test();
    prj.add_source("FfiEnv.t.sol", FFI_ECHO_CONTRACT);
    cmd.env("FOUNDRY_FFI", "false");
    cmd.args(["test", "--ffi", "--match-test", "testFfiEcho"])
        .assert_failure()
        .stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful!

Ran 1 test for src/FfiEnv.t.sol:FfiEnvTest
[FAIL: vm.ffi: FFI is disabled; add the `--ffi` flag to allow tests to call external commands] testFfiEcho() ([GAS])
Suite result: FAILED. 0 passed; 1 failed; 0 skipped; [ELAPSED]

Ran 1 test suite [ELAPSED]: 0 tests passed, 1 failed, 0 skipped (1 total tests)

Failing tests:
Encountered 1 failing test in src/FfiEnv.t.sol:FfiEnvTest
[FAIL: vm.ffi: FFI is disabled; add the `--ffi` flag to allow tests to call external commands] testFfiEcho() ([GAS])

Encountered a total of 1 failing tests, 0 tests succeeded

Tip: Run `forge test --rerun` to retry only the 1 failed test
Tip: Run `forge test --debug --match-test <TEST_NAME>` to inspect one failing test in the debugger

"#]]);
});
