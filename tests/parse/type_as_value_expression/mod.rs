mod test {
    use teo_parser::diagnostics::printer::print_diagnostics;
    use teo_parser::parse;

    #[test]
    fn type_as_value_expression_can_be_used_as_constant() {
        let path_buf = std::env::current_dir().unwrap().join("tests/parse/type_as_value_expression/schemas/01.teo");
        let path = path_buf.to_str().unwrap();
        let (_, diagnostics) = parse(path, None, None);
        assert_eq!(diagnostics.has_errors(), false);
        assert_eq!(diagnostics.has_warnings(), true);
    }

    #[test]
    fn type_as_value_generic_should_work_as_expected() {
        let path_buf = std::env::current_dir().unwrap().join("tests/parse/type_as_value_expression/schemas/02.teo");
        let path = path_buf.to_str().unwrap();
        let (_, diagnostics) = parse(path, None, None);
        print_diagnostics(&diagnostics, true);
        assert_eq!(diagnostics.has_errors(), false);
        assert_eq!(diagnostics.has_warnings(), false);
    }
}