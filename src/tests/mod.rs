use deboa::TestResult;

use crate::FlashHookConfig;

#[test]
fn test_static_files_config() -> TestResult<()> {
    let static_files_config = FlashHookConfig::builder()
        .response("Welcome to VeTiS!")
        .status_code(200)
        .build()?;
    assert_eq!(static_files_config.response(), &Some("Welcome to VeTiS!".into()));
    assert_eq!(static_files_config.status_code(), 200);
    Ok(())
}
