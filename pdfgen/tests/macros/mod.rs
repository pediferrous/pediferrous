#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of_val<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let mut name = type_name_of_val(f).strip_suffix("::f").unwrap_or("");
        while let Some(rest) = name.strip_suffix("::{{closure}}") {
            name = rest;
        }
        name
    }};
}

#[macro_export]
macro_rules! snap_test {
    ($doc:ident) => {{
        let crate_dir = env!("CARGO_MANIFEST_DIR");
        let module_path = module_path!()
            .strip_prefix("r#mod::")
            .unwrap_or(module_path!())
            .replace("::", "/");

        let path = format!("{crate_dir}/tests/snapshots/{module_path}");
        std::fs::create_dir_all(&path).unwrap();

        let function_name = macros::function_name!().split("::").last().unwrap();
        let file_path = format!("{path}/{function_name}.pdf");

        let update_snaps = std::env::var("PDFGEN_UPDATE_SNAPS").is_ok_and(|val| val == "1");

        let mut writer = Vec::default();
        $doc.write(&mut writer).unwrap();

        let doc_content = String::from_utf8_lossy(&writer);

        if PathBuf::from(&file_path).is_file() {
            let buf = std::fs::read(&file_path).unwrap();
            let file_content = String::from_utf8_lossy(&buf);

            if update_snaps {
                let cmp = pretty_assertions::StrComparison::new(&file_content, &doc_content);
                eprintln!("Updating snapshot '{file_path}':\n{cmp}");

                std::fs::write(&file_path, doc_content.as_bytes()).unwrap();
            } else {
                pretty_assertions::assert_str_eq!(file_content, doc_content);
                println!("To update snapshots, run tests again with 'cargo bless'")
            }
        } else {
            let mut file = std::fs::OpenOptions::new()
                .write(true) // read-write file
                .read(true)
                .create(true) // create if not existing
                .truncate(true) // overwrite completely
                .open(&file_path) // at this path
                .unwrap();

            let file_content = {
                let mut buf = Vec::new();
                file.read_to_end(&mut buf).unwrap();
                String::from_utf8_lossy(&buf).into_owned()
            };

            if std::env::var("PDFGEN_UPDATE_SNAPS").is_ok() {
                file.write_all(&writer).unwrap();
            } else {
                std::fs::remove_file(file_path).unwrap();
                pretty_assertions::assert_str_eq!(file_content, doc_content);
            }
        }
    }};
}

pub use {function_name, snap_test};
