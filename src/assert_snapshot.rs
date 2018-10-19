use failure::err_msg;
use failure::Error;
use serde;
use serde_json;
use sorted_json::to_json;
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    process::Command,
    string::String,
};
// ours
use destination;
use function;
use scrub;

pub fn assert_snapshot_<T>(
    file: &'static str,
    line: u32,
    extra: &str,
    data: &T,
    scrubs: &HashMap<String, String>,
) -> Result<(), Error>
where
    T: serde::Serialize,
{
    let new_content = serde_json::to_value(data)?;
    let mut new_content = to_json(&new_content);
    let mut ext = "json";
    if let serde_json::Value::String(s) = serde_json::to_value(data)? {
        new_content = s;
        ext = "txt";
    }
    let new_content = new_content + "\n";
    let new_content = scrub::scrub(&new_content, scrubs);

    let func = function::function(file, line);
    // see if a file exits in test folder(=> next to src), if so we compare the output with that
    // else we create the file
    let dst: PathBuf = PathBuf::from(destination::destination(file, &func, extra, false, ext));
    let dst_tmp: PathBuf = PathBuf::from(destination::destination(file, &func, extra, true, ext));
    if dst.exists() {
        let mut existing = File::open(&dst)?;
        let mut existing_content = String::new();
        existing.read_to_string(&mut existing_content)?;
        if new_content != existing_content {
            let mut new = File::create(&dst_tmp)?;
            new.write_all(&new_content.into_bytes())?;
            let process = Command::new("git")
                .args(&[
                    "diff",
                    "--color",
                    "--no-index",
                    "--",
                    dst.to_str().ok_or(err_msg("PathNotUtf8"))?,
                    dst_tmp.to_str().ok_or(err_msg("PathNotUtf8"))?,
                ]).output()?;
            println!("{}", String::from_utf8(process.stdout)?);
            println!(
                "\n\nIf the new output is correct, run: \nmv {} {}\n\n",
                (env::current_dir()?.join(dst_tmp))
                    .to_str()
                    .ok_or(err_msg("PathNotUtf8"))?,
                (env::current_dir()?.join(dst.clone()))
                    .to_str()
                    .ok_or(err_msg("PathNotUtf8"))?
            );
            Err(err_msg("TestFailed"))?
        }
    } else {
        let parent = dst.parent().ok_or(err_msg("TestFailed"))?;
        Command::new("mkdir")
            .args(&["-p", parent.to_str().ok_or(err_msg("PathNotUtf8"))?])
            .output()?;
        let mut new = File::create(&dst_tmp)?;
        new.write_all(&new_content.into_bytes())?;
        let process = Command::new("git")
            .args(&[
                "diff",
                "--color",
                "--no-index",
                "--",
                "/dev/null",
                dst_tmp.to_str().ok_or(err_msg("PathNotUtf8"))?,
            ]).output()?;
        println!("{}", String::from_utf8(process.stdout)?);
        println!(
            "\n\nIf the new output is correct, run: \nmv {} {}\n\n",
            (env::current_dir()?.join(dst_tmp))
                .to_str()
                .ok_or(err_msg("PathNotUtf8"))?,
            (env::current_dir()?.join(dst.clone()))
                .to_str()
                .ok_or(err_msg("PathNotUtf8"))?
        );
        Err(err_msg("TestFailed"))?
    }

    Ok(())
}
