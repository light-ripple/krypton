/*
 * I wanted this rather to be categorized than sequentially.
 * Maybe to find stuff faster, 90% preference.
*/
#[derive(Debug)]
#[repr(u16)]
pub enum PacketType {
    /// META
    StatusUpdate,
    Logout = 2,
    Ping = 4,
    RequestStatusUpdate = 3,
    #[deprecated(note = "This Request is now being done via web handlers")]
    RequestBeatmapInfo = 68,

    FriendAdd = 73,
    FriendRemove = 74,
    // This will be mostly unnecessary since we're autopushing updates.
    ReceiveUpdates = 79,
    RequestUserStats = 85,

    /// Spectator stuff
    StartSpectate = 16,
    StopSpectate = 17,
    SpectateFrames = 18,

    /// All and everything multiplayer
    LeaveLobby = 29,
    JoinLobby = 30,
    CreateMatch = 31,
    JoinMatch = 32,
    LeaveMatch = 33,
    MatchChangeSlot = 38,
    MatchReady = 39,
    MatchLockSlot = 40,
    MatchChangeSettings = 41,
    MatchStart = 44,
    MatchScoreFrame = 47,
    MatchFinished = 49,
    MatchChangeMods = 51,
    MatchLoadComplete = 52,
    MatchNoBeatmap = 54,
    MatchNotReady = 55,
    MatchFailed = 56,
    MatchGotBeatmap = 59,
    MatchSkipIntro = 60,
    MatchTransferHost = 70,
    MatchChangeTeam = 77,
    // This is just a simple IrcMessage object (why)
    MatchInvite = 87,
    MatchChangePassword = 90,
    MatchAbort = 106,

    /// Irc Stuff
    IrcMessage = 1,
    PrivateMessage = 25,
    ChannelJoin = 63,
    ChannelLeave = 78,
}

impl From<u16> for PacketType {
    fn from(v: u16) -> Self {
        unsafe { std::mem::transmute(v) }
    }
}