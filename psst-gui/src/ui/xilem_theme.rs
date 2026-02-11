use crate::data::Theme;

// Re-exporting Color for now; in a real Xilem app this might need to be peniko::Color
// or a custom wrapper. For migration, we'll stick to a simple representation or
// use the one from druid if convenient, but let's try to be agnostic.
#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub const fn rgb8(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn grey8(l: u8) -> Self {
        Self {
            r: l,
            g: l,
            b: l,
            a: 255,
        }
    }

    pub const fn from_rgba32_u32(val: u32) -> Self {
        let r = ((val >> 24) & 0xff) as u8;
        let g = ((val >> 16) & 0xff) as u8;
        let b = ((val >> 8) & 0xff) as u8;
        let a = (val & 0xff) as u8;
        Self { r, g, b, a }
    }
}

#[derive(Clone, Debug)]
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
        let grey_000 = Color::grey8(0x00);
        let grey_100 = Color::grey8(0x33);
        let grey_200 = Color::grey8(0x4f);
        let grey_300 = Color::grey8(0x82);
        let grey_400 = Color::grey8(0xbd);
        let grey_500 = Color::from_rgba32_u32(0xe5e6e7ff);
        let grey_600 = Color::from_rgba32_u32(0xf5f6f7ff);
        let grey_700 = Color::from_rgba32_u32(0xffffffff);
        let blue_100 = Color::rgb8(0x5c, 0xc4, 0xff);
        let blue_200 = Color::rgb8(0x00, 0x8d, 0xdd);

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
            red: Color::rgba8(0xEB, 0x57, 0x57, 0xFF),

            menu_button_bg_active: grey_500.clone(),
            menu_button_bg_inactive: grey_600.clone(),
            menu_button_fg_active: grey_000.clone(),
            menu_button_fg_inactive: grey_100.clone(),

            grid_unit: 8.0,
        }
    }

    pub fn dark() -> Self {
        let grey_000 = Color::grey8(0xff);
        let grey_100 = Color::grey8(0xf2);
        let grey_200 = Color::grey8(0xe0);
        let grey_300 = Color::grey8(0xbd);
        let grey_400 = Color::grey8(0x82);
        let grey_500 = Color::grey8(0x4f);
        let grey_600 = Color::grey8(0x33);
        let grey_700 = Color::grey8(0x28);
        let blue_100 = Color::rgb8(0x00, 0x8d, 0xdd);
        let blue_200 = Color::rgb8(0x5c, 0xc4, 0xff);

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
            red: Color::rgba8(0xEB, 0x57, 0x57, 0xFF),

            menu_button_bg_active: grey_500.clone(),
            menu_button_bg_inactive: grey_600.clone(),
            menu_button_fg_active: grey_000.clone(),
            menu_button_fg_inactive: grey_100.clone(),

            grid_unit: 8.0,
        }
    }
}
