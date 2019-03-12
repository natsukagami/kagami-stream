use serenity::command;

use crate::MpdManager;

command!(np(ctx, msg, _args) {
	let mut data = ctx.data.lock();
	let mpd = data.get_mut::<MpdManager>().expect("mpd client where??");

	let status = mpd.currentsong()?;

	msg.reply(&match status {
		None => "Nothing is playing".to_owned(),
		Some(song) => 
			format!(
				"{} - {}", 
				song.title.unwrap_or("Unknown title".to_owned()), 
				song.tags
					.get("Artist")
					.cloned()
					.unwrap_or("Unknown artist".to_owned()))
	})?;
});
