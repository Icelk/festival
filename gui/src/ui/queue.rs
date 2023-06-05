//---------------------------------------------------------------------------------------------------- Use
use crate::constants::{
	BONE,MEDIUM_GRAY,
	QUEUE_ALBUM_ART_SIZE,
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
pub fn show_tab_queue(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
	self.set_visuals(ui);

	// Sizing.
	let width  = ui.available_width();
	let height = ui.available_height();

	//-------------------------------------------------- Queue.
	ScrollArea::both()
		.id_source("Queue")
		.max_width(f32::INFINITY)
		.max_height(f32::INFINITY)
		.auto_shrink([false; 2])
		.show_viewport(ui, |ui, _|
	{
		let mut current_artist = None;
		let mut current_album  = None;

		for (index, key) in self.audio_state.queue.iter().enumerate() {
			let (artist, album, song) = self.collection.walk(key);

			let same_artist = current_artist == Some(artist);
			let same_album  = current_album == Some(album);

			//-------------------------------------------------- Artist.
			if !same_artist {
				ui.add_space(30.0);

				// Artist info.
				let artist_name = Label::new(
					RichText::new(&artist.name)
					.text_style(TextStyle::Name("30".into()))
				);
				if ui.add(artist_name.sense(Sense::click())).clicked() {
					crate::artist!(self, album.artist);
				}
				current_artist = Some(artist);

				// FIXME:
				// This code is duplicated below for new albums.
				ui.add_space(10.0);
				ui.separator();
				ui.add_space(10.0);

				ui.horizontal(|ui| {
					crate::no_rounding!(ui);
					crate::album_button!(self, album, song.album, ui, ctx, QUEUE_ALBUM_ART_SIZE);

					ui.vertical(|ui| {
						// Info.
						let album_title = Label::new(RichText::new(&album.title).color(BONE));
						ui.add(album_title);
						ui.label(album.release.as_str());
						ui.label(album.runtime.as_str());
					});
				});

				ui.add_space(10.0);
				ui.separator();
				current_album = Some(album)
			//-------------------------------------------------- Album.
			} else if !same_album {
				// FIXME: see above.
				ui.add_space(10.0);
				ui.separator();
				ui.add_space(10.0);

				ui.horizontal(|ui| {
					crate::no_rounding!(ui);
					crate::album_button!(self, album, song.album, ui, ctx, QUEUE_ALBUM_ART_SIZE);

					ui.vertical(|ui| {
						// Info.
						let album_title = Label::new(RichText::new(&album.title).color(BONE));
						ui.add(album_title);
						ui.label(album.release.as_str());
					});
				});

				ui.add_space(10.0);
				ui.separator();
				current_album = Some(album)
			}

			//-------------------------------------------------- Song.
			ui.horizontal(|ui| {
				let width = width / 20.0;
				const HEIGHT: f32 = 35.0;

				// Remove button.
				if ui.add_sized([width, HEIGHT], Button::new("-")).clicked() {
					send!(self.to_kernel, FrontendToKernel::RemoveQueueRange(index..=index));
				}

				let mut rect = ui.cursor();
				rect.max.y = rect.min.y + HEIGHT;
				rect.max.x = rect.min.x + ui.available_width();

				if ui.put(rect, SelectableLabel::new(self.audio_state.queue_idx == Some(index), "")).clicked() {
					crate::play_queue_index!(self, index);
				}


				ui.allocate_ui_at_rect(rect, |ui| {
					ui.horizontal_centered(|ui| {
						ui.add(Label::new(format!("{: >3}    {: >8}    {}", song.track.unwrap_or(0), &song.runtime, &song.title)));
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
