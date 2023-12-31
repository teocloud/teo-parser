mod test {
    use teo_parser::diagnostics::printer::print_diagnostics;
    use teo_parser::parse;

    #[test]
    fn builtin_std_teo_should_be_no_errors() {
        let path_buf = std::env::current_dir().unwrap().join("src/builtin/std.teo");
        let path = path_buf.to_str().unwrap();
        let (_, diagnostics) = parse(path, None, None);
        print_diagnostics(&diagnostics, true);
        assert_eq!(diagnostics.has_errors(), false);
        assert_eq!(diagnostics.has_warnings(), false);
    }
}