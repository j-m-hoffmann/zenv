use zenv::{zenv, Zenv};

#[test]
fn zenv_basic() {
    let z = Zenv::new("tests/.env.basic", false).parse().unwrap();

    assert_eq!(z.get("BASIC").unwrap(), "basic");
    assert_eq!(z.get("EMPTY").unwrap(), "");
    assert_eq!(z.get("SINGLE_QUOTES").unwrap(), "single_quotes");
    assert_eq!(z.get("DOUBLE_QUOTES").unwrap(), "double_quotes");
}

#[test]
fn zenv_expanded() {
    let z = Zenv::new("tests/.env.expanded", true).parse().unwrap();

    assert_eq!(z.get("BASIC").unwrap(), "basic");

    assert_eq!(z.get("EXPANDED").unwrap(), "basic-expanded");
    assert_eq!(z.get("DOUBLE_EXPANDED").unwrap(), "basic-basic-expanded");

    assert_eq!(z.get("EXPANDED_NEW").unwrap(), "basic_expanded");
    assert_eq!(
        z.get("DOUBLE_EXPANDED_NEW").unwrap(),
        "basic_basic-basic-expanded"
    );

    assert_eq!(z.get("NO_EXPANDED").unwrap(), "$BASIC-expanded");
    assert_eq!(z.get("NO_DOUBLE_EXPANDED").unwrap(), "$BASIC-$EXPANDED");

    assert_eq!(z.get("NO_EXPANDED_NEW").unwrap(), "${BASIC}_expanded");
    assert_eq!(
        z.get("NO_DOUBLE_EXPANDED_NEW").unwrap(),
        "${BASIC}_${DOUBLE_EXPANDED}"
    );

    assert_eq!(z.get("NO_SYSTEM_VAR").unwrap(), "_dont_exist");
    assert_ne!(z.get("SYSTEM_VAR").unwrap(), "_exist");
}

#[test]
fn zenv_macro_basic() {
    use std::env::var_os;

    zenv!("tests/.env.basic");

    assert_eq!(var_os("BASIC").unwrap(), "basic");
    assert_eq!(var_os("EMPTY").unwrap(), "");
    assert_eq!(var_os("SINGLE_QUOTES").unwrap(), "single_quotes");
    assert_eq!(var_os("DOUBLE_QUOTES").unwrap(), "double_quotes");
}

#[test]
fn zenv_macro_expanded() {
    use std::env::var_os;

    zenv!("tests/.env.expanded", true);

    assert_eq!(var_os("BASIC").unwrap(), "basic");
    assert_eq!(var_os("EXPANDED").unwrap(), "basic-expanded");
    assert_eq!(var_os("DOUBLE_EXPANDED").unwrap(), "basic-basic-expanded");
}
