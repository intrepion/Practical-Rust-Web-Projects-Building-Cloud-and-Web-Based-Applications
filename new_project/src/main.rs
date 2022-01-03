use std::env;
use std::iter;
use std::path::Path;
use std::process;
use std::process::Command;

fn display_usage(code: i32) {
    println!("\nusage: ./setup-coding <lastPage> <projectName> <projectPage>");

    process::exit(code);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_has_four_values = args.len() == 4;

    if !(args_has_four_values) {
        display_usage(1);
    }

    let last_page = &args[1]
        .parse::<i32>()
        .expect("last_page_should_be_a_number");
    let project_name = &args[2];
    let project_page = &args[3]
        .parse::<i32>()
        .expect("project_page should be a number");

    let page_number_digits = ((*project_page as f64).log(10_f64).trunc() as i32) + 1;
    let last_page_digits = ((*last_page as f64).log(10_f64).trunc() as i32) + 1;
    let difference = last_page_digits - page_number_digits;

    let project_folder = "page-".to_owned()
        + &iter::repeat("0")
            .take(difference as usize)
            .collect::<String>()
        + &project_page.to_string()
        + "-"
        + &project_name;

    let project_path = "../projects/".to_owned() + &project_folder;

    Command::new("mkdir")
        .current_dir("..")
        .arg("projects")
        .spawn()
        .expect("failed to spawn mkdir")
        .wait()
        .expect("failed to wait mkdir");

    if Path::new(&project_path).exists() {
        println!("{} already exists", &project_path);
        process::exit(1);
    }

    Command::new("cargo")
        .current_dir("../projects")
        .arg("new")
        .arg(&project_folder)
        .spawn()
        .expect("failed to spawn cargo new")
        .wait()
        .expect("failed to wait cargo new");

    Command::new("cargo")
        .current_dir(&project_path)
        .arg("build")
        .spawn()
        .expect("failed to spawn cargo build")
        .wait()
        .expect("failed to wait cargo build");

    Command::new("git")
        .arg("add")
        .arg("-A")
        .spawn()
        .expect("failed to spawn git add")
        .wait()
        .expect("failed to wait git add");

    let commit_message = format!("cargo new {}", &project_folder);

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .spawn()
        .expect("failed to spawn git commit")
        .wait()
        .expect("failed to wait git commit");

    Command::new("git")
        .arg("push")
        .spawn()
        .expect("failed to spawn git push")
        .wait()
        .expect("failed to wait git push");
}
