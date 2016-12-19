use std::collections::HashMap;
use std::sync::mpsc::Sender as MpscSender;
use super::{Handler, Target};
use ::gateway::GatewayStatus;
use ::model::{ChannelId, GuildId, UserId};

/// A manager is a struct responsible for managing [`Handler`]s which belong to
/// a single [WebSocket connection]. This is a fairly complex key-value store,
/// with a bit of extra utility for easily joining a "target".
///
/// The "target" used by the Manager is determined based on the `guild_id` and
/// `channel_id` provided. If a `guild_id` is _not_ provided to methods that
/// optionally require it, then the target is a group or 1-on-1 call with a
/// user. The `channel_id` is then used as the target.
///
/// If a `guild_id` is provided, then the target is the guild, as a user
/// can not be connected to two channels within one guild simultaneously.
///
/// [`Group`]: ../../model/struct.Group.html
/// [`Handler`]: struct.Handler.html
/// [guild's channel]: ../../model/enum.ChannelType.html#variant.Voice
/// [WebSocket connection]: ../../client/struct.Connection.html
pub struct Manager {
    handlers: HashMap<Target, Handler>,
    user_id: UserId,
    ws: MpscSender<GatewayStatus>,
}

impl Manager {
    #[doc(hidden)]
    pub fn new(ws: MpscSender<GatewayStatus>, user_id: UserId) -> Manager {
        Manager {
            handlers: HashMap::new(),
            user_id: user_id,
            ws: ws,
        }
    }

    /// Retrieves a mutable handler for the given target, if one exists.
    pub fn get<T: Into<Target>>(&mut self, target_id: T)
        -> Option<&mut Handler> {
        self.handlers.get_mut(&target_id.into())
    }

    /// Connects to a target by retrieving its relevant [`Handler`] and
    /// connecting, or creating the handler if required.
    ///
    /// This can also switch to the given channel, if a handler already exists
    /// for the target and the current connected channel is not equal to the
    /// given channel.
    ///
    /// In the case of channel targets, the same channel is used to connect to.
    ///
    /// In the case of guilds, the provided channel is used to connect to. The
    /// channel _must_ be in the provided guild. This is _not_ checked by the
    /// library, and will result in an error. If there is already a connected
    /// handler for the guild, _and_ the provided channel is different from the
    /// channel that the connection is already connected to, then the handler
    /// will switch the connection to the provided channel.
    ///
    /// If you _only_ need to retrieve the handler for a target, then use
    /// [`get`].
    ///
    /// [`Handler`]: struct.Handler.html
    /// [`get`]: #method.get
    #[allow(map_entry)]
    pub fn join(&mut self, guild_id: Option<GuildId>, channel_id: ChannelId)
        -> &mut Handler {
        if let Some(guild_id) = guild_id {
            let target = Target::Guild(guild_id);

            {
                let mut found = false;

                if let Some(handler) = self.handlers.get_mut(&target) {
                    handler.switch_to(channel_id);

                    found = true;
                }

                if found {
                    // Actually safe, as the key has already been found above.
                    return self.handlers.get_mut(&target).unwrap();
                }
            }

            let mut handler = Handler::new(target, self.ws.clone(), self.user_id);
            handler.join(channel_id);

            self.handlers.insert(target, handler);

            // Actually safe, as the key would have been inserted above.
            self.handlers.get_mut(&target).unwrap()
        } else {
            let target = Target::Channel(channel_id);

            if !self.handlers.contains_key(&target) {
                let mut handler = Handler::new(target, self.ws.clone(), self.user_id);
                handler.join(channel_id);

                self.handlers.insert(target, handler);
            }

            // Actually safe, as the key would have been inserted above.
            self.handlers.get_mut(&target).unwrap()
        }
    }

    /// Retrieves the [handler][`Handler`] for the given target and leaves the
    /// associated voice channel, if connected.
    ///
    /// This will _not_ drop the handler, and will preserve it and its settings.
    ///
    /// This is a wrapper around [getting][`get`] a handler and calling
    /// [`leave`] on it.
    ///
    /// [`Handler`]: struct.Handler.html
    /// [`get`]: #method.get
    /// [`leave`]: struct.Handler.html#method.leave
    pub fn leave<T: Into<Target>>(&mut self, target_id: T) {
        let target = target_id.into();

        if let Some(handler) = self.handlers.get_mut(&target) {
            handler.leave();
        }
    }

    /// Retrieves the [`Handler`] for the given target and leaves the associated
    /// voice channel, if connected.
    ///
    /// The handler is then dropped, removing settings for the target.
    ///
    /// [`Handler`]: struct.Handler.html
    pub fn remove<T: Into<Target>>(&mut self, target_id: T) {
        let target = target_id.into();

        self.leave(target);

        self.handlers.remove(&target);
    }
}
