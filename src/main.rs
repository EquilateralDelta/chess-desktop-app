use chess::{Color, Figure, File, Game, Position, Rank, Tile};
use iced::{button, Button, Column, Container, Element, Image, Length, Row, Sandbox, Settings};

mod styles;

pub fn main() -> iced::Result {
    ChessApp::run(Settings::default())
}

struct ChessApp {
    game: Game,
    button_states: [[button::State; 8]; 8],
    selected: Option<Position>,
    moves: Vec<Position>,
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
            moves: Default::default(),
        }
    }

    fn title(&self) -> String {
        String::from("ChessApp - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TileSelected(position) => {
                if self.moves.contains(&position) {
                    self.game
                        .make_move(self.selected.unwrap(), position)
                        .unwrap();
                    self.selected = None;
                    self.moves = Default::default();
                } else {
                    self.selected = Some(position);
                    self.moves = self.game.moves_available(position);
                }
            }
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
                let style = ChessApp::get_tile_style(position, &self.moves, self.selected);
                let element = ChessApp::get_tile_element(&self.game, position, state, style);
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
    fn get_tile_style(
        position: Position,
        moves: &Vec<Position>,
        selected: Option<Position>,
    ) -> styles::Tile {
        let is_even = (position.rank as usize + position.file as usize) % 2 == 0;
        if moves.contains(&position) {
            styles::Tile::Move
        } else if selected == Some(position) {
            styles::Tile::Selected { is_even }
        } else {
            styles::Tile::NonSelected { is_even }
        }
    }

    fn get_tile_element<'a>(
        game: &Game,
        position: Position,
        state: &'a mut button::State,
        style: styles::Tile,
    ) -> Button<'a, Message> {
        let tile = game.field.get(position);
        let image = ChessApp::get_tile_image(&tile)
            .width(Length::Fill)
            .height(Length::Fill);

        Button::new(state, image)
            .on_press(Message::TileSelected(position))
            .width(Length::Units(40))
            .height(Length::Units(40))
            .padding(0)
            .style(style)
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
