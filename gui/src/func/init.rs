//---------------------------------------------------------------------------------------------------- Use
use crate::constants::{
    APP_HEIGHT_DEFAULT, APP_HEIGHT_MIN, APP_WIDTH_DEFAULT, APP_WIDTH_MIN, FONT_ARRAY,
    RUNTIME_WIDTH, SETTINGS_VERSION, SPACING, STATE_VERSION, VISUALS,
};
use crate::data::{DebugInfo, Settings, Settings0, Settings1, Settings2, State, State0};
use benri::{atomic_store, now, send};
use crossbeam::channel::{Receiver, Sender};
use disk::{Bincode2, Json};
use egui::{FontData, FontDefinitions, FontFamily, FontId, Style, TextStyle, ThemePreference};
use log::{debug, info, warn};
use shukusai::{
    collection::Collection,
    constants::{FESTIVAL_DBUS, FESTIVAL_ICON},
    kernel::{FrontendToKernel, KernelToFrontend},
    state::{AudioState, ResetState},
};
use std::sync::{atomic::AtomicBool, Arc, Mutex};

//---------------------------------------------------------------------------------------------------- GUI Init.
// Instead of having [Gui::new()] be 1000s of lines long,
// these private functions will be separate stuff.
//
// See `Gui::init` at the bottom to see the function that "starts" the `GUI`.
impl crate::data::Gui {
    #[inline(always)]
    fn init_style() -> egui::Style {
        Style {
            text_styles: [
                (TextStyle::Small, FontId::new(10.0, FontFamily::Monospace)),
                (
                    TextStyle::Name("12.5".into()),
                    FontId::new(12.5, FontFamily::Monospace),
                ),
                (
                    TextStyle::Name("15".into()),
                    FontId::new(15.0, FontFamily::Monospace),
                ),
                (TextStyle::Body, FontId::new(20.0, FontFamily::Monospace)),
                (TextStyle::Button, FontId::new(20.0, FontFamily::Monospace)),
                (
                    TextStyle::Monospace,
                    FontId::new(20.0, FontFamily::Monospace),
                ),
                (
                    TextStyle::Name("25".into()),
                    FontId::new(25.0, FontFamily::Monospace),
                ),
                (
                    TextStyle::Name("30".into()),
                    FontId::new(30.0, FontFamily::Monospace),
                ),
                (
                    TextStyle::Name("35".into()),
                    FontId::new(35.0, FontFamily::Monospace),
                ),
                (TextStyle::Heading, FontId::new(40.0, FontFamily::Monospace)),
            ]
            .into(),

            spacing: SPACING.clone(),

            // Smooths out some resizing animations.
            // Default is `1.0 / 12.0` (very fast).
            animation_time: 0.25,

            // This gets rid of the delay for tooltips.
            // (It wants to wait until the pointer is still).
            interaction: egui::style::Interaction {
                show_tooltips_only_when_still: false,
                selectable_labels: false,
                ..Default::default()
            },

            ..Default::default()
        }
    }

    #[inline(always)]
    fn init_visuals() -> egui::Visuals {
        VISUALS.clone()
    }

    #[inline(always)]
    fn init_fonts() -> egui::FontDefinitions {
        let mut fonts = FontDefinitions::default();

        for (font, bytes) in FONT_ARRAY.iter().rev() {
            fonts
                .font_data
                .insert(font.to_string(), FontData::from_static(bytes));

            fonts
                .families
                .get_mut(&FontFamily::Monospace)
                .expect("Failed to get: egui::FontFamily::Monospace")
                .insert(0, font.to_string());
            // ↑: last font in iter (which has index 0 because reverse), gets inserted first.
            fonts
                .families
                .get_mut(&FontFamily::Proportional)
                .expect("Failed to get: egui::FontFamily::Proportional")
                .push(font.to_string());
        }

        fonts
    }

    //---------------------------------------------------------------------------------------------------- `egui/eframe` options & init
    #[inline(always)]
    // Sets the initial options for native rendering with eframe
    pub fn options() -> eframe::NativeOptions {
        // Icon
        // SAFETY: This image is known at compile-time. It should never fail.
        let icon = image::load_from_memory(FESTIVAL_ICON).unwrap().to_rgba8();
        let (width, height) = icon.dimensions();
        let icon_data = Some(Arc::new(egui::IconData {
            rgba: icon.into_raw(),
            width,
            height,
        }));

        // The rest
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder {
                min_inner_size: Some(egui::vec2(APP_WIDTH_MIN, APP_HEIGHT_MIN)),
                inner_size: Some(egui::vec2(APP_WIDTH_DEFAULT, APP_HEIGHT_DEFAULT)),
                drag_and_drop: Some(true),
                app_id: Some(FESTIVAL_DBUS.to_string()),
                icon: icon_data,

                ..Default::default()
            },
            // FIXME:
            // `eframe::Renderer::Wgpu` causes colors to
            // be over-saturated on `KDE`. For now, use
            // `Glow` on Linux (even though `Wgpu` works
            // fine on GNOME).
            //
            // Not changing Windows/macOS off `Wgpu` since it works.
            //
            // https://github.com/hinto-janai/festival/pull/32
            // https://github.com/hinto-janai/festival/pull/33
            // https://github.com/hinto-janai/festival/pull/42
            #[cfg(target_os = "linux")]
            renderer: eframe::Renderer::Glow,
            #[cfg(not(target_os = "linux"))]
            renderer: eframe::Renderer::Wgpu,
            ..Default::default()
        }
    }

    #[inline(always)]
    // This "starts" the `GUI` thread.
    pub fn init(
        cc: &eframe::CreationContext<'_>,
        to_kernel: Sender<FrontendToKernel>,
        from_kernel: Receiver<KernelToFrontend>,
    ) -> Self {
        // Read `Settings` from disk.
        let settings = Settings::from_versions(&[
            (SETTINGS_VERSION, Settings::from_file),
            (2, Settings2::disk_into),
            (1, Settings1::disk_into),
            (0, Settings0::disk_into),
        ]);
        let settings = match settings {
            Ok((v, s)) if v == SETTINGS_VERSION => {
                info!("GUI Init [1/8] ... Settings{SETTINGS_VERSION} from disk");
                s
            }
            Ok((v, s)) => {
                info!("GUI Init [1/8] ... Settings{v} from disk, converted to Settings{SETTINGS_VERSION}");
                s
            }
            Err(e) => {
                warn!("GUI Init [1/8] ... Settings failed from disk: {e}, returning default Settings{SETTINGS_VERSION}");
                Settings::new()
            }
        };
        debug!("Settings{SETTINGS_VERSION}: {settings:#?}");

        cc.egui_ctx.set_theme(ThemePreference::Dark);
        cc.egui_ctx.options_mut(|options| {
            options.line_scroll_speed = 100.0;
        });

        cc.egui_ctx
            .set_pixels_per_point(settings.pixels_per_point as f32);
        atomic_store!(
            shukusai::audio::PREVIOUS_THRESHOLD,
            settings.previous_threshold
        );

        // Send `CachePath` signal to `Kernel`.
        if settings.collection_paths.is_empty() {
            match dirs::audio_dir() {
                Some(p) => {
                    debug!(
                        "GUI - collection_paths.is_empty(), using dir::audio_dir() for CachePath"
                    );
                    send!(to_kernel, FrontendToKernel::CachePath(vec![p]));
                }
                None => warn!("GUI - dirs::audio_dir() failed, can't send CachePath message"),
            }
        } else {
            send!(
                to_kernel,
                FrontendToKernel::CachePath(settings.collection_paths.clone())
            );
        }

        // Read `State` from disk.
        let state =
            State::from_versions(&[(STATE_VERSION, State::from_file), (0, State0::disk_into)]);
        let state = match state {
            Ok((v, s)) if v == STATE_VERSION => {
                info!("GUI Init [1/8] ... State{STATE_VERSION} from disk");
                s
            }
            Ok((v, s)) => {
                info!("GUI Init [1/8] ... State{v} from disk, converted to State{STATE_VERSION}");
                s
            }
            Err(e) => {
                warn!("GUI Init [1/8] ... State failed from disk: {e}, returning default State{STATE_VERSION}");
                State::new()
            }
        };
        debug!("State{STATE_VERSION}: {state:#?}");

        // Send signal to `Kernel` for `AudioState` if set.
        if settings.restore_state {
            info!("GUI Init [3/8] ... Restoring AudioState");
            send!(to_kernel, FrontendToKernel::RestoreAudioState);
        } else {
            info!("GUI Init [3/8] ... Skipping AudioState");
        }

        // Style
        cc.egui_ctx.set_style(Self::init_style());
        info!("GUI Init [5/8] ... Style");

        // Visuals
        cc.egui_ctx.set_visuals(Self::init_visuals());
        info!("GUI Init [6/8] ... Visuals");

        // Fonts
        cc.egui_ctx.set_fonts(Self::init_fonts());
        info!("GUI Init [7/8] ... Fonts");

        // Done.
        info!("GUI Init [8/8] ... Init");
        Self {
            // `Kernel` channels.
            to_kernel,
            from_kernel,

            // `shukusai` data.
            collection: Collection::dummy(),

            // `GUI` settings.
            og_settings: settings.clone(),
            settings,

            // `GUI` state.
            og_state: state.clone(),
            state,
            state_restore: Default::default(),

            // AudioState.
            audio_state: AudioState::new(),
            queue_time: readable::Time::zero(),
            audio_seek: 0,
            audio_leeway: now!(),
            last_song: None,
            runtime_width: RUNTIME_WIDTH,
            auto_save: now!(),

            reset_state: ResetState::new(),

            rect: egui::Rect {
                min: Default::default(),
                max: Default::default(),
            },
            resize_leeway: now!(),

            // `egui_notify`
            toasts: egui_notify::Toasts::new(),

            // `rfd`.
            rfd_open: Arc::new(AtomicBool::new(false)),
            rfd_new: Arc::new(Mutex::new(None)),
            deleted_paths: vec![],

            // Search state.
            searching: false,
            search_jump: false,

            og_playlists: shukusai::state::Playlists::new(),
            playlist_clone: None,
            playlist_remove: None,
            playlist_from: None,
            playlist_to: None,
            playlist_swap_entry: None,
            playlist_remove_entry: None,
            playlist_name_edit_enter: false,
            playlist_add_screen: None,
            playlist_add_screen_result: None,

            // Local cache.
            count_artist: "Artists: 0".to_string(),
            count_album: "Albums: 0".to_string(),
            count_song: "Songs: 0".to_string(),
            count_art: "Art: 0".to_string(),

            exiting: false,
            exit_instant: now!(),

            resetting_collection: false,
            kernel_returned: false,

            debug_screen: false,
            debug_info: DebugInfo::new(),

            modifiers: Default::default(),
        }
    }
}
