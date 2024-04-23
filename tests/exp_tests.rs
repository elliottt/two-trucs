use two_trucs::rewrite;

fn main() -> anyhow::Result<()> {
    let mut errors = 0;
    let mut failed = 0;
    let mut passed = 0;

    for entry in std::fs::read_dir("tests/data")? {
        let file = entry?;
        let path = &file.path();
        if file.file_type()?.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "md" {
                    match exp_test(path) {
                        Ok(res) => {
                            if res {
                                passed += 1;
                            } else {
                                failed += 1;
                            }
                        }

                        Err(err) => {
                            println!("{:?}", err);
                            errors += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{} passed; {} failed; {} errors", passed, failed, errors);

    if errors > 0 || failed > 0 {
        std::process::exit(1);
    }

    Ok(())
}

fn exp_test(path: &std::path::Path) -> anyhow::Result<bool> {
    println!("{}", path.display());

    let name = path.file_name().expect("Invalid test name");
    let dir = path.parent().expect("Invalid test path");

    let source = std::fs::read_to_string(path)?;

    let sorted_res = {
        let sort = dir.join("sort").join(name);
        if sort.exists() {
            test_sort(&path, &source, &sort)?
        } else {
            println!(
                "Missing sort expectaiton for {} in {}",
                path.display(),
                sort.display()
            );
            false
        }
    };

    let next_res = {
        let next = dir.join("next").join(name);
        if next.exists() {
            test_next(&path, &source, &next)?
        } else {
            println!(
                "Missing next expectation for {} in {}",
                path.display(),
                next.display()
            );
            false
        }
    };

    Ok(sorted_res && next_res)
}

fn test_sort(path: &std::path::Path, input: &str, exp: &std::path::Path) -> anyhow::Result<bool> {
    let expected = &std::fs::read_to_string(exp)?;
    compare_output(&format!("sort: {}", path.display()), None, input, expected)
}

fn test_next(path: &std::path::Path, input: &str, exp: &std::path::Path) -> anyhow::Result<bool> {
    let expected = &std::fs::read_to_string(exp)?;
    compare_output(
        &format!("next: {}", path.display()),
        Some(String::from("Today")),
        input,
        expected,
    )
}

fn compare_output(
    test_name: &str,
    opt_title: Option<String>,
    input: &str,
    expected: &str,
) -> anyhow::Result<bool> {
    let mut buf = Vec::new();
    rewrite::rewrite(opt_title, input, &mut buf)?;
    let actual = std::str::from_utf8(&buf)?;

    if expected != actual {
        println!("-- {} failed", test_name);
        println!("{}", colored_diff::PrettyDifference { expected, actual });
        Ok(false)
    } else {
        Ok(true)
    }
}
