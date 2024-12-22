use log::info;

use std::fs;

const TEST_FILE_NAME: &'static str = "temp.lnk";

#[test]
#[cfg(feature="binwrite")]
fn create_read_blank() {
    pretty_env_logger::init();

    for is_unicode in &[false, true] {
        info!("Saving shortcut...");
        let mut shortcut = lnk::ShellLink::default();
        shortcut
            .header_mut()
            .update_link_flags(lnk::LinkFlags::IS_UNICODE, *is_unicode);
        shortcut.set_name(Some("Blank name".to_string()));
        shortcut
            .save(TEST_FILE_NAME)
            .expect("Failed to save shortcut!");

        info!("Reading shortcut...");
        let shortcut = lnk::ShellLink::open(TEST_FILE_NAME, encoding_rs::WINDOWS_1252).unwrap();
        println!("{:#?}", shortcut);
        assert_eq!(
            shortcut.string_data().name_string(),
            &Some("Blank name".to_string())
        );
    }

    info!("Cleaning up...");
    fs::remove_file(TEST_FILE_NAME).expect("delete shortcut");
}
