extern crate failure;
extern crate itertools;
extern crate serde;
extern crate serde_json;
extern crate sorted_json;
extern crate regex;

mod function;
mod destination;

//use failure::err_msg;
//use failure::Error;
//use sorted_json::to_json;
//use std::{
//    collections::HashMap,
//    fs::File,
//    io::{Read, Write},
//    process::Command,
//    string::String,
//};
//
//const SNAPSHOT_DIR_ENV: &'static str = "SNAPSHOT_DIR";

//fn scrub(content: &str, scrubs: &HashMap<String, String>) -> String {
//    let mut content = content.to_owned();
//    for (key, val) in scrubs.iter() {
//        content = content.replace(key, val);
//    }
//    content
//}
//
//fn assert_snapshot<T>(id: &str, data: &T, scrubs: &HashMap<String, String>) -> Result<(), Error>
//where
//    T: serde::Serialize,
//{
//    let new_content = serde_json::to_value(data)?;
//    let mut new_content = to_json(&new_content);
//    let mut ext = "json";
//    if let serde_json::Value::String(s) = serde_json::to_value(data)? {
//        new_content = s;
//        ext = "txt";
//    }
//    let new_content = new_content + "\n";
//    let new_content = scrub(&new_content, scrubs);
//    // see if a file exits in test folder(=> next to src), if so we compare the output with that
//    // else we create the file
//    let dst = format!("tests/{}.{}", id.replace("::", "/"), ext);
//    let dst_tmp = format!("tests/{}-test.{}", id.replace("::", "/"), ext);
//    if dst.exists() {
//        let mut existing = File::open(&dst)?;
//        let mut existing_content = String::new();
//        existing.read_to_string(&mut existing_content)?;
//        if new_content != existing_content {
//            let mut new = File::create(&dst_tmp)?;
//            new.write_all(&new_content.into_bytes())?;
//            let process = Command::new("git")
//                .args(&[
//                    "diff",
//                    "--color",
//                    "--no-index",
//                    "--",
//                    dst.to_str().ok_or(error::ErrorKind::PathNotUTF8)?,
//                    dst_tmp.to_str().ok_or(error::ErrorKind::PathNotUTF8)?,
//                ]).output()?;
//            println!("{}", String::from_utf8(process.stdout)?);
//            println!(
//                "\n\nIf the new output is correct, run: \nmv {} {}\n\n",
//                dst_tmp.to_str().ok_or(error::ErrorKind::PathNotUTF8)?,
//                dst.to_str().ok_or(error::ErrorKind::PathNotUTF8)?
//            );
//            Err(error::ErrorKind::TestFailed)?
//        }
//    } else {
//        let parent = dst.parent().ok_or(error::ErrorKind::TestFailed)?;
//        Command::new("mkdir")
//            .args(&["-p", parent.to_str().ok_or(error::ErrorKind::PathNotUTF8)?])
//            .output()?;
//        let mut new = File::create(&dst_tmp)?;
//        new.write_all(&new_content.into_bytes())?;
//        let process = Command::new("git")
//            .args(&[
//                "diff",
//                "--color",
//                "--no-index",
//                "--",
//                "/dev/null",
//                dst_tmp.to_str().ok_or(error::ErrorKind::PathNotUTF8)?,
//            ]).output()?;
//        println!("{}", String::from_utf8(process.stdout)?);
//        println!(
//            "\n\nIf the new output is correct, run: \nmv {} {}\n\n",
//            dst_tmp.to_str().ok_or(error::ErrorKind::PathNotUTF8)?,
//            dst.to_str().ok_or(error::ErrorKind::PathNotUTF8)?
//        );
//        Err(error::ErrorKind::TestFailed)?
//    }
//
//    Ok(())
//}
