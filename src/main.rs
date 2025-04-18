mod block;
mod game;

use block::{BLOCKS, BlockKind};
use game::*;
use getch_rs::{Getch, Key};
use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {
	let game = Arc::new(Mutex::new(Game::new()));

	// 画面をクリアし、
	//カーソルを一番左上へ移動し、
	//カーソルを非表示にする
	println!("\x1b[2J\x1b[H\x1b[?25l");
	draw(&game.lock().unwrap());

	{
		let game = Arc::clone(&game);

		let _ = thread::spawn(move || {
			loop {
				thread::sleep(time::Duration::from_millis(500));
				let mut game = game.lock().unwrap();
				let new_pos = Position {
					x: game.pos.x,
					y: game.pos.y + 1,
				};
				if !is_collision(&game.field, &new_pos, &game.block) {
					game.pos = new_pos;
				} else {
					//ブロックを固定
					fix_block(&mut game);

					//ラインの削除処理
					erase_line(&mut game.field);

					//プロックを生成
					if spawn_block(&mut game).is_err() {
						game_over(&game);
						break;
					}
				}

				draw(&game);
			}
		});
	}

	let g = Getch::new();
	loop {
		match g.getch() {
			Ok(Key::Char('q')) => {
				break;
			}
			Ok(Key::Char('a')) => {
				let mut game = game.lock().unwrap();
				let new_pos = Position {
					x: game.pos.x.checked_sub(1).unwrap_or_else(|| game.pos.x),
					y: game.pos.y,
				};
				move_block(&mut game, new_pos);
				draw(&game);
			}
			Ok(Key::Char('s')) => {
				let mut game = game.lock().unwrap();
				let new_pos = Position {
					x: game.pos.x,
					y: game.pos.y.checked_add(1).unwrap_or_else(|| game.pos.y),
				};
				move_block(&mut game, new_pos);
				draw(&game);
			}
			Ok(Key::Char('d')) => {
				let mut game = game.lock().unwrap();
				let new_pos = Position {
					x: game.pos.x.checked_add(1).unwrap_or_else(|| game.pos.x),
					y: game.pos.y,
				};
				move_block(&mut game, new_pos);
				draw(&game);
			}
			Ok(Key::Char('j')) => {
				let mut game = game.lock().unwrap();
				rotate_left(&mut game);
				draw(&game);
			}
			Ok(Key::Char('l')) => {
				let mut game = game.lock().unwrap();
				rotate_right(&mut game);
				draw(&game);
			}
			_ => (),
		}
	}

	quit();
}
