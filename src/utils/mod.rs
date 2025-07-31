type AudioPath = &'static str;
type AudioCaption = &'static str;
type AudioClip = (AudioPath, AudioCaption);
pub type AudioClips<const N: usize> = [AudioClip; N];
