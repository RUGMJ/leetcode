use std::{
    env, fs, io,
    path::PathBuf,
    process::{Command, Stdio},
};

use cargo::{ops::NewOptions, Config};
use dotenvy_macro::dotenv;
use html2md::parse_html;
use leetcoderustapi::{problem_actions::Problem, resources::descr::Question, UserApi};

async fn find_problems<'a>(api: &UserApi, problem_keyword: &'a str) -> Vec<Question> {
    println!("Searching for \"{}\"", problem_keyword);

    let problems = api
        .problem_builder()
        .set_keyword(problem_keyword)
        .set_note_limit(1)
        .build()
        .await
        .unwrap();

    problems.data.problemsetQuestionList.questions
}

async fn get_full_problem_data(api: &UserApi, problem: &Question) -> Problem {
    api.set_problem(&problem.titleSlug).await.unwrap()
}

fn get_code_snippet(problem: &Problem) -> String {
    let code_snippets = problem.code_snippets().unwrap();
    let code_snippets: Vec<_> = code_snippets
        .iter()
        .filter(|s| s.langSlug == "rust")
        .collect();

    code_snippets.first().unwrap().code.clone()
}

struct Markdown(String);

impl Markdown {
    fn new(title: &str, slug: &str, content: &Option<String>) -> Markdown {
        let mut markdown = String::new();

        markdown.push_str(&format!("# {}\n", title));
        markdown.push_str(&format!("> <https://leetcode.com/problems/{}>\n\n", slug));

        if let Some(content) = content {
            markdown.push_str(&format!("{}\n", parse_html(content)));
        }

        Markdown(markdown)
    }

    fn to_raw(&self) -> String {
        self.0.clone()
    }
}

fn create_project(path: &str, title: &str, readme: &str) {
    if fs::metadata(path).is_ok() {
        panic!(
            "A file or directory already exists with the name \"{}\" ",
            path
        );
    }

    let new_opts = NewOptions::new(
        None,
        true,
        false,
        PathBuf::from(&path),
        Some(title.to_string()),
        None,
        None,
    )
    .unwrap();

    cargo::ops::new(&new_opts, &Config::default().unwrap())
        .expect("Something went wrong creating the binary");

    fs::write(format!("{}/README.md", path), readme)
        .expect("Something went wrong creating the README");
}

fn create_binary(path: &str, code_snippet: &str) {
    let code_snippet = format!(
        r#"
fn main() {{}}

struct Solution;
{}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test() {{
        unimplemented!();
    }}
}}
"#,
        code_snippet
    );

    fs::write(format!("{}/src/main.rs", path), code_snippet)
        .expect("Something went wrong creating main.rs");
}

fn render_markdown(path: &str) {
    let mut glow = Command::new("glow")
        .arg(format!("{}/README.md", path))
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Something went wrong when launching glow");

    glow.wait()
        .expect("Something went wrong when waiting for glow");

    println!("-- Press any key to continue ... --");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong");
}

fn open_in_editor(path: &str, editor: &str) {
    Command::new(editor)
        .arg(path)
        .output()
        .expect("Something went wrong launching neovide");
}

#[tokio::main]
async fn main() {
    let token = dotenv!("COOKIE");
    let solutions_path = dotenv!("SOLUTIONS_PATH");
    let editor = dotenv!("EDITOR");

    let args: Vec<String> = env::args().collect();
    let problem_keyword = &args[1..].join(" ");

    if problem_keyword.is_empty() {
        panic!("You didnt provide a search term");
    }

    let api = UserApi::new(token).await.unwrap();

    let problems = find_problems(&api, problem_keyword).await;
    let problem = problems.first();

    if problem.is_none() {
        panic!("Couldn't find the problem \"{}\"", problem_keyword);
    }

    let problem = problem.unwrap();
    let problem = get_full_problem_data(&api, problem).await;
    let code_snippet = get_code_snippet(&problem);
    let problem = problem.full_data.data.question;
    let markdown = Markdown::new(&problem.title, &problem.titleSlug, &problem.content);

    let snake_case_title = problem.titleSlug.replace('-', "_");
    let path = format!("{}{}", solutions_path, snake_case_title);

    create_project(&path, &snake_case_title, &markdown.to_raw());
    create_binary(&path, &code_snippet);
    render_markdown(&path);
    open_in_editor(&path, editor);
}
