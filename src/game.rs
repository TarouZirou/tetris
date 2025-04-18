use crate::block::{
	BLOCKS, BlockColor, BlockKind, BlockShape, COLOR_TABLE, block_kind, block_kind::WALL as W,
};

pub const FIELD_WIDTH: usize = 11 + 2 + 2;
pub const FIELD_HEIGHT: usize = 20 + 1 + 1;
pub type Field = [[BlockColor; FIELD_WIDTH]; FIELD_HEIGHT];

pub struct Position {
	pub x: usize,
	pub y: usize,
}

impl Position {
	pub fn init() -> Position {
		Position { x: 4, y: 0 }
	}
}

pub struct Game {
	pub field: Field,
	pub pos: Position,
	pub block: BlockShape,
}

impl Game {
	pub fn new() -> Game {
		Game {
			field: [
				[0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
				[0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
				[0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
				[0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
				[0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
				[0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
				[0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
				[0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
				[0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
				[0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
				[0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
				[0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
				[0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
				[0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
				[0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
				[0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
				[0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
				[0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
				[0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
				[0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
				[0, W, W, W, W, W, W, W, W, W, W, W, W, W, 0],
				[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			],
			pos: Position::init(),
			block: BLOCKS[rand::random::<BlockKind>() as usize],
		}
	}
}

pub fn is_collision(field: &Field, pos: &Position, block: &BlockShape) -> bool {
	for y in 0..4 {
		for x in 0..4 {
			if y + pos.y >= FIELD_HEIGHT || x + pos.x >= FIELD_WIDTH {
				continue;
			}
			//もしブロックとフィールドが重なっていたら、衝突したことになる
			if block[y][x] != block_kind::NONE && field[y + pos.y][x + pos.x] != block_kind::NONE {
				return true;
			}
		}
	}

	false
}

#[allow(clippy::needless_range_loop)]
pub fn draw(Game { field, pos, block }: &Game) {
	let mut field_buf = *field;

	for y in 0..4 {
		for x in 0..4 {
			if block[y][x] != block_kind::NONE {
				field_buf[y + pos.y][x + pos.x] = block[y][x];
			}
		}
	}

	println!("\x1b[H"); //カーソルを先頭に移動
	for y in 0..FIELD_HEIGHT - 1 {
		for x in 1..FIELD_WIDTH - 1 {
			print!("{}", COLOR_TABLE[field_buf[y][x]]);
		}
		println!();
	}
}

pub fn fix_block(game: &mut Game) {
	for y in 0..4 {
		for x in 0..4 {
			if game.block[y][x] != block_kind::NONE {
				game.field[y + game.pos.y][x + game.pos.x] = game.block[y][x];
			}
		}
	}
}

//ラインの削除処理
pub fn erase_line(field: &mut Field) {
	for y in 1..FIELD_HEIGHT - 2 {
		let mut can_erase = true;
		for x in 2..FIELD_WIDTH - 2 {
			if field[y][x] == 0 {
				can_erase = false;
				break;
			}
		}

		if can_erase {
			for y2 in (2..=y).rev() {
				field[y2] = field[y2 - 1];
			}
		}
	}
}

pub fn move_block(game: &mut Game, new_pos: Position) {
	if !is_collision(&game.field, &new_pos, &game.block) {
		game.pos = new_pos;
	}
}

pub fn rotate_right(game: &mut Game) {
	let mut new_shape: BlockShape = Default::default();
	for y in 0..4 {
		for x in 0..4 {
			new_shape[y][x] = game.block[4 - 1 - x][y];
		}
	}

	if !is_collision(&game.field, &game.pos, &new_shape) {
		game.block = new_shape;
	}
}

pub fn rotate_left(game: &mut Game) {
	let mut new_shape: BlockShape = Default::default();
	for y in 0..4 {
		for x in 0..4 {
			new_shape[4 - 1 - x][y] = game.block[y][x];
		}
	}

	if !is_collision(&game.field, &game.pos, &new_shape) {
		game.block = new_shape;
	}
}

pub fn spawn_block(game: &mut Game) -> Result<(), ()> {
	game.pos = Position::init();
	game.block = BLOCKS[rand::random::<BlockKind>() as usize];
	//let new_pos = Position::init();
	//let new_block = BLOCKS[rand::random::<BlockKind>() as usize];
	if is_collision(&game.field, &game.pos, &game.block) {
		Err(())
	} else {
		//game.pos = new_pos;
		//game.block = new_block;
		Ok(())
	}
}

pub fn game_over(game: &Game) {
	draw(game);
	println!("GAME OVER!");
	println!("press `q` key to exit.");
}

pub fn quit() {
	println!("\x1b[?25h");
}
