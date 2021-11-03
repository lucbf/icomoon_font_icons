#![doc(html_logo_url = "https://github.com/lucbf/icomoon_font_icons/raw/master/IcoMoon.svg", html_favicon_url = "https://github.com/lucbf/icomoon_font_icons/raw/master/IcoMoon.svg")]
//! **[The IcoMoon Font Icons]**.
//! 
//! The icons are [redistributed] by [Kordamp] with the [Apache 2.0]
//! license. Please, consider supporting its original creators by purchasing it at
//! [icomoon.io].
//! 
//! This crate contains the [Kordamp] version of IcoMoon, but is under the MIT license,
//! therefore the [Apache 2.0] license only applies to its redistributed font files.
//! 
//! [The IcoMoon Font Icons]: https://kordamp.org/ikonli/cheat-sheet-icomoon.html
//! [icomoon.io]: https://icomoon.io/#icons-icomoon
//! [redistributed]: https://github.com/kordamp/ikonli
//! [Kordamp]: https://kordamp.org/
//! [GPL]: https://www.gnu.org/licenses/gpl-3.0.html
//! [CC-4.0]: https://creativecommons.org/licenses/by/4.0/legalcode
//! [Apache 2.0]: https://www.apache.org/licenses/LICENSE-2.0

#![no_std]

///
/// An appropriate font file format is meant to be used together with [IcoMoon].
/// The correct format naturally depends on the program being built or the target os
pub mod font_file {
	use core::include_bytes;

/// .ttf font
	pub static TTF: &[u8] = include_bytes!("icomoon.ttf");

/// .eot font 
	pub static EOT: &[u8] = include_bytes!("icomoon.eot");

/// .woff font
	pub static WOFF: &[u8] = include_bytes!("icomoon.woff");
}

/// Represents a char, but is meant to be used with `char::from()`.
/// 
/// Example:
/// ```
/// use icomoon_font_icons::IcoMoon;
/// 
/// let i = char::from(IcoMoon::Home);
/// ```
/// 
/// <style type="text/css">
/// @font-face {
/// font-family: 'icomoon';
/// src: url('https://kordamp.org/ikonli/fonts/icomoon.eot?hy0xsg');
/// src: url('https://kordamp.org/ikonli/fonts/icomoon.eot?hy0xsg#iefix') format('embedded-opentype'),
/// url('https://kordamp.org/ikonli/fonts/icomoon.ttf?hy0xsg') format('truetype'),
/// url('https://kordamp.org/ikonli/fonts/icomoon.woff?hy0xsg') format('woff'),
/// url('https://kordamp.org/ikonli/fonts/icomoon.svg?hy0xsg#icomoon') format('svg');
/// font-weight: normal;
/// font-style: normal;
/// }
///
/// .icomoon {
/// font-family: 'icomoon' !important;
/// font-size: 28px
/// }
/// </style>
/// 
/// You can also click in a button below to copy the character to the clipboard.
#[repr(C)]
pub enum IcoMoon{
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59826)'></button>
	Accessibility = 59826,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59716)'></button>
	AddressBook = 59716,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59800)'></button>
	AidKit = 59800,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59823)'></button>
	Airplane = 59823,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59728)'></button>
	Alarm = 59728,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60039)'></button>
	Amazon = 60039,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60096)'></button>
	Android = 60096,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59885)'></button>
	Angry = 59885,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59886)'></button>
	AngryR = 59886,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60094)'></button>
	Appleinc = 60094,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59958)'></button>
	ArrowDown = 59958,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59966)'></button>
	ArrowDownR = 59966,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59959)'></button>
	ArrowDownLeft = 59959,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59967)'></button>
	ArrowDownLeftR = 59967,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59957)'></button>
	ArrowDownRight = 59957,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59965)'></button>
	ArrowDownRightR = 59965,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59960)'></button>
	ArrowLeft = 59960,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59968)'></button>
	ArrowLeftR = 59968,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59956)'></button>
	ArrowRight = 59956,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59964)'></button>
	ArrowRightR = 59964,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59954)'></button>
	ArrowUp = 59954,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59962)'></button>
	ArrowUpR = 59962,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59953)'></button>
	ArrowUpLeft = 59953,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59961)'></button>
	ArrowUpLeftR = 59961,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59955)'></button>
	ArrowUpRight = 59955,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59963)'></button>
	ArrowUpRightR = 59963,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59853)'></button>
	Attachment = 59853,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59930)'></button>
	Backward = 59930,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59935)'></button>
	BackwardR = 59935,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59891)'></button>
	Baffled = 59891,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59892)'></button>
	BaffledR = 59892,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59703)'></button>
	Barcode = 59703,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60082)'></button>
	Basecamp = 60082,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60072)'></button>
	Behance = 60072,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60073)'></button>
	BehanceR = 60073,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59729)'></button>
	Bell = 59729,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59820)'></button>
	Bin = 59820,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59821)'></button>
	BinR = 59821,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59781)'></button>
	Binoculars = 59781,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59918)'></button>
	Blocked = 59918,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59657)'></button>
	Blog = 59657,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60087)'></button>
	Blogger = 60087,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60088)'></button>
	BloggerR = 60088,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60002)'></button>
	Bold = 60002,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59679)'></button>
	Book = 59679,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59858)'></button>
	Bookmark = 59858,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59859)'></button>
	Bookmarks = 59859,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59680)'></button>
	Books = 59680,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59742)'></button>
	BoxAdd = 59742,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59743)'></button>
	BoxRemove = 59743,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59822)'></button>
	Briefcase = 59822,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59862)'></button>
	BrightnessContrast = 59862,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59755)'></button>
	Bubble = 59755,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59758)'></button>
	BubbleR = 59758,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59756)'></button>
	Bubbles = 59756,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59757)'></button>
	BubblesR = 59757,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59759)'></button>
	BubblesS = 59759,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59760)'></button>
	BubblesT = 59760,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59801)'></button>
	Bug = 59801,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59674)'></button>
	Bullhorn = 59674,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59712)'></button>
	Calculator = 59712,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59731)'></button>
	Calendar = 59731,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59663)'></button>
	Camera = 59663,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59917)'></button>
	CancelCircle = 59917,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59706)'></button>
	Cart = 59706,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59986)'></button>
	CheckboxChecked = 59986,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59987)'></button>
	CheckboxUnchecked = 59987,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59920)'></button>
	Checkmark = 59920,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59921)'></button>
	CheckmarkR = 59921,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60121)'></button>
	Chrome = 60121,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59971)'></button>
	CircleDown = 59971,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59972)'></button>
	CircleLeft = 59972,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59970)'></button>
	CircleRight = 59970,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59969)'></button>
	CircleUp = 59969,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60015)'></button>
	ClearFormatting = 60015,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59832)'></button>
	Clipboard = 59832,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59726)'></button>
	Clock = 59726,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59727)'></button>
	ClockR = 59727,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59841)'></button>
	Cloud = 59841,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59844)'></button>
	CloudCheck = 59844,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59842)'></button>
	CloudDownload = 59842,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59843)'></button>
	CloudUpload = 59843,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59672)'></button>
	Clubs = 59672,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60136)'></button>
	Codepen = 60136,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59796)'></button>
	Cog = 59796,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59797)'></button>
	Cogs = 59797,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59707)'></button>
	CoinDollar = 59707,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59708)'></button>
	CoinEuro = 59708,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59709)'></button>
	CoinPound = 59709,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59710)'></button>
	CoinYen = 59710,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59982)'></button>
	Command = 59982,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59721)'></button>
	Compass = 59721,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59722)'></button>
	CompassR = 59722,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59893)'></button>
	Confused = 59893,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59894)'></button>
	ConfusedR = 59894,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59675)'></button>
	Connection = 59675,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59861)'></button>
	Contrast = 59861,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59883)'></button>
	Cool = 59883,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59884)'></button>
	CoolR = 59884,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59692)'></button>
	Copy = 59692,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59711)'></button>
	CreditCard = 59711,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59991)'></button>
	Crop = 59991,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59919)'></button>
	Cross = 59919,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59905)'></button>
	Crying = 59905,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59906)'></button>
	CryingR = 59906,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60134)'></button>
	CssS = 60134,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59984)'></button>
	Ctrl = 59984,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59748)'></button>
	Database = 59748,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60109)'></button>
	Delicious = 60109,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60074)'></button>
	Deviantart = 60074,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59673)'></button>
	Diamonds = 59673,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59669)'></button>
	Dice = 59669,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59734)'></button>
	Display = 59734,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59744)'></button>
	Download = 59744,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59845)'></button>
	DownloadR = 59845,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59847)'></button>
	DownloadS = 59847,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59740)'></button>
	Drawer = 59740,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59741)'></button>
	DrawerR = 59741,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60071)'></button>
	Dribbble = 60071,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59747)'></button>
	Drive = 59747,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60078)'></button>
	Dropbox = 60078,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59659)'></button>
	Droplet = 59659,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59850)'></button>
	Earth = 59850,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60124)'></button>
	Edge = 60124,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59941)'></button>
	Eject = 59941,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60086)'></button>
	Ello = 60086,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60031)'></button>
	Embed = 60031,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60032)'></button>
	EmbedR = 60032,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59785)'></button>
	Enlarge = 59785,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59787)'></button>
	EnlargeR = 59787,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59923)'></button>
	Enter = 59923,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59717)'></button>
	Envelop = 59717,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59794)'></button>
	Equalizer = 59794,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59795)'></button>
	EqualizerR = 59795,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59887)'></button>
	Evil = 59887,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59888)'></button>
	EvilR = 59888,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59924)'></button>
	Exit = 59924,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59854)'></button>
	Eye = 59854,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59658)'></button>
	Eyedropper = 59658,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59857)'></button>
	EyeBlocked = 59857,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59856)'></button>
	EyeMinus = 59856,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59855)'></button>
	EyePlus = 59855,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60048)'></button>
	Facebook = 60048,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60049)'></button>
	FacebookR = 60049,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59677)'></button>
	Feed = 59677,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59685)'></button>
	FilesEmpty = 59685,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59684)'></button>
	FileEmpty = 59684,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60130)'></button>
	FileExcel = 60130,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59688)'></button>
	FileMusic = 59688,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60128)'></button>
	FileOpenoffice = 60128,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60127)'></button>
	FilePdf = 60127,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59687)'></button>
	FilePicture = 59687,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59689)'></button>
	FilePlay = 59689,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59682)'></button>
	FileText = 59682,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59686)'></button>
	FileTextR = 59686,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59690)'></button>
	FileVideo = 59690,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60129)'></button>
	FileWord = 60129,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59691)'></button>
	FileZip = 59691,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59667)'></button>
	Film = 59667,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59995)'></button>
	Filter = 59995,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60095)'></button>
	Finder = 60095,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59817)'></button>
	Fire = 59817,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60122)'></button>
	Firefox = 60122,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59937)'></button>
	First = 59937,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59852)'></button>
	Flag = 59852,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60117)'></button>
	Flattr = 60117,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60067)'></button>
	Flickr = 60067,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60068)'></button>
	FlickrR = 60068,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60069)'></button>
	FlickrS = 60069,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60070)'></button>
	FlickrT = 60070,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59746)'></button>
	FloppyDisk = 59746,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59695)'></button>
	Folder = 59695,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59699)'></button>
	FolderDownload = 59699,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59698)'></button>
	FolderMinus = 59698,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59696)'></button>
	FolderOpen = 59696,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59697)'></button>
	FolderPlus = 59697,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59700)'></button>
	FolderUpload = 59700,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59996)'></button>
	Font = 59996,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60001)'></button>
	FontSize = 60001,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59753)'></button>
	Forward = 59753,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59931)'></button>
	ForwardR = 59931,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59936)'></button>
	ForwardS = 59936,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60118)'></button>
	Foursquare = 60118,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59903)'></button>
	Frustrated = 59903,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59904)'></button>
	FrustratedR = 59904,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59807)'></button>
	Gift = 59807,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60135)'></button>
	Git = 60135,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60080)'></button>
	Github = 60080,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59808)'></button>
	Glass = 59808,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59809)'></button>
	GlassR = 59809,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60040)'></button>
	Google = 60040,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60041)'></button>
	GoogleR = 60041,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60042)'></button>
	GoogleS = 60042,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60047)'></button>
	GoogleDrive = 60047,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60043)'></button>
	GooglePlus = 60043,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60044)'></button>
	GooglePlusR = 60044,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60045)'></button>
	GooglePlusS = 60045,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59881)'></button>
	Grin = 59881,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59882)'></button>
	GrinR = 59882,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60103)'></button>
	Hackernews = 60103,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59798)'></button>
	Hammer = 59798,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59816)'></button>
	HammerR = 59816,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60046)'></button>
	Hangouts = 60046,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59871)'></button>
	Happy = 59871,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59872)'></button>
	HappyR = 59872,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59664)'></button>
	Headphones = 59664,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59866)'></button>
	Heart = 59866,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59867)'></button>
	HeartBroken = 59867,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59897)'></button>
	Hipster = 59897,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59898)'></button>
	HipsterR = 59898,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59725)'></button>
	History = 59725,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59648)'></button>
	Home = 59648,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59649)'></button>
	HomeR = 59649,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59650)'></button>
	HomeS = 59650,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59769)'></button>
	HourGlass = 59769,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60132)'></button>
	HtmlFive = 60132,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60133)'></button>
	HtmlFiveR = 60133,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60138)'></button>
	Icomoon = 60138,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60123)'></button>
	Ie = 60123,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59661)'></button>
	Image = 59661,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59662)'></button>
	Images = 59662,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60028)'></button>
	IndentDecrease = 60028,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60027)'></button>
	IndentIncrease = 60027,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59951)'></button>
	Infinite = 59951,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59916)'></button>
	Info = 59916,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60018)'></button>
	InsertTemplate = 60018,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60050)'></button>
	Instagram = 60050,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60004)'></button>
	Italic = 60004,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60085)'></button>
	Joomla = 60085,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59789)'></button>
	Key = 59789,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59790)'></button>
	KeyR = 59790,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59733)'></button>
	Keyboard = 59733,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59818)'></button>
	Lab = 59818,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60066)'></button>
	Lanyrd = 60066,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59735)'></button>
	Laptop = 59735,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59938)'></button>
	Last = 59938,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60107)'></button>
	Lastfm = 60107,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60108)'></button>
	LastfmR = 60108,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59812)'></button>
	Leaf = 59812,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59681)'></button>
	Library = 59681,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60131)'></button>
	Libreoffice = 60131,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59713)'></button>
	Lifebuoy = 59713,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59997)'></button>
	Ligature = 59997,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59998)'></button>
	LigatureR = 59998,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59851)'></button>
	Link = 59851,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60105)'></button>
	Linkedin = 60105,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60106)'></button>
	LinkedinR = 60106,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59834)'></button>
	List = 59834,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59835)'></button>
	ListR = 59835,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59833)'></button>
	ListNumbered = 59833,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59719)'></button>
	Location = 59719,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59720)'></button>
	LocationR = 59720,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59791)'></button>
	Lock = 59791,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59949)'></button>
	Loop = 59949,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59950)'></button>
	LoopR = 59950,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60020)'></button>
	Ltr = 60020,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59799)'></button>
	MagicWand = 59799,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59819)'></button>
	Magnet = 59819,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60035)'></button>
	Mail = 60035,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60036)'></button>
	MailR = 60036,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60037)'></button>
	MailS = 60037,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60038)'></button>
	MailT = 60038,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59992)'></button>
	MakeGroup = 59992,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59868)'></button>
	Man = 59868,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59870)'></button>
	ManWoman = 59870,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59723)'></button>
	Map = 59723,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59724)'></button>
	MapR = 59724,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59837)'></button>
	Menu = 59837,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59838)'></button>
	MenuR = 59838,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59839)'></button>
	MenuS = 59839,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59840)'></button>
	MenuT = 59840,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59814)'></button>
	Meter = 59814,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59815)'></button>
	MeterR = 59815,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59678)'></button>
	Mic = 59678,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59915)'></button>
	Minus = 59915,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59736)'></button>
	Mobile = 59736,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59737)'></button>
	MobileR = 59737,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59975)'></button>
	MoveDown = 59975,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59974)'></button>
	MoveUp = 59974,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59810)'></button>
	Mug = 59810,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59665)'></button>
	Music = 59665,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59895)'></button>
	Neutral = 59895,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59896)'></button>
	NeutralR = 59896,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59652)'></button>
	Newspaper = 59652,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60030)'></button>
	NewTab = 60030,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59929)'></button>
	Next = 59929,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59940)'></button>
	NextR = 59940,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59912)'></button>
	Notification = 59912,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60081)'></button>
	Npm = 60081,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59651)'></button>
	Office = 59651,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60006)'></button>
	Omega = 60006,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60079)'></button>
	Onedrive = 60079,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60126)'></button>
	Opera = 60126,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59985)'></button>
	Opt = 59985,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59670)'></button>
	Pacman = 59670,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60014)'></button>
	Pagebreak = 60014,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60008)'></button>
	PageBreak = 60008,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59660)'></button>
	PaintFormat = 59660,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60024)'></button>
	ParagraphCenter = 60024,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60026)'></button>
	ParagraphJustify = 60026,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60023)'></button>
	ParagraphLeft = 60023,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60025)'></button>
	ParagraphRight = 60025,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59693)'></button>
	Paste = 59693,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59926)'></button>
	Pause = 59926,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59933)'></button>
	PauseR = 59933,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60120)'></button>
	Paypal = 60120,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59656)'></button>
	Pen = 59656,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59653)'></button>
	Pencil = 59653,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59654)'></button>
	PencilR = 59654,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59714)'></button>
	Phone = 59714,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59715)'></button>
	PhoneHangUp = 59715,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59802)'></button>
	PieChart = 59802,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60019)'></button>
	Pilcrow = 60019,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60113)'></button>
	Pinterest = 60113,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60114)'></button>
	PinterestR = 60114,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59666)'></button>
	Play = 59666,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59925)'></button>
	PlayR = 59925,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59932)'></button>
	PlayS = 59932,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59914)'></button>
	Plus = 59914,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59676)'></button>
	Podcast = 59676,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59909)'></button>
	PointDown = 59909,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59910)'></button>
	PointLeft = 59910,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59908)'></button>
	PointRight = 59908,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59907)'></button>
	PointUp = 59907,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59829)'></button>
	Power = 59829,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59831)'></button>
	PowerCord = 59831,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59928)'></button>
	Previous = 59928,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59939)'></button>
	PreviousR = 59939,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59701)'></button>
	PriceTag = 59701,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59702)'></button>
	PriceTags = 59702,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59732)'></button>
	Printer = 59732,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59683)'></button>
	Profile = 59683,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59718)'></button>
	Pushpin = 59718,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59704)'></button>
	Qrcode = 59704,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59913)'></button>
	Question = 59913,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59655)'></button>
	Quill = 59655,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59767)'></button>
	QuotesLeft = 59767,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59768)'></button>
	QuotesRight = 59768,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59988)'></button>
	RadioChecked = 59988,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59989)'></button>
	RadioCheckedR = 59989,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59990)'></button>
	RadioUnchecked = 59990,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60102)'></button>
	Reddit = 60102,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59750)'></button>
	Redo = 59750,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59752)'></button>
	RedoR = 59752,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60057)'></button>
	Renren = 60057,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59754)'></button>
	Reply = 59754,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59825)'></button>
	Road = 59825,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59813)'></button>
	Rocket = 59813,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60059)'></button>
	Rss = 60059,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60060)'></button>
	RssR = 60060,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60021)'></button>
	Rtl = 60021,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60075)'></button>
	SUPPpx = 60075,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59877)'></button>
	Sad = 59877,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59878)'></button>
	SadR = 59878,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60125)'></button>
	Safari = 60125,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59994)'></button>
	Scissors = 59994,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59782)'></button>
	Search = 59782,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60022)'></button>
	Section = 60022,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60029)'></button>
	Share = 60029,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60034)'></button>
	ShareR = 60034,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59828)'></button>
	Shield = 59828,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59983)'></button>
	Shift = 59983,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59889)'></button>
	Shocked = 59889,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59890)'></button>
	ShockedR = 59890,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59786)'></button>
	Shrink = 59786,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59788)'></button>
	ShrinkR = 59788,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59952)'></button>
	Shuffle = 59952,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60007)'></button>
	Sigma = 60007,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60058)'></button>
	SinaWeibo = 60058,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60101)'></button>
	Skype = 60101,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59901)'></button>
	Sleepy = 59901,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59902)'></button>
	SleepyR = 59902,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59873)'></button>
	Smile = 59873,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59874)'></button>
	SmileR = 59874,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59976)'></button>
	SortAlphaAsc = 59976,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59977)'></button>
	SortAlphaDesc = 59977,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59980)'></button>
	SortAmountAsc = 59980,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59981)'></button>
	SortAmountDesc = 59981,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59979)'></button>
	SortNumbericDesc = 59979,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59978)'></button>
	SortNumericAsc = 59978,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60099)'></button>
	Soundcloud = 60099,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60100)'></button>
	SoundcloudR = 60100,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59671)'></button>
	Spades = 59671,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59922)'></button>
	SpellCheck = 59922,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59849)'></button>
	Sphere = 59849,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59770)'></button>
	Spinner = 59770,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59779)'></button>
	SpinnerQP = 59779,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59780)'></button>
	SpinnerQQ = 59780,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59771)'></button>
	SpinnerR = 59771,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59772)'></button>
	SpinnerS = 59772,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59773)'></button>
	SpinnerT = 59773,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59774)'></button>
	SpinnerU = 59774,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59775)'></button>
	SpinnerV = 59775,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59776)'></button>
	SpinnerW = 59776,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59777)'></button>
	SpinnerX = 59777,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59778)'></button>
	SpinnerY = 59778,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59811)'></button>
	SpoonKnife = 59811,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60052)'></button>
	Spotify = 60052,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59694)'></button>
	Stack = 59694,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60112)'></button>
	Stackoverflow = 60112,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59863)'></button>
	StarEmpty = 59863,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59865)'></button>
	StarFull = 59865,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59864)'></button>
	StarHalf = 59864,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59804)'></button>
	StatsBars = 59804,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59805)'></button>
	StatsBarsR = 59805,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59803)'></button>
	StatsDots = 59803,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60076)'></button>
	Steam = 60076,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60077)'></button>
	SteamR = 60077,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59927)'></button>
	Stop = 59927,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59934)'></button>
	StopR = 59934,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59730)'></button>
	Stopwatch = 59730,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60005)'></button>
	Strikethrough = 60005,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60110)'></button>
	Stumbleupon = 60110,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60111)'></button>
	StumbleuponR = 60111,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60010)'></button>
	Subscript = 60010,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60012)'></button>
	SubscriptR = 60012,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59860)'></button>
	Sun = 59860,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60009)'></button>
	Superscript = 60009,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60011)'></button>
	SuperscriptR = 60011,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60137)'></button>
	Svg = 60137,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59830)'></button>
	Switch = 59830,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59973)'></button>
	Tab = 59973,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60016)'></button>
	Table = 60016,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60017)'></button>
	TableR = 60017,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59738)'></button>
	Tablet = 59738,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59827)'></button>
	Target = 59827,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60053)'></button>
	Telegram = 60053,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60033)'></button>
	Terminal = 60033,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60013)'></button>
	TextColor = 60013,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59999)'></button>
	TextHeight = 59999,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60000)'></button>
	TextWidth = 60000,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59705)'></button>
	Ticket = 59705,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59875)'></button>
	Tongue = 59875,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59876)'></button>
	TongueR = 59876,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59836)'></button>
	Tree = 59836,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60083)'></button>
	Trello = 60083,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59806)'></button>
	Trophy = 59806,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59824)'></button>
	Truck = 59824,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60089)'></button>
	Tumblr = 60089,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60090)'></button>
	TumblrR = 60090,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60093)'></button>
	Tux = 60093,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59739)'></button>
	Tv = 59739,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60063)'></button>
	Twitch = 60063,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60054)'></button>
	Twitter = 60054,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60003)'></button>
	Underline = 60003,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59749)'></button>
	Undo = 59749,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59751)'></button>
	UndoR = 59751,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59993)'></button>
	Ungroup = 59993,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59792)'></button>
	Unlocked = 59792,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59745)'></button>
	Upload = 59745,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59846)'></button>
	UploadR = 59846,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59848)'></button>
	UploadS = 59848,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59761)'></button>
	User = 59761,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59762)'></button>
	Users = 59762,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59765)'></button>
	UserCheck = 59765,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59764)'></button>
	UserMinus = 59764,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59763)'></button>
	UserPlus = 59763,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59766)'></button>
	UserTie = 59766,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59668)'></button>
	VideoCamera = 59668,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60064)'></button>
	Vimeo = 60064,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60065)'></button>
	VimeoR = 60065,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60055)'></button>
	Vine = 60055,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60056)'></button>
	Vk = 60056,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59948)'></button>
	VolumeDecrease = 59948,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59942)'></button>
	VolumeHigh = 59942,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59947)'></button>
	VolumeIncrease = 59947,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59944)'></button>
	VolumeLow = 59944,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59943)'></button>
	VolumeMedium = 59943,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59945)'></button>
	VolumeMute = 59945,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59946)'></button>
	VolumeMuteR = 59946,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59911)'></button>
	Warning = 59911,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60051)'></button>
	Whatsapp = 60051,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60104)'></button>
	Wikipedia = 60104,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60097)'></button>
	Windows = 60097,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60098)'></button>
	WindowsX = 60098,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59879)'></button>
	Wink = 59879,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59880)'></button>
	WinkR = 59880,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59869)'></button>
	Woman = 59869,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59899)'></button>
	Wondering = 59899,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59900)'></button>
	WonderingR = 59900,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60084)'></button>
	Wordpress = 60084,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59793)'></button>
	Wrench = 59793,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60115)'></button>
	Xing = 60115,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60116)'></button>
	XingR = 60116,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60091)'></button>
	Yahoo = 60091,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60092)'></button>
	YahooR = 60092,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60119)'></button>
	Yelp = 60119,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60061)'></button>
	Youtube = 60061,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(60062)'></button>
	YoutubeR = 60062,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59783)'></button>
	ZoomIn = 59783,
/// <button class='icomoon' onclick='navigator.clipboard.writeText(59784)'></button>
	ZoomOut = 59784,
}

impl From<IcoMoon> for char{
    fn from(icon: IcoMoon) -> char {
        unsafe{char::from_u32_unchecked(icon as u32)}
    }
}
