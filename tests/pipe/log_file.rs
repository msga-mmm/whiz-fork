use std::{fs, path::Path};

#[test]
fn creates_log_files() {
    let test_workdir = Path::new("./tests/pipe");
    let test_temp_folder = test_workdir.join("logs");

    // clean temporal folder for tests
    if test_temp_folder.exists() {
        fs::remove_dir_all(&test_temp_folder).unwrap();
    }

    let mut command =
        rexpect::spawn("cargo run -- -f tests/pipe/log_file.yaml", Some(5_000)).unwrap();
    let output = command.exp_string("Exited(0)").unwrap();
    command.send_control('c').unwrap();

    assert!(
        !output.contains("[server]"),
        "all output should be redirected to log files",
    );

    let expected = vec![
        ("logs/home/GET.log", "[server] GET /home 200\n"),
        ("logs/settings/GET.log", "[server] GET /settings 200\n"),
        ("logs/users/POST.log", "[server] POST /users 500\n"),
        ("logs/users/UPDATE.log", "[server] UPDATE /users 200\n"),
    ];

    for (path, expected_content) in expected {
        let path_file = test_workdir.join(path);
        assert!(path_file.exists(), "should create log file: {path}",);

        let content = fs::read_to_string(path_file).unwrap();
        assert_eq!(
            content, expected_content,
            "wrong content for log file {}",
            path
        );
    }

    // clean all generated temp files
    if test_temp_folder.exists() {
        fs::remove_dir_all(&test_temp_folder).unwrap();
    }
}
