mod test {
    use teo_parser::diagnostics::diagnostics::DiagnosticsLog;
    use teo_parser::parse;

    #[test]
    fn type_coerce_shouldnt_error() {
        let path_buf = std::env::current_dir().unwrap().join("tests/parse/type_coerce/schemas/01.teo");
        let path = path_buf.to_str().unwrap();
        let (_, diagnostics) = parse(path, None, None);
        assert_eq!(diagnostics.has_errors(), false);
        assert_eq!(diagnostics.has_warnings(), false);
    }

    #[test]
    fn cannot_coerce_optional_to_non_optional() {
        let path_buf = std::env::current_dir().unwrap().join("tests/parse/type_coerce/schemas/02.teo");
        let path = path_buf.to_str().unwrap();
        let (_, diagnostics) = parse(path, None, None);
        assert_eq!(diagnostics.errors().len(), 1);
        assert_eq!(diagnostics.errors().first().unwrap().message(), "expect Int, found Int64?");
    }

    #[test]
    fn works_for_synthesized_enum_reference() {
        let path_buf = std::env::current_dir().unwrap().join("tests/parse/type_coerce/schemas/03.teo");
        let path = path_buf.to_str().unwrap();
        let (_, diagnostics) = parse(path, None, None);
        assert_eq!(diagnostics.errors().len(), 1);
        assert_eq!(diagnostics.errors().first().unwrap().message(), "expect SerializableScalarFields<Perform>, found other fields");
    }

    #[test]
    fn coerce_synthesized_shape_to_interface() {
        let path_buf = std::env::current_dir().unwrap().join("tests/parse/type_coerce/schemas/04.teo");
        let path = path_buf.to_str().unwrap();
        let (_, diagnostics) = parse(path, None, None);
        assert_eq!(diagnostics.errors().len(), 0);
    }
}