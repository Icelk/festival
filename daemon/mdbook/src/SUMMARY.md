# Summary

[Intro](intro.md)

* [Quick Start](quick-start.md)
* [Config](config.md)
* [Disk](disk.md)
* [Media Controls](media-controls.md)
* [API Stability](api-stability.md)
* [Tor](tor.md)
* [Authorization](authorization/authorization.md)
	- [JSON-RPC](authorization/json-rpc.md)
	- [REST](authorization/rest.md)
	- [Documentation](authorization/documentation.md)
* [Command Line](command-line/command-line.md)
	- [Top-level flags](command-line/flags.md)
	- [Sub-command: `signal`](command-line/signal.md)
* [Common Objects](common-objects/common-objects.md)
	- [Collection](common-objects/collection.md)
	- [Key](common-objects/key.md)
	- [Artist](common-objects/artist.md)
	- [Album](common-objects/album.md)
	- [Song](common-objects/song.md)
* [JSON-RPC](json-rpc/json-rpc.md)
	- [Quick Start](json-rpc/quick-start.md)
	- [Collection](json-rpc/collection/collection.md)
		- [collection_new](json-rpc/collection/collection_new.md)
		- [collection_brief](json-rpc/collection/collection_brief.md)
		- [collection_full](json-rpc/collection/collection_full.md)
		- [collection_relation](json-rpc/collection/collection_relation.md)
		- [collection_relation_full](json-rpc/collection/collection_relation_full.md)
		- [collection_perf](json-rpc/collection/collection_perf.md)
		- [collection_resource_size](json-rpc/collection/collection_resource_size.md)
	- [State](json-rpc/state/state.md)
		- [state_daemon](json-rpc/state/state_daemon.md)
		- [state_audio](json-rpc/state/state_audio.md)
		- [state_reset](json-rpc/state/state_reset.md)
		- [state_config](json-rpc/state/state_config.md)
		- [state_ip](json-rpc/state/state_ip.md)
	- [Key](json-rpc/key/key.md)
		- [key_artist](json-rpc/key/key_artist.md)
		- [key_album](json-rpc/key/key_album.md)
		- [key_song](json-rpc/key/key_song.md)
	- [Map](json-rpc/map/map.md)
		- [map_artist](json-rpc/map/map_artist.md)
		- [map_album](json-rpc/map/map_album.md)
		- [map_song](json-rpc/map/map_song.md)
	- [Current](json-rpc/current/current.md)
		- [current_artist](json-rpc/current/current_artist.md)
		- [current_album](json-rpc/current/current_album.md)
		- [current_song](json-rpc/current/current_song.md)
	- [Rand](json-rpc/rand/rand.md)
		- [rand_artist](json-rpc/rand/rand_artist.md)
		- [rand_album](json-rpc/rand/rand_album.md)
		- [rand_song](json-rpc/rand/rand_song.md)
	- [Search](json-rpc/search/index.md)
		- [search](json-rpc/search/search.md)
		- [search_artist](json-rpc/search/search_artist.md)
		- [search_album](json-rpc/search/search_album.md)
		- [search_song](json-rpc/search/search_song.md)
	- [Playback](json-rpc/playback/playback.md)
		- [toggle](json-rpc/playback/toggle.md)
		- [play](json-rpc/playback/play.md)
		- [pause](json-rpc/playback/pause.md)
		- [next](json-rpc/playback/next.md)
		- [stop](json-rpc/playback/stop.md)
		- [shuffle](json-rpc/playback/shuffle.md)
		- [repeat_off](json-rpc/playback/repeat_off.md)
		- [repeat_song](json-rpc/playback/repeat_song.md)
		- [repeat_queue](json-rpc/playback/repeat_queue.md)
		- [previous](json-rpc/playback/previous.md)
		- [volume](json-rpc/playback/volume.md)
		- [clear](json-rpc/playback/clear.md)
		- [seek](json-rpc/playback/seek.md)
		- [skip](json-rpc/playback/skip.md)
		- [back](json-rpc/playback/back.md)
	- [Queue](json-rpc/queue/queue.md)
		- [queue_add_key_artist](json-rpc/queue/queue_add_key_artist.md)
		- [queue_add_key_album](json-rpc/queue/queue_add_key_album.md)
		- [queue_add_key_song](json-rpc/queue/queue_add_key_song.md)
		- [queue_add_map_artist](json-rpc/queue/queue_add_map_artist.md)
		- [queue_add_map_album](json-rpc/queue/queue_add_map_album.md)
		- [queue_add_map_song](json-rpc/queue/queue_add_map_song.md)
		- [queue_add_rand_artist](json-rpc/queue/queue_add_rand_artist.md)
		- [queue_add_rand_album](json-rpc/queue/queue_add_rand_album.md)
		- [queue_add_rand_song](json-rpc/queue/queue_add_rand_song.md)
		- [queue_add_playlist](json-rpc/queue/queue_add_playlist.md)
		- [queue_set_index](json-rpc/queue/queue_set_index.md)
		- [queue_remove_range](json-rpc/queue/queue_remove_range.md)
	- [Playlist](json-rpc/playlist/playlist.md)
		- [playlist_new](json-rpc/playlist/playlist_new.md)
		- [playlist_remove](json-rpc/playlist/playlist_remove.md)
		- [playlist_clone](json-rpc/playlist/playlist_clone.md)
		- [playlist_remove_song](json-rpc/playlist/playlist_remove_song.md)
		- [playlist_add_key_artist](json-rpc/playlist/playlist_add_key_artist.md)
		- [playlist_add_key_album](json-rpc/playlist/playlist_add_key_album.md)
		- [playlist_add_key_song](json-rpc/playlist/playlist_add_key_song.md)
		- [playlist_add_map_artist](json-rpc/playlist/playlist_add_map_artist.md)
		- [playlist_add_map_album](json-rpc/playlist/playlist_add_map_album.md)
		- [playlist_add_map_song](json-rpc/playlist/playlist_add_map_song.md)
		- [playlist_names](json-rpc/playlist/playlist_names.md)
		- [playlist_count](json-rpc/playlist/playlist_count.md)
		- [playlist_single](json-rpc/playlist/playlist_single.md)
		- [playlist_all](json-rpc/playlist/playlist_all.md)
* [REST](rest/rest.md)
	- [Quick Start](rest/quick-start.md)
	- [/key](rest/key/key.md)
		- [/key/artist/$ARTIST_KEY](rest/key/artist.md)
		- [/key/album/$ALBUM_KEY](rest/key/album.md)
		- [/key/song/$SONG_KEY](rest/key/song.md)
		- [/key/art/$ALBUM_KEY](rest/key/art.md)
	- [/map](rest/map/map.md)
		- [/map/$ARTIST_NAME](rest/map/artist.md)
		- [/map/$ARTIST_NAME/$ALBUM_TITLE](rest/map/album.md)
		- [/map/$ARTIST_NAME/$ALBUM_TITLE/$SONG_TITLE](rest/map/song.md)
	- [/current](rest/current/current.md)
		- [/current/artist](rest/current/artist.md)
		- [/current/album](rest/current/album.md)
		- [/current/song](rest/current/song.md)
		- [/current/art](rest/current/art.md)
	- [/rand](rest/rand/rand.md)
		- [/rand/artist](rest/rand/artist.md)
		- [/rand/album](rest/rand/album.md)
		- [/rand/song](rest/rand/song.md)
		- [/rand/art](rest/rand/art.md)
	- [/art](rest/art/art.md)
		- [/art/$ARTIST_NAME](rest/art/artist.md)
		- [/art/$ARTIST_NAME/$ALBUM_TITLE](rest/art/album.md)
	- [/collection](rest/collection.md)