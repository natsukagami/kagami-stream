extern crate mpd;
extern crate serde;
extern crate serde_json;
extern crate serenity;

mod commands;
mod config;
mod error;

use config::Config;

use serenity::{
    client::bridge::gateway::{ShardId, ShardManager},
    framework::standard::{
        help_commands, Args, CommandOptions, DispatchError, HelpBehaviour, StandardFramework,
    },
    model::{channel::Message, gateway::Ready, Permissions},
    prelude::*,
    utils::{content_safe, ContentSafeOptions},
};

/**
 * Manages the configuration file.
 */
pub struct ConfigManager;

impl TypeMapKey for ConfigManager {
    type Value = Config;
}

/**
 * Manages the mpd client.
 */
pub struct MpdManager;

impl TypeMapKey for MpdManager {
    type Value = mpd::Client;
}

/**
 * Ready! event
 */
struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

/**
 * The bot will read some environment variables
 * and act upon them.
 */

fn main() {
    println!("Hello, world!");
}
