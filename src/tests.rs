#[test]
fn ensure_working() {
    struct TestCase {
        rules: Vec<String>,
        expected_regexes: Vec<String>,
        input: String,
        expected_allowed: bool,
    }

    let test_cases = vec![
        // Simple: direct match
        TestCase {
            rules: vec!["@someone:example.com".to_owned()],
            expected_regexes: vec![r"^@someone:example\.com$".to_owned()],
            input: "@someone:example.com".to_owned(),
            expected_allowed: true,
        },
        // Simple: no match
        TestCase {
            rules: vec!["@someone:example.com".to_owned()],
            expected_regexes: vec![r"^@someone:example\.com$".to_owned()],
            input: "@another:example.com".to_owned(),
            expected_allowed: false,
        },
        // Single rule: Username wildcard
        TestCase {
            rules: vec!["@bot.*:example.com".to_owned()],
            expected_regexes: vec![r"^@bot\.([^:@]*):example\.com$".to_owned()],
            input: "@bot.one:example.com".to_owned(),
            expected_allowed: true,
        },
        TestCase {
            rules: vec!["@bot.*:example.com".to_owned()],
            expected_regexes: vec![r"^@bot\.([^:@]*):example\.com$".to_owned()],
            input: "@bot.two:example.com".to_owned(),
            expected_allowed: true,
        },
        TestCase {
            rules: vec!["@bot.*:example.com".to_owned()],
            expected_regexes: vec![r"^@bot\.([^:@]*):example\.com$".to_owned()],
            input: "@another:example.com".to_owned(),
            expected_allowed: false,
        },
        // Single rule: Domain wildcard
        TestCase {
            rules: vec!["@bot:*.com".to_owned()],
            expected_regexes: vec![r"^@bot:([^:@]*)\.com$".to_owned()],
            input: "@bot:example.com".to_owned(),
            expected_allowed: true,
        },
        TestCase {
            rules: vec!["@bot:*.com".to_owned()],
            expected_regexes: vec![r"^@bot:([^:@]*)\.com$".to_owned()],
            input: "@bot:another.com".to_owned(),
            expected_allowed: true,
        },
        TestCase {
            rules: vec!["@bot:*.com".to_owned()],
            expected_regexes: vec![r"^@bot:([^:@]*)\.com$".to_owned()],
            input: "@bot:another.org".to_owned(),
            expected_allowed: false,
        },
        // Multi rule: Domain wildcard
        TestCase {
            rules: vec!["@bot.*:*.com".to_owned(), "@someone:example.com".to_owned()],
            expected_regexes: vec![
                r"^@bot\.([^:@]*):([^:@]*)\.com$".to_owned(),
                r"^@someone:example\.com$".to_owned(),
            ],
            input: "@bot.one:example.com".to_owned(),
            expected_allowed: true,
        },
        TestCase {
            rules: vec!["@bot.*:*.com".to_owned(), "@someone:example.com".to_owned()],
            expected_regexes: vec![
                r"^@bot\.([^:@]*):([^:@]*)\.com$".to_owned(),
                r"^@someone:example\.com$".to_owned(),
            ],
            input: "@someone:example.com".to_owned(),
            expected_allowed: true,
        },
        TestCase {
            rules: vec!["@bot.*:*.com".to_owned(), "@someone:example.com".to_owned()],
            expected_regexes: vec![
                r"^@bot\.([^:@]*):([^:@]*)\.com$".to_owned(),
                r"^@someone:example\.com$".to_owned(),
            ],
            input: "@bot-dashed:example.com".to_owned(),
            expected_allowed: false,
        },
        TestCase {
            rules: vec!["@bot.*:*.com".to_owned(), "@someone:example.com".to_owned()],
            expected_regexes: vec![
                r"^@bot\.([^:@]*):([^:@]*)\.com$".to_owned(),
                r"^@someone:example\.com$".to_owned(),
            ],
            input: "@another:example.com".to_owned(),
            expected_allowed: false,
        },
    ];

    for test_case in test_cases {
        let regexes = super::parse_patterns_vector(&test_case.rules).unwrap();

        for (i, regex) in regexes.iter().enumerate() {
            assert_eq!(regex.as_str(), test_case.expected_regexes[i].as_str());
        }

        let allowed = super::match_user_id(&test_case.input, &regexes);
        assert_eq!(allowed, test_case.expected_allowed);
    }
}
