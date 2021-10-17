use iced::{button, Background, Color};

const COLOR_ODD: Color = Color::from_rgb(0.694, 0.894, 0.725);
const COLOR_SELECTED_ODD: Color = Color::from_rgb(0.564, 0.725, 0.592);
const COLOR_EVEN: Color = Color::from_rgb(0.439, 0.635, 0.639);
const COLOR_SELECTED_EVEN: Color = Color::from_rgb(0.33, 0.53, 0.53);
const COLOR_MOVE: Color = Color::from_rgb(0.57, 0.17, 0.25);

pub enum Tile {
    Move,
    Selected { is_even: bool },
    NonSelected { is_even: bool },
}

impl Tile {
    fn color(&self) -> Color {
        match self {
            Tile::Move => COLOR_MOVE,
            Tile::Selected {is_even: true} => COLOR_SELECTED_EVEN,
            Tile::Selected {is_even: false} => COLOR_SELECTED_ODD,
            Tile::NonSelected { is_even: true } => COLOR_EVEN,
            Tile::NonSelected { is_even: false } => COLOR_ODD,
        }
    }
}

impl button::StyleSheet for Tile {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(self.color())),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        self.active()
    }
}