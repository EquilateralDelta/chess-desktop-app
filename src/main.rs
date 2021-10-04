use chess::{Color, FieldTrait, Figure, File, Game, Position, Rank, Tile};
use iced::{button, Button, Column, Container, Element, Image, Length, Row, Sandbox, Settings};

pub fn main() -> iced::Result {
    ChessApp::run(Settings::default())
}

struct ChessApp {
    game: Game,
    button_states: [[button::State; 8]; 8],
    selected: Option<Position>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    TileSelected(Position),
}

impl Sandbox for ChessApp {
    type Message = Message;

    fn new() -> Self {
        ChessApp {
            game: Game::new(),
            button_states: Default::default(),
            selected: Default::default(),
        }
    }

    fn title(&self) -> String {
        String::from("ChessApp - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TileSelected(position) => self.selected = Some(position),
        }
    }

    fn view(&mut self) -> Element<Message> {
        let mut content = Column::new();
        for (rank, state_row) in Rank::iter().zip(self.button_states.iter_mut()).rev() {
            let mut rank_elements = Row::new();
            for (file, state) in File::iter().zip(state_row.iter_mut()) {
                let position = Position {
                    rank: *rank,
                    file: *file,
                };
                let element =
                    ChessApp::get_tile_element(&self.game, position, self.selected, state);
                rank_elements = rank_elements.push(element);
            }
            content = content.push(rank_elements);
        }

        Container::new(content)
            .height(Length::Fill)
            .center_y()
            .width(Length::Fill)
            .center_x()
            .into()
    }
}

impl ChessApp {
    fn get_tile_element<'a>(
        game: &'_ Game,
        position: Position,
        selected_position: Option<Position>,
        state: &'a mut button::State,
    ) -> Button<'a, Message> {
        let tile = game.field.get(position);
        let image = ChessApp::get_tile_image(&tile)
            .width(Length::Fill)
            .height(Length::Fill);
        let is_even = (position.rank as usize + position.file as usize) % 2 == 0;
        let is_selected = if let Some(selected_position) = selected_position {
            position == selected_position
        } else {
            false
        };
        let tile_style = style::Tile {
            is_even,
            is_selected,
        };
        Button::new(state, image)
            .on_press(Message::TileSelected(position))
            .width(Length::Units(40))
            .height(Length::Units(40))
            .padding(0)
            .style(tile_style)
    }

    fn get_tile_image(tile: &Tile) -> Image {
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
}

mod style {
    use iced::{button, Background, Color};

    const COLOR_ODD: Color = Color::from_rgb(0.694, 0.894, 0.725);
    const COLOR_ODD_SELECTED: Color = Color::from_rgb(0.564, 0.725, 0.592);
    const COLOR_EVEN: Color = Color::from_rgb(0.439, 0.635, 0.639);
    const COLOR_EVEN_SELECTED: Color = Color::from_rgb(0.33, 0.53, 0.53);

    pub struct Tile {
        pub is_even: bool,
        pub is_selected: bool,
    }

    impl Tile {
        fn color(&self) -> Color {
            if self.is_even {
                if self.is_selected {
                    COLOR_EVEN_SELECTED
                } else {
                    COLOR_EVEN
                }
            } else {
                if self.is_selected {
                    COLOR_ODD_SELECTED
                } else {
                    COLOR_ODD
                }
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
}
