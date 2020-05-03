use nu_test_support::{nu, nu_error, pipeline};

#[test]
fn by_column() {
    let actual = nu!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
            open cargo_sample.toml --raw
            | lines
            | skip 1
            | first 4
            | split-column "="
            | sort-by Column1
            | skip 1
            | first 1
            | get Column1
            | trim
            | echo $it
        "#
    ));

    assert_eq!(actual, "description");
}

#[test]
fn errors_sorting_by_non_existent_column() {

    let actual = nu_error!(
        cwd: "tests/fixtures/formats",pipeline(
        r#"
            ls
            | sort-by NOSUCHCOLUMN
        "#
        ));

    assert!(
            actual.contains("Unknown column"),
            format!("actual: {:?}", actual)
    );
}

#[test]
fn sort_primitive_values() {
    let actual = nu!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
            open cargo_sample.toml --raw
            | lines
            | skip 1
            | first 6
            | sort-by
            | first 1
            | echo $it
        "#
    ));

    assert_eq!(actual, "authors = [\"The Nu Project Contributors\"]");
}
