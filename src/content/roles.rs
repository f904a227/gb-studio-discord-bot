pub(crate) trait RoleDescribe {
    const NAME: &'static str;
    const EMOJI: char;
    const DESCRIPTION: &'static str;
}

pub(crate) struct ArtistRole;
impl RoleDescribe for ArtistRole {
    const NAME: &'static str = "Artist";
    const EMOJI: char = '🎨';
    const DESCRIPTION: &'static str = "2-bit Pixel Painter";
}

pub(crate) struct BetaTesterRole;
impl RoleDescribe for BetaTesterRole {
    const NAME: &'static str = "Beta tester";
    const EMOJI: char = '🧪';
    const DESCRIPTION: &'static str = "Adventurer looking for bugs";
}

pub(crate) struct DesignerRole;
impl RoleDescribe for DesignerRole {
    const NAME: &'static str = "Designer";
    const EMOJI: char = '📓';
    const DESCRIPTION: &'static str = "Makes the magic that is the \"fun\"";
}

pub(crate) struct HardwareEnthusiastRole;
impl RoleDescribe for HardwareEnthusiastRole {
    const NAME: &'static str = "Hardware enthusiast";
    const EMOJI: char = '🔨';
    const DESCRIPTION: &'static str = "One of the four legendary enthusiasts, in their spare time they create all kinds of wicked hardware";
}

pub(crate) struct MusicianRole;
impl RoleDescribe for MusicianRole {
    const NAME: &'static str = "Musician";
    const EMOJI: char = '🎵';
    const DESCRIPTION: &'static str = "Eats Chiptunes for breakfast";
}

pub(crate) struct ScripterRole;
impl RoleDescribe for ScripterRole {
    const NAME: &'static str = "Scripter";
    const EMOJI: char = '💻';
    const DESCRIPTION: &'static str = "`Hello World`";
}
