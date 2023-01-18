use container_health_check::HealthCheck;

struct TestData {
    app_name: String,
    log_path: String,
}

impl TestData {
    fn new() -> TestData {
        TestData {
            app_name: String::from("TestApp"),
            log_path: String::from("tests/testLog.log"),
        }
    }
}

#[test]
fn creates_health_check_struct() {
    let test_data = TestData::new();
    let health = HealthCheck::new(test_data.app_name, test_data.log_path);
    assert_eq!(health.app_name(), &String::from("TestApp"));
}

#[test]
fn fails_health_check() {
    let test_data = TestData::new();
    let health = HealthCheck::new(test_data.app_name, test_data.log_path);
    assert_eq!(health.check_application(), false);
}
