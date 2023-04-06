use bob_the::bob::Script;


fn main() {
    static SOURCE: &str = r#"
        PRINT "Hello, world!"#;
    let script = Script::from(SOURCE);
    script.run().unwrap();
}
