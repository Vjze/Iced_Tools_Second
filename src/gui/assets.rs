pub mod pngs {
    pub static IMG_LOGO: &[u8] = include_bytes!("../../resources/logo/logo.png");
    pub static IMG_LOGO_DARK: &[u8] = include_bytes!("../../resources/logo/logo_dark.png");
    pub static IMG_LOGO_LIGHT: &[u8] = include_bytes!("../../resources/logo/logo_light.png");
    pub static SEARCT: &[u8] = include_bytes!("../../resources/assets/search.svg");
}
pub mod fonts {
    pub static FONT: &[u8] =
        include_bytes!("../../resources/assets/SourceHanSansHWSC-Regular.otf").as_slice();
}
