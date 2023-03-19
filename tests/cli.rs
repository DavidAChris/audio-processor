use assert_cmd::Command;

fn bin_helper() -> Command {
    Command::cargo_bin("wav-validator").unwrap()
}

#[test]
fn average_wav() {
    // This test covers your typical wav file
    // duration is 1859.8, total file size is 357.1MB
    let file_name = "tests/data/avg-wav-357.wav";
    let mut cmd = bin_helper();
    let assert = cmd.args(["-f", file_name]);
    assert.assert().success();
}

#[test]
fn all_silence() {
    // This test covers a wav file that is all silence
    // Duration is 306.35 total file size is 58.8MB, Entire signature is silence
    let file_name = "tests/data/complete-silence-58.wav";
    let mut cmd = bin_helper();
    let assert = cmd.args(["-f", file_name]);
    assert.assert().success();
}

#[test]
fn large_wav_file() {
    // This test covers an above average wav file size
    // Duration is 7002.3, total file size is 1.34GB, Only 1 second of silence
    let file_name = "tests/data/above-average-1_34GB.wav";
    let mut cmd = bin_helper();
    let assert = cmd.args(["-f", file_name]);
    assert.assert().success();
}
