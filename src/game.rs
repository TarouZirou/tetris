use std::collections::VecDeque;

use crate::block::{
	BLOCKS, BlockColor, BlockKind, BlockShape, COLOR_TABLE, block_kind, block_kind::WALL as W,
	gen_block_7,
};

pub const FIELD_WIDTH: usize = 11 + 2 + 2;
pub const FIELD_HEIGHT: usize = 20 + 1 + 1;
pub const NEXT_LENGTH: usize = 3;

pub const SCORE_TABLE: [usize; 5] = [0, 100, 200, 400, 800];

pub type Field = [[BlockColor; FIELD_WIDTH]; FIELD_HEIGHT];

#[derive(Clone, Copy)]
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

	pub hold: Option<BlockShape>,
	pub holded: bool,

	pub next: VecDeque<BlockShape>,
	pub next_buf: VecDeque<BlockShape>,

	pub score: usize,
	pub line: usize,
}

impl Game {
	pub fn new() -> Game {
		let mut game = Game {
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
			hold: None,
			holded: false,
			next: gen_block_7().into(),
			next_buf: gen_block_7().into(),
			score: 0,
			line: 0,
		};

		spawn_block(&mut game).ok();
		game
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
pub fn draw(
	Game {
		field,
		pos,
		block,
		hold,
		next,
		score,
		line,
		..
	}: &Game,
) {
	let mut field_buf = *field;
	let ghost_pos = ghost_pos(field, pos, block);

	for y in 0..4 {
		for x in 0..4 {
			if block[y][x] != block_kind::NONE {
				field_buf[y + ghost_pos.y][x + ghost_pos.x] = block_kind::GHOST;
				field_buf[y + pos.y][x + pos.x] = block[y][x];
			}
		}
	}

	//holdを描写
	println!("\x1b[2;28HHOLD");
	if let Some(hold) = hold {
		for y in 0..4 {
			print!("\x1b[{};28H", y + 3);
			for x in 0..4 {
				print!("{}", COLOR_TABLE[hold[y][x]]);
			}
			println!();
		}
	}

	//nextを描写
	for (i, next) in next.iter().take(NEXT_LENGTH).enumerate() {
		for y in 0..4 {
			print!("\x1b[{};28H", i * 4 + y + 9);
			for x in 0..4 {
				print!("{}", COLOR_TABLE[next[y][x]]);
			}
			println!();
		}
	}
	//scoreを描写
	println!("\x1b[7;28HSCORE:{}", score);

	//lineを描写
	println!("\x1b[8;28HLINE:{}", line);

	//カーソルを先頭に移動
	println!("\x1b[H");
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

//ホールド機能
pub fn hold(game: &mut Game) {
	if game.holded {
		return;
	}

	if let Some(mut hold) = game.hold {
		std::mem::swap(&mut hold, &mut game.block);
		game.hold = Some(hold);
		game.pos = Position::init();
	} else {
		game.hold = Some(game.block);
		spawn_block(game).ok();
	}

	game.holded = true;
}

//ラインの削除処理
pub fn erase_line(field: &mut Field) -> usize {
	let mut erase_count = 0;
	for y in 1..FIELD_HEIGHT - 2 {
		let mut can_erase = true;
		for x in 2..FIELD_WIDTH - 2 {
			if field[y][x] == 0 {
				can_erase = false;
				break;
			}
		}

		if can_erase {
			erase_count += 1;
			for y2 in (2..=y).rev() {
				field[y2] = field[y2 - 1];
			}
		}
	}
	erase_count
}

pub fn move_block(game: &mut Game, new_pos: Position) {
	if !is_collision(&game.field, &new_pos, &game.block) {
		game.pos = new_pos;
	}
}

#[allow(clippy::needless_range_loop)]
pub fn rotate_right(game: &mut Game) {
	let mut new_shape: BlockShape = Default::default();
	for y in 0..4 {
		for x in 0..4 {
			new_shape[y][x] = game.block[4 - 1 - x][y];
		}
	}

	if !is_collision(&game.field, &game.pos, &new_shape) {
		game.block = new_shape;
	} else if let Ok(new_pos) = super_rotation(&game.field, &game.pos, &new_shape) {
		game.pos = new_pos;
		game.block = new_shape;
	}
}

#[allow(clippy::needless_range_loop)]
pub fn rotate_left(game: &mut Game) {
	let mut new_shape: BlockShape = Default::default();
	for y in 0..4 {
		for x in 0..4 {
			new_shape[4 - 1 - x][y] = game.block[y][x];
		}
	}

	if !is_collision(&game.field, &game.pos, &new_shape) {
		game.block = new_shape;
	} else if let Ok(new_pos) = super_rotation(&game.field, &game.pos, &new_shape) {
		game.pos = new_pos;
		game.block = new_shape;
	}
}

//ブロックの影の座標を返す
pub fn ghost_pos(field: &Field, pos: &Position, block: &BlockShape) -> Position {
	let mut ghost_pos = pos.clone();
	while {
		let new_pos = Position {
			x: ghost_pos.x,
			y: ghost_pos.y + 1,
		};
		!is_collision(&field, &new_pos, &block)
	} {
		ghost_pos.y += 1;
	}
	ghost_pos
}

//ハードドロップ
pub fn hard_drop(game: &mut Game) {
	while {
		let new_pos = Position {
			x: game.pos.x,
			y: game.pos.y + 1,
		};
		!is_collision(&game.field, &new_pos, &game.block)
	} {
		game.pos.y += 1;
	}
	let new_pos = game.pos;
	move_block(game, new_pos);
}

//スーパーローテーションをおこなう
pub fn super_rotation(field: &Field, pos: &Position, block: &BlockShape) -> Result<Position, ()> {
	let diff_pos = [
		Position {
			x: pos.x,
			y: pos.y.checked_sub(1).unwrap_or(pos.y),
		},
		Position {
			x: pos.x + 1,
			y: pos.y,
		},
		Position {
			x: pos.x,
			y: pos.y + 1,
		},
		Position {
			x: pos.x.checked_sub(1).unwrap_or(pos.x),
			y: pos.y + 1,
		},
	];
	for pos in diff_pos {
		if !is_collision(&field, &pos, &block) {
			return Ok(pos);
		}
	}
	Err(())
}

//落下したときの処理
pub fn landing(game: &mut Game) -> Result<(), ()> {
	fix_block(game);

	//ラインを消去して、スコアを加算
	let line_count = erase_line(&mut game.field);
	game.score += SCORE_TABLE[line_count];

	//合計消去ライン数を加算する
	game.line += line_count;

	spawn_block(game)?;
	//再びホールド可能にする
	game.holded = false;
	Ok(())
}

pub fn spawn_block(game: &mut Game) -> Result<(), ()> {
	game.pos = Position::init();
	//ネクストからブロックを取り出す
	game.block = game.next.pop_front().unwrap();
	if let Some(next) = game.next_buf.pop_front() {
		game.next.push_back(next);
	} else {
		game.next_buf = gen_block_7().into();

		game.next.push_back(game.next_buf.pop_front().unwrap());
	}

	if is_collision(&game.field, &game.pos, &game.block) {
		Err(())
	} else {
		Ok(())
	}
}

pub fn game_over(game: &Game) {
	draw(game);
	println!("\x1b[23;28HGAME OVER!");
	println!("press `q` key to exit.");
}

pub fn quit() {
	println!("\x1b[?25h\x1b[25;28H");
}
