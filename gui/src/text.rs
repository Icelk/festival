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
pub const UI_PLAY:         &str = "▶";
pub const UI_PAUSE:        &str = "⏸";
pub const UI_PREVIOUS:     &str = "⏪";
pub const UI_FORWARDS:     &str = "⏩";
pub const UI_REPEAT_SONG:  &str = "🔂";
pub const UI_REPEAT:       &str = "🔁";

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
pub const EMPTY_COLLECTION: &str =
r#"This scans the system's Music directory.

Configure which directories to scan in the [Settings] tab."#;

//---------------------------------------------------------------------------------------------------- Songs tab
pub const OPEN_PARENT_FOLDER: &str = "Open the directory containing this song";

//---------------------------------------------------------------------------------------------------- Queue tab
pub const UI_QUEUE_CLEAR:   &str = "⏹";
pub const UI_QUEUE_SHUFFLE: &str = "🔀";
pub const QUEUE_CLEAR:      &str = "Clear queue and stop playback";
pub const QUEUE_SHUFFLE:    &str = "Shuffle the queue and reset to the first song";

//---------------------------------------------------------------------------------------------------- Settings Tab
pub const RESET:             &str = formatcp!("Reset changes ({MOD}+Z)");
pub const SAVE:              &str = formatcp!("Save changes to disk ({MOD}+S)");
pub const SHUFFLE_MODE:      &str = "Which method to shuffle songs by";
pub const ARTIST_SORT_ORDER: &str = formatcp!("Which method to sort the artists by in the [Artists] tab ({MOD}+Q)");
pub const ALBUM_SORT_ORDER:  &str = formatcp!("Which method to sort the albums by in the [Albums] tab ({MOD}+W)");
pub const SONG_SORT_ORDER:   &str = formatcp!("Which method to sort the songs by in the [Songs] tab ({MOD}+E)");
pub const SEARCH_KIND:       &str = "Which type of search to use in the [Search] tab";
pub const SEARCH_SORT:       &str = "Which sub-tab to use in the [Search] tab";
pub const ALBUM_ART_SIZE:    &str = "How big the album art cover should be in the [Albums] tab";
pub const STATIC_PIXEL_SIZE: &str = formatcp!(
	"Always show album art at a static pixel size regardless of the window size ({}-{})",
	ALBUM_ART_SIZE_MIN as usize,
	ALBUM_ART_SIZE_MAX as usize,
);
pub const ALBUM_PER_ROW:     &str = formatcp!("Show [x] amount of albums per row, scaling the pixel size to fit ({ALBUMS_PER_ROW_MIN}-{ALBUMS_PER_ROW_MAX})");
pub const RESTORE_STATE:     &str = "Restore playback state from the last session when opening Festival";
pub const WINDOW_TITLE:      &str = "Set Festival's window title when changing songs";
pub const ACCENT_COLOR:      &str = formatcp!(
	"Which accent color to use (default: [{}, {}, {}])",
	ACCENT_COLOR_RGB[0],
	ACCENT_COLOR_RGB[1],
	ACCENT_COLOR_RGB[2],
);
pub const COLLECTION:        &str = "The main music Collection that stores all metadata about the audio files";
pub const ADD_FOLDER:        &str = formatcp!("Add a maximum of 10 folders to scan for the Collection ({MOD}+A)");
pub const REMOVE_FOLDER:     &str = "Remove this folder";
pub const RESET_COLLECTION:  &str = formatcp!(
r#"Scan the folders listed and create a new Collection ({MOD}+R).

If no directories are listed, the default Music directory is scanned."#);
pub const EMPTY_AUTOPLAY:    &str = "Start playing automatically if songs are added to an empty queue";
pub const STATS:             &str = "Stats about your current Collection";

#[cfg(not(target_os = "macos"))]
pub const HELP: &str =
r#"*-------------------------------------------------------*
|       Key/Mouse | Action                              |
|-------------------------------------------------------|
|     [A-Za-z0-9] | Jump to search tab                  |
|          CTRL+S | Save Settings                       |
|          CTRL+Z | Reset Settings                      |
|          CTRL+R | Reset Collection                    |
|          CTRL+A | Add Scan Directory                  |
|          CTRL+Q | Rotate Album Sort                   |
|          CTRL+W | Rotate Artist Sort                  |
|          CTRL+E | Rotate Song Sort                    |
|          CTRL+D | Goto Last Tab                       |
|              Up | Last Tab                            |
|            Down | Next Tab                            |
|           Right | Last Sub-Tab                        |
|            Left | Last Sub-Tab                        |
|   Primary Mouse | Set Artist, Album, Song             |
| Secondary Mouse | Append Artist, Album, Song to Queue |
|    Middle Mouse | Open Album/Song Directory           |
*-------------------------------------------------------*"#;

// macOS doesn't have a middle click on the trackpad natively...
#[cfg(target_os = "macos")]
pub const HELP: &str =
r#"*-------------------------------------------------------*
|       Key/Mouse | Action                              |
|-------------------------------------------------------|
|     [A-Za-z0-9] | Jump to search tab                  |
|       Command+S | Save Settings                       |
|       Command+Z | Reset Settings                      |
|       Command+R | Reset Collection                    |
|       Command+A | Add Scan Directory                  |
|       Command+Q | Rotate Album Sort                   |
|       Command+W | Rotate Artist Sort                  |
|       Command+E | Rotate Song Sort                    |
|       Command+D | Goto Last Tab                       |
|              Up | Last Tab                            |
|            Down | Next Tab                            |
|           Right | Last Sub-Tab                        |
|            Left | Last Sub-Tab                        |
|   Primary Mouse | Set Artist, Album, Song             |
| Secondary Mouse | Append Artist, Album, Song to Queue |
| Command+Primary | Open Album/Song Directory           |
*-------------------------------------------------------*"#;

//---------------------------------------------------------------------------------------------------- Search Tab
// This is inaccurate because `char` != `u8` but meh.
pub const SEARCH_MAX:              &str = formatcp!("Search character limit has been reached ({SEARCH_MAX_LEN})");
pub const SEARCH_BAR:              &str = "Search for albums, artists, and songs.\nYou can start typing from anywhere in Festival to start searching.";
pub const SEARCH_HELP:             &str = "🔍 Search for albums, artists, and songs.";
pub const SEARCH_EMPTY_COLLECTION: &str = "The Collection is empty. There is nothing to search.";
pub const SEARCH_SORT_SONG:        &str = "Search by song title";
pub const SEARCH_SORT_ALBUM:       &str = "Search by album title";
pub const SEARCH_SORT_ARTIST:      &str = "Search by artist name";

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
