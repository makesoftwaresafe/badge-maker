#![no_main]
use badge_maker::BadgeBuilder;
use libfuzzer_sys::fuzz_target;
use libfuzzer_sys::arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
pub struct SvgInputs {
    pub label: String,
    pub message: String,
    pub link: String,
    pub link_left: String,
    pub link_right: String,
    pub logo_url: String,
    pub logo_width: usize,
    pub logo_padding: isize,
}

// Only fuzzing string inputs, since randomly generated
// parsed text (ie: color_parse, label_color_parse) will
// pretty much always produce invalid input.
fuzz_target!(|input: SvgInputs| {
    let svg = BadgeBuilder::new()
        .label(&input.label)
        .message(&input.message)
        .link(&input.link)
        .link_left(&input.link_left)
        .link_right(&input.link_right)
        .logo_url(&input.logo_url)
        .logo_width(input.logo_width)
        .logo_padding(input.logo_padding)
        .build();

        assert!(svg.is_ok());
});
