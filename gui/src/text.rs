// This file contains most of the static text that
// is used for the widget tooltips via `.on_hover_text()`.
//
// Some of the text is actually responsible for the UI,
// using either emojis or unicode, e.g the play button: "▶".

//---------------------------------------------------------------------------------------------------- Use
use crate::constants::{
	ACCENT_COLOR_RGB,
	ALBUMS_PER_ROW_MIN,
	ALBUMS_PER_ROW_MAX,
	ALBUM_ART_SIZE_MIN,
	ALBUM_ART_SIZE_MAX,
	SEARCH_MAX_LEN,
	PIXELS_PER_POINT_UNIT_STR,
	PIXELS_PER_POINT_MIN_STR,
	PIXELS_PER_POINT_MAX_STR,
	STATE_VERSION,
	SETTINGS_VERSION,
	FESTIVAL_NAME_VER,
};
use shukusai::constants::{
	COLLECTION_VERSION,
	AUDIO_VERSION,
	OS_ARCH,
	COMMIT,
	SHUKUSAI_NAME_VER,
};
use const_format::formatcp;

//---------------------------------------------------------------------------------------------------- Platform
#[cfg(not(target_os = "macos"))]
pub const MOD: &str = "CTRL";
#[cfg(target_os = "macos")]
pub const MOD: &str = "⌘";

//---------------------------------------------------------------------------------------------------- Collection State
pub const COLLECTION_LOADING:   &str = "Loading Collection";
pub const COLLECTION_RESETTING: &str = "Resetting Collection";

//---------------------------------------------------------------------------------------------------- UI
pub const UI_PLAY:         &str = "⏵";
pub const UI_PAUSE:        &str = "⏸";
pub const UI_PREVIOUS:     &str = "⏪";
pub const UI_FORWARDS:     &str = "⏩";
pub const UI_REPEAT_SONG:  &str = "🔂";
pub const UI_REPEAT:       &str = "🔁";
pub const SELECT_ARTIST:   &str = "🗋 Select an artist by clicking any artist name";
pub const SELECT_ALBUM:    &str = "🗋 Select an album by clicking any album art";
pub const SELECT_QUEUE:    &str = "🗋 Add any song/album/artist to the queue by right-clicking";
pub const SELECT_PLAYLIST: &str = "🗋 Select any playlist by clicking any playlist name";
pub const UI_PLUS:         &str = "➕";
pub const UI_MINUS:        &str = "➖";
pub const UI_X:            &str = "❌";
pub const UI_UP:           &str = "⬆";
pub const UI_DOWN:         &str = "⬇";

//---------------------------------------------------------------------------------------------------- Left Tab
pub const INCREMENT_ALBUM_SIZE: &str = "Increase the album art size";
pub const DECREMENT_ALBUM_SIZE: &str = "Decrease the album art size";
pub const VOLUME_SLIDER:        &str = "Increase/decrease audio volume";
pub const SHUFFLE_OFF:          &str = "Shuffle is turned off";
pub const REPEAT_SONG:          &str = "The current song will be repeated forever";
pub const REPEAT_QUEUE:         &str = "The current queue will be repeated forever";
pub const REPEAT_OFF:           &str = "Repeat is turned off";

//---------------------------------------------------------------------------------------------------- Bottom Bar
pub const SAVING: &str = "Festival is still saving a recently created Collection";

//---------------------------------------------------------------------------------------------------- Albums tab
pub const ARTIST_TOTAL_ALBUM:   &str = "Total album count";
pub const ARTIST_TOTAL_SONG:    &str = "Total song count";
pub const ARTIST_TOTAL_RUNTIME: &str = "Total runtime";

//---------------------------------------------------------------------------------------------------- Albums tab
pub const EMPTY_COLLECTION: &str =
r#"This scans the system's Music directory by default.

Configure which directories to scan in the [Settings] tab."#;

//---------------------------------------------------------------------------------------------------- Songs tab
pub const OPEN_PARENT_FOLDER: &str = "Open the directory containing this song";

//---------------------------------------------------------------------------------------------------- Queue tab
pub const UI_QUEUE_CLEAR:   &str = "⏹";
pub const UI_QUEUE_SHUFFLE: &str = "🔀";
pub const QUEUE_CLEAR:      &str = "Clear queue and stop playback";
pub const QUEUE_SHUFFLE:    &str = "Shuffle the queue and reset to the first song";

//---------------------------------------------------------------------------------------------------- Playlists Tab
pub const PLAYLIST_TEXT_EMPTY:    &str = "Playlist name is empty";
pub const PLAYLIST_TEXT:          &str = "Create a playlist with this name";
pub const PLAYLIST_CREATE:        &str = "Create this playlist";
pub const PLAYLIST_EXISTS:        &str = "A playlist with this name already exists";
pub const PLAYLIST_DELETE:        &str = "Delete this playlist";
pub const PLAYLIST_EDIT:          &str = "Edit this playlist's name";
pub const PLAYLIST_EDIT_SAVE:     &str = "Save this playlist with the new name";
pub const PLAYLIST_COPY:          &str = "Create a copy of this playlist";
pub const PLAYLIST_COUNT:         &str = "Total amount of playlists";
pub const PLAYLIST_TOTAL_SONG:    &str = "Total playlist song count";
pub const PLAYLIST_TOTAL_RUNTIME: &str = "Total playlist runtime";
pub const PLAYLIST_ENTRY_DELETE:  &str = "Delete this entry";
pub const PLAYLIST_ENTRY_UP:      &str = "Move this entry up";
pub const PLAYLIST_ENTRY_DOWN:    &str = "Move this entry down";
pub const PLAYLIST_INVALID: &str =
r#"This song is invalid (it does not exist within the current Collection).

This song & its runtime do not count towards the playlist's total stats.

If added to the queue, only valid songs will be added.

Festival will continue to hold onto this entry in this playlist indefinitely
and it will automatically recover to a valid entry if a matching song is found
after a Collection reset."#;

//---------------------------------------------------------------------------------------------------- Settings Tab
pub const RESET:             &str = formatcp!("Reset changes ({MOD}+Z)");
pub const SAVE:              &str = formatcp!("Save changes to disk ({MOD}+S)");
pub const SHUFFLE_MODE:      &str = "Which method to shuffle songs by";
pub const ARTIST_SORT_ORDER: &str = formatcp!("Which method to sort the artists by in the [Artists] tab ({MOD}+W)");
pub const ALBUM_SORT_ORDER:  &str = formatcp!("Which method to sort the albums by in the [Albums] tab ({MOD}+E)");
pub const SONG_SORT_ORDER:   &str = formatcp!("Which method to sort the songs by in the [Songs] tab ({MOD}+R)");
pub const SEARCH_KIND:       &str = "Which type of search to use in the [Search] tab";
pub const SEARCH_SORT:       &str = "Which sub-tab to use in the [Search] tab";
pub const ALBUM_ART_SIZE:    &str = "How big album art should be in the [Albums] tab";
pub const STATIC_PIXEL_SIZE: &str = formatcp!(
	"Always show album art at a static pixel size regardless of the window size ({}-{})",
	ALBUM_ART_SIZE_MIN as usize,
	ALBUM_ART_SIZE_MAX as usize,
);
pub const ALBUM_PER_ROW:     &str = formatcp!("Show [x] amount of albums per row, scaling the pixel size to fit ({ALBUMS_PER_ROW_MIN}-{ALBUMS_PER_ROW_MAX})");
pub const PREVIOUS_THRESHOLD:   &str =
r#"If the current song runtime has passed this number, the [Previous] button will reset the current song instead of skipping backwards.

Setting this to [0] will make the [Previous] button always go to the previous song."#;
pub const RESTORE_STATE:     &str = "Restore playback state from the last session when opening Festival";
pub const WINDOW_TITLE:      &str = "Set Festival's window title when changing songs";
pub const ACCENT_COLOR:      &str = formatcp!(
	"Which accent color to use (default: [{}, {}, {}])",
	ACCENT_COLOR_RGB[0],
	ACCENT_COLOR_RGB[1],
	ACCENT_COLOR_RGB[2],
);
pub const PIXELS_PER_POINT:  &str = formatcp!(
r#"Manually scale UI pixel size ({PIXELS_PER_POINT_MIN_STR}-{PIXELS_PER_POINT_MAX_STR})

Warning: using a custom pixel size may lead to improperly sized UI."#);
pub const PIXELS_PER_POINT_ADD: &str = formatcp!("Increase by {PIXELS_PER_POINT_UNIT_STR}");
pub const PIXELS_PER_POINT_SUB: &str = formatcp!("Decrease by {PIXELS_PER_POINT_UNIT_STR}");
pub const COLLECTION:        &str = "Festival's music Collection that stores all metadata about the audio files";
pub const ADD_FOLDER:        &str = formatcp!(
r#"Add a maximum of 10 folders to scan for the Collection ({MOD}+A).

You can also add folders by drag-and-dropping them anywhere into Festival.

It's highly recommended to only add folders with music and art files to increase scanning speed."#);
pub const REMOVE_FOLDER:     &str = "Remove this folder";
pub const RESET_COLLECTION:  &str = formatcp!(
r#"Scan the folders listed and create a new Collection ({MOD}+C).

If no folders are listed, the default Music directory is scanned."#);
pub const EMPTY_AUTOPLAY:    &str = "Start playing automatically if songs are added to an empty queue";
pub const STATS:             &str = "Stats about your current Collection";

#[cfg(not(target_os = "macos"))]
pub const HELP: &str =
r#"*-------------------------------------------------------*
|       Key/Mouse | Action                              |
|-------------------------------------------------------|
|     [A-Za-z0-9] | Jump to search tab                  |
|          CTRL+S | Save Changes                        |
|          CTRL+Z | Reset Changes                       |
|          CTRL+C | Reset Collection                    |
|          CTRL+A | Add Scan Directory                  |
|          CTRL+W | Rotate Album Sort                   |
|          CTRL+E | Rotate Artist Sort                  |
|          CTRL+R | Rotate Song Sort                    |
|          CTRL+D | Goto Last Tab                       |
|              Up | Last Tab                            |
|            Down | Next Tab                            |
|           Right | Last Sub-Tab                        |
|            Left | Next Sub-Tab                        |
|   Primary Mouse | Set Artist, Album, Song             |
| Secondary Mouse | Add Artist, Album, Song to Queue    |
|    CTRL+Primary | Add Artist, Album, Song to Playlist |
|  CTRL+Secondary | Open Album/Song Directory           |
*-------------------------------------------------------*"#;

// macOS doesn't have a middle click on the trackpad natively...
#[cfg(target_os = "macos")]
pub const HELP: &str =
r#"*---------------------------------------------------------*
|         Key/Mouse | Action                              |
|---------------------------------------------------------|
|       [A-Za-z0-9] | Jump to search tab                  |
|         Command+S | Save Changes                        |
|         Command+Z | Reset Changes                       |
|         Command+C | Reset Collection                    |
|         Command+A | Add Scan Directory                  |
|         Command+W | Rotate Album Sort                   |
|         Command+E | Rotate Artist Sort                  |
|         Command+R | Rotate Song Sort                    |
|         Command+D | Goto Last Tab                       |
|                Up | Last Tab                            |
|              Down | Next Tab                            |
|             Right | Last Sub-Tab                        |
|              Left | Next Sub-Tab                        |
|     Primary Mouse | Set Artist, Album, Song             |
|   Secondary Mouse | Add Artist, Album, Song to Queue    |
|   Command+Primary | Add Artist, Album, Song to Playlist |
| Command+Secondary | Open Album/Song Directory           |
*-------------------------------------------------------*"#;

/// - Festival name + version
/// - OS + Arch
/// - Git commit hash
/// - Binary struct versions
pub const FESTIVAL_SHUKUSAI_COMMIT: &str = {
	use crate::constants::FESTIVAL_NAME_VER;

	use shukusai::constants::{
		COMMIT,
	};

	formatcp!(
r#"{FESTIVAL_NAME_VER}
{OS_ARCH}
Collection v{COLLECTION_VERSION}
Audio v{AUDIO_VERSION}
State v{STATE_VERSION}
Settings v{SETTINGS_VERSION}
{COMMIT}
"#)
};

//---------------------------------------------------------------------------------------------------- Search Tab
// This is inaccurate because `char` != `u8` but meh.
pub const SEARCH_MAX:              &str = formatcp!("Search character limit has been reached ({SEARCH_MAX_LEN})");
pub const SEARCH_BAR:              &str = "Search for albums, artists, and songs.\nYou can start typing from anywhere in Festival to start searching.";
pub const SEARCH_HELP:             &str = "🔍 Search for albums, artists, and songs.";
pub const SEARCH_EMPTY_COLLECTION: &str = "The Collection is empty. There is nothing to search.";
pub const SEARCH_SORT_SONG:        &str = "Search by song title";
pub const SEARCH_SORT_ALBUM:       &str = "Search by album title";
pub const SEARCH_SORT_ARTIST:      &str = "Search by artist name";

//---------------------------------------------------------------------------------------------------- Misc
pub const DRAG_AND_DROP: &str =
r#"🗁

Or drag and drop a folder anywhere into

Festival to start scanning it."#;

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
