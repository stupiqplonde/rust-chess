#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Piece {
    kind: PieceKind,
    color: Color,
}

impl PieceKind {
    fn value(self) -> i32 {
        match self {
            PieceKind::Pawn => 100,
            PieceKind::Knight => 320,
            PieceKind::Bishop => 330,
            PieceKind::Rook => 500,
            PieceKind::Queen => 900,
            PieceKind::King => 0,
        }
    }

    fn letter(self) -> char {
        match self {
            PieceKind::Pawn => 'P',
            PieceKind::Knight => 'N',
            PieceKind::Bishop => 'B',
            PieceKind::Rook => 'R',
            PieceKind::Queen => 'Q',
            PieceKind::King => 'K',
        }
    }
}

impl Piece {
    fn to_char(self) -> char {
        let c = self.kind.letter();
        match self.color {
            Color::White => c,
            Color::Black => c.to_ascii_lowercase(),
        }
    }

    fn from_char(c: char) -> Option<Piece> {
        let color = if c.is_ascii_uppercase() {
            Color::White
        } else {
            Color::Black
        };

        let upper = c.to_ascii_uppercase();

        let kind = match upper {
            'P' => PieceKind::Pawn,
            'N' => PieceKind::Knight,
            'B' => PieceKind::Bishop,
            'R' => PieceKind::Rook,
            'Q' => PieceKind::Queen,
            'K' => PieceKind::King,
            _ => return None,
        };

        Some(Piece { kind, color })
    }
}

fn main() {
    let knight = Piece { kind: PieceKind::Knight, color: Color::White };
    let queen = Piece { kind: PieceKind::Queen, color: Color::Black };

    println!("Конь стоит {}", knight.kind.value());
    println!("Ферзь стоит {}", queen.kind.value());
    println!("Белый конь: {}", knight.to_char());
    println!("Чёрный ферзь: {}", queen.to_char());
}