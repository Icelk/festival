//---------------------------------------------------------------------------------------------------- Use
use crate::constants::{
	BONE,MEDIUM_GRAY,
	QUEUE_ALBUM_ART_SIZE,
};
use crate::text::{
	UI_QUEUE_CLEAR,UI_QUEUE_SHUFFLE,
	QUEUE_CLEAR,QUEUE_SHUFFLE,
};
use shukusai::collection::{
	Song,Album
};
use shukusai::kernel::{
	FrontendToKernel,
};
use egui::{
	ScrollArea,Label,RichText,SelectableLabel,
	Sense,TextStyle,Button,
};
use benri::send;
use readable::HeadTail;

//---------------------------------------------------------------------------------------------------- Queue
impl crate::data::Gui {
#[inline(always)]
pub fn show_tab_queue(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, width: f32, height: f32) {
	self.set_visuals(ui);

	//-------------------------------------------------- Queue.
	ScrollArea::vertical()
		.id_source("Queue")
		.max_width(f32::INFINITY)
		.max_height(f32::INFINITY)
		.auto_shrink([false; 2])
		.show_viewport(ui, |ui, _|
	{
		// Sizing.
		let width  = ui.available_width();
		let height = ui.available_height();
		const SIZE:  f32 = 35.0;
		const SIZE2: f32 = SIZE * 2.0;

		ui.horizontal(|ui| {
			let width = (width / 3.0) - 10.0;

			let button = Button::new(RichText::new(UI_QUEUE_CLEAR).size(SIZE));
			if ui.add_sized([width, SIZE2], button).on_hover_text(QUEUE_CLEAR).clicked() {
				crate::clear_stop!(self);
			}
			let button = Button::new(RichText::new(UI_QUEUE_SHUFFLE).size(SIZE));
			if ui.add_sized([width, SIZE2], button).on_hover_text(QUEUE_SHUFFLE).clicked() {
				send!(self.to_kernel, FrontendToKernel::Shuffle);
			}

			let len = self.audio_state.queue.len();
			let index = if len == 0 { 0 } else { self.audio_state.queue_idx.unwrap_or(0) + 1 };
			let text = Label::new(
				RichText::new(format!("[{index}/{len}]"))
					.color(BONE)
					.text_style(TextStyle::Name("30".into()))
			);
			ui.add_sized([ui.available_width(), SIZE2], text);
		});

		ui.add_space(5.0);
		ui.separator();

		let mut current_artist = None;
		let mut current_album  = None;

		for (index, key) in self.audio_state.queue.iter().enumerate() {
			let (artist, album, song) = self.collection.walk(key);

			let same_artist = current_artist == Some(artist);
			let same_album  = current_album == Some(album);

			//-------------------------------------------------- Artist.
			if !same_artist {
				// Only add space if we've added previous `Artist`'s before.
				if current_artist.is_some() {
					ui.add_space(60.0);
				}

				// Artist info.
				let artist_name = Label::new(
					RichText::new(&artist.name)
					.text_style(TextStyle::Name("30".into()))
				);
				crate::artist_label!(self, artist, album.artist, ui, artist_name);
				current_artist = Some(artist);
				ui.add_space(5.0);
			}

			if !same_album {
				ui.separator();
				ui.horizontal(|ui| {
					// Remove button.
					let button = Button::new(RichText::new("-").size(SIZE));
					if ui.add_sized([SIZE, QUEUE_ALBUM_ART_SIZE], button).clicked() {
						// HACK:
						// Iterate until we find a `Song` that doesn't
						// belong to the same `Album`.
						//
						// This could end bad if there's an `Album` with _many_ `Song`'s.
						// Considering this is in the `GUI` update
						// loop, even worse... buuuuut who is going to
						// have an `Album` with 10,000s of `Song`'s... right?
						let mut end = index;
						let mut hit = false;
						let len = self.audio_state.queue.len();
						for key in self.audio_state.queue.range(index..) {
							if self.collection.songs[key].album != song.album {
								let end = if end == 0 { 1 } else { end };
								crate::remove_queue_range!(self, index..end);
								hit = true;
								break;
							}
							end += 1;
						}

						if !hit {
							let end = if end == 0 { 1 }  else { end };
							crate::remove_queue_range!(self, index..end);
						}
					}
					crate::no_rounding!(ui);
					crate::album_button!(self, album, song.album, ui, ctx, QUEUE_ALBUM_ART_SIZE, "");

					ui.vertical(|ui| {
						// Info.
						let album_title = Label::new(RichText::new(&album.title).color(BONE));
						ui.add(album_title);
						ui.label(album.release.as_str());
						ui.label(album.runtime.as_str());
					});
				});

				current_album = Some(album);
			}

			//-------------------------------------------------- Song.
			ui.horizontal(|ui| {
				// Remove button.
				if ui.add_sized([SIZE, SIZE,], Button::new("-")).clicked() {
					crate::remove_queue_range!(self, index..index+1);
				}

				// FIXME:
				// Queue's song buttons are slightly different,
				// so we don't get to use the `song_button!()` macro.
				let mut rect = ui.cursor();
				rect.max.y = rect.min.y + SIZE;
				rect.max.x = rect.min.x + ui.available_width();

				// HACK:
				// If we remove an index but are still playing the `Song`,
				// the colored label indicating which one we're on will be wrong,
				// so it has to the the same index _and_ the same song.
				let same =
					self.audio_state.queue_idx == Some(index) &&
					self.audio_state.song      == Some(*key);

				let resp = ui.put(rect, SelectableLabel::new(same, ""));
				if resp.clicked() {
					crate::play_queue_index!(self, index);
				} else if resp.middle_clicked() {
					crate::open!(self, album);
				} else if resp.secondary_clicked() {
					crate::add_song!(self, song.title, *key);
				}


				ui.allocate_ui_at_rect(rect, |ui| {
					ui.horizontal_centered(|ui| {
						match song.track {
							Some(t) => ui.add(Label::new(format!("{: >3}{: >8}    {}", t, song.runtime.as_str(), &song.title))),
							None    => ui.add(Label::new(format!("{: >3}{: >8}    {}", "", song.runtime.as_str(), &song.title))),
						}
					});
				});
			});
		}
	});
}}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
