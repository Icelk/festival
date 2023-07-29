mod base64;
mod cert;
mod cli;
mod config;
mod constants;
mod hash;
mod macros;
mod ptr;
mod statics;
mod rest;
mod resp;
mod rpc;
mod router;
mod zip;

fn main() {
	// Handle CLI arguments.
	let (disable_watch, disable_media_controls, log) = {
		if std::env::args_os().len() == 1 {
			(false, false, log::LevelFilter::Info)
		} else {
			crate::cli::Cli::get()
		}
	};

	// Init logger.
	shukusai::logger::init_logger(log);

	// Set `umask` (`rwxr-x---`)
	disk::umask(0o027);

	// Setup `Kernel` <-> `Frontend` channels.
	let (to_kernel, from_kernel) = match shukusai::kernel::Kernel::spawn(!disable_watch, !disable_media_controls) {
		Ok((t, f)) => (t, f),
		Err(e)     => panic!("Kernel::spawn() failed: {e}"),
	};

	// These last forever.
	// INVARIANT: Initialize `CONFIG`. This must be set, and once only.
	let CONFIG:      &'static crate::config::Config = crate::config::ConfigBuilder::file_or().build_and_set();
	let TO_KERNEL:   &'static crossbeam::channel::Sender<shukusai::kernel::FrontendToKernel>   = Box::leak(Box::new(to_kernel));
	let FROM_KERNEL: &'static crossbeam::channel::Receiver<shukusai::kernel::KernelToFrontend> = Box::leak(Box::new(from_kernel));

	// Start HTTP router.
	crate::router::init(CONFIG, TO_KERNEL, FROM_KERNEL);
}
