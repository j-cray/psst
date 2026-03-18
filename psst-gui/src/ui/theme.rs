use crate::data::Theme;
use xilem::Color;

#[derive(Clone, Debug, PartialEq)]
pub struct AppTheme {
    pub background_light: Color,
    pub background_dark: Color,
    pub foreground_light: Color,
    pub foreground_dark: Color,

    pub grey_000: Color,
    pub grey_100: Color,
    pub grey_200: Color,
    pub grey_300: Color,
    pub grey_400: Color,
    pub grey_500: Color,
    pub grey_600: Color,
    pub grey_700: Color,

    pub blue_100: Color,
    pub blue_200: Color,
    pub red: Color,

    pub menu_button_bg_active: Color,
    pub menu_button_bg_inactive: Color,
    pub menu_button_fg_active: Color,
    pub menu_button_fg_inactive: Color,

    pub grid_unit: f64,
}

impl AppTheme {
    pub fn new(theme_type: Theme) -> Self {
        match theme_type {
            Theme::Light => Self::light(),
            Theme::Dark => Self::dark(),
        }
    }

    pub fn grid(&self, count: f64) -> f64 {
        self.grid_unit * count
    }

    pub fn light() -> Self {
        let grey_000 = Color::from_rgb8(0x00, 0x00, 0x00);
        let grey_100 = Color::from_rgb8(0x33, 0x33, 0x33);
        let grey_200 = Color::from_rgb8(0x4f, 0x4f, 0x4f);
        let grey_300 = Color::from_rgb8(0x82, 0x82, 0x82);
        let grey_400 = Color::from_rgb8(0xbd, 0xbd, 0xbd);
        let grey_500 = Color::from_rgba8(0xe5, 0xe6, 0xe7, 0xff);
        let grey_600 = Color::from_rgba8(0xf5, 0xf6, 0xf7, 0xff);
        let grey_700 = Color::from_rgba8(0xff, 0xff, 0xff, 0xff);
        let blue_100 = Color::from_rgb8(0x5c, 0xc4, 0xff);
        let blue_200 = Color::from_rgb8(0x00, 0x8d, 0xdd);

        Self {
            background_light: grey_700.clone(),
            background_dark: grey_600.clone(),
            foreground_light: grey_100.clone(),
            foreground_dark: grey_000.clone(),

            grey_000,
            grey_100: grey_100.clone(),
            grey_200,
            grey_300,
            grey_400,
            grey_500: grey_500.clone(),
            grey_600: grey_600.clone(),
            grey_700,

            blue_100,
            blue_200,
            red: Color::from_rgba8(0xEB, 0x57, 0x57, 0xFF),

            menu_button_bg_active: grey_500.clone(),
            menu_button_bg_inactive: grey_600.clone(),
            menu_button_fg_active: grey_000.clone(),
            menu_button_fg_inactive: grey_100.clone(),

            grid_unit: 8.0,
        }
    }

    pub fn dark() -> Self {
        let grey_000 = Color::from_rgb8(0xff, 0xff, 0xff);
        let grey_100 = Color::from_rgb8(0xf2, 0xf2, 0xf2);
        let grey_200 = Color::from_rgb8(0xe0, 0xe0, 0xe0);
        let grey_300 = Color::from_rgb8(0xbd, 0xbd, 0xbd);
        let grey_400 = Color::from_rgb8(0x82, 0x82, 0x82);
        let grey_500 = Color::from_rgb8(0x4f, 0x4f, 0x4f);
        let grey_600 = Color::from_rgb8(0x33, 0x33, 0x33);
        let grey_700 = Color::from_rgb8(0x28, 0x28, 0x28);
        let blue_100 = Color::from_rgb8(0x00, 0x8d, 0xdd);
        let blue_200 = Color::from_rgb8(0x5c, 0xc4, 0xff);

        Self {
            background_light: grey_700.clone(),
            background_dark: grey_600.clone(),
            foreground_light: grey_100.clone(),
            foreground_dark: grey_000.clone(),

            grey_000,
            grey_100: grey_100.clone(),
            grey_200,
            grey_300,
            grey_400,
            grey_500: grey_500.clone(),
            grey_600: grey_600.clone(),
            grey_700,

            blue_100,
            blue_200,
            red: Color::from_rgba8(0xEB, 0x57, 0x57, 0xFF),

            menu_button_bg_active: grey_500.clone(),
            menu_button_bg_inactive: grey_600.clone(),
            menu_button_fg_active: grey_000.clone(),
            menu_button_fg_inactive: grey_100.clone(),

            grid_unit: 8.0,
        }
    }
}
