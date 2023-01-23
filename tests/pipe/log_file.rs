use std::fs;

use crate::utils::run_config_file_test;

#[test]
fn creates_log_files() {
    let test = run_config_file_test("tests/pipe/log_file.yaml").unwrap();

    assert!(
        !test.output.contains("[server]"),
        "all output should be redirected to log files",
    );

    let expected = vec![
        ("logs/home/GET.log", "[server] GET /home 200\n"),
        ("logs/settings/GET.log", "[server] GET /settings 200\n"),
        ("logs/users/POST.log", "[server] POST /users 500\n"),
        ("logs/users/UPDATE.log", "[server] UPDATE /users 200\n"),
    ];

    for (path, expected_content) in expected {
        let path_file = test.test_dir.join(path);
        assert!(path_file.exists(), "should create log file: {path}",);

        let content = fs::read_to_string(path_file).unwrap();
        assert_eq!(
            content, expected_content,
            "wrong content for log file {}",
            path
        );
    }
}
