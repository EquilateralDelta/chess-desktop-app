use chess::{Color, Figure, Game, Tile};
use iced::{
    pane_grid, Align, Button, Column, Container, Element, Image, Length, PaneGrid, Row, Sandbox,
    Settings, Text,
};

pub fn main() -> iced::Result {
    ChessApp::run(Settings::default())
}

struct ChessApp {
    game: Game,
    panes: pane_grid::State<Content>,
}

#[derive(Debug, Clone, Copy, Default)]
struct Content();

impl Content {
    fn new() -> Content {
        Content {}
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {}

impl Sandbox for ChessApp {
    type Message = Message;

    fn new() -> Self {
        ChessApp {
            game: Game::new(),
            panes: pane_grid::State::new(Content::new()).0,
        }
    }

    fn title(&self) -> String {
        String::from("ChessApp - Iced")
    }

    fn update(&mut self, message: Message) {}

    fn view(&mut self) -> Element<Message> {
        let mut content = Column::new();
        for (column, column_tiles) in self.game.field.iter().enumerate() {
            let mut row_element = Row::new();
            for (row, tile) in column_tiles.iter().enumerate() {
                let image = get_image(tile).width(Length::Fill).height(Length::Fill);
                let is_even = (column + row) % 2 == 0;
                let tile_style = if is_even {
                    style::Tile::Even
                } else {
                    style::Tile::Odd
                };
                let container = Container::new(image)
                    .width(Length::Units(40))
                    .height(Length::Units(40))
                    .style(tile_style);
                row_element = row_element.push(container);
            }
            content = content.push(row_element);
        }

        Container::new(content)
            .height(Length::Fill)
            .center_y()
            .width(Length::Fill)
            .center_x()
            .into()
    }
}

fn get_image(tile: &Tile) -> Image {
    let path = match tile {
        Tile::Occupied(color, figure) => match (color, figure) {
            (Color::White, Figure::Pawn) => "images/pawn_white.png",
            (Color::White, Figure::Knight) => "images/knight_white.png",
            (Color::White, Figure::Bishop) => "images/bishop_white.png",
            (Color::White, Figure::Rook) => "images/rook_white.png",
            (Color::White, Figure::Queen) => "images/queen_white.png",
            (Color::White, Figure::King) => "images/king_white.png",
            (Color::Black, Figure::Pawn) => "images/pawn_black.png",
            (Color::Black, Figure::Knight) => "images/knight_black.png",
            (Color::Black, Figure::Bishop) => "images/bishop_black.png",
            (Color::Black, Figure::Rook) => "images/rook_black.png",
            (Color::Black, Figure::Queen) => "images/queen_black.png",
            (Color::Black, Figure::King) => "images/king_black.png",
        },
        _ => "",
    };
    Image::new(path)
}

mod style {
    use iced::{container, Background, Color};

    const COLOR_ODD: Color = Color::from_rgb(0.694, 0.894, 0.725);
    const COLOR_EVEN: Color = Color::from_rgb(0.439, 0.635, 0.639);

    pub enum Tile {
        Odd,
        Even,
    }

    impl Tile {
        fn color(&self) -> Color {
            match self {
                Tile::Odd => COLOR_ODD,
                Tile::Even => COLOR_EVEN,
            }
        }
    }

    impl container::StyleSheet for Tile {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(self.color())),
                ..container::Style::default()
            }
        }
    }
}
