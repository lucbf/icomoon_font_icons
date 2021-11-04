#![doc(html_logo_url = "https://github.com/lucbf/icomoon_font_icons/raw/master/IcoMoon.svg", html_favicon_url = "https://github.com/lucbf/icomoon_font_icons/raw/master/IcoMoon.svg")]

//! The Icomoon Font Icons are [redistributed] by [Kordamp] with the [Apache 2.0]
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
/// **[The icons]**. An appropriate font file format is meant to be used together with [IcoMoon].
/// The correct format naturally depends on the program being built or the target os.
/// 
/// [The icons]: https://kordamp.org/ikonli/cheat-sheet-icomoon.html
pub mod font_file {
	use core::include_bytes;

/// .ttf font
	pub static TTF: &[u8] = include_bytes!("icomoon.ttf");

/// .eot font 
	pub static EOT: &[u8] = include_bytes!("icomoon.eot");

/// .woff font
	pub static WOFF: &[u8] = include_bytes!("icomoon.woff");
}

/// Represents a char. The conversion is done using `char::from()`.
/// 
/// Examples:
/// ```
/// use icomoon_font_icons::IcoMoon;
/// 
/// let i: char = char::from(IcoMoon::Home);
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
/// font-size: 28px;
/// color: #212121;
/// }
/// </style>
/// 
/// You can also click in a button below to copy the character to the clipboard,
/// but this is not recommended because your code becomes less transparent to the
/// reader.
/// ```
/// let i: char = '\u{e900}';//IcoMoon::Home
/// ```
#[repr(C)]
pub enum IcoMoon{
/// <button class='icomoon' id='\u{e9b2}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Accessibility = 59826,
/// <button class='icomoon' id='\u{e944}' onclick='navigator.clipboard.writeText(this.id)'></button>
	AddressBook = 59716,
/// <button class='icomoon' id='\u{e998}' onclick='navigator.clipboard.writeText(this.id)'></button>
	AidKit = 59800,
/// <button class='icomoon' id='\u{e9af}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Airplane = 59823,
/// <button class='icomoon' id='\u{e950}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Alarm = 59728,
/// <button class='icomoon' id='\u{ea87}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Amazon = 60039,
/// <button class='icomoon' id='\u{eac0}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Android = 60096,
/// <button class='icomoon' id='\u{e9ed}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Angry = 59885,
/// <button class='icomoon' id='\u{e9ee}' onclick='navigator.clipboard.writeText(this.id)'></button>
	AngryR = 59886,
/// <button class='icomoon' id='\u{eabe}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Appleinc = 60094,
/// <button class='icomoon' id='\u{ea36}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ArrowDown = 59958,
/// <button class='icomoon' id='\u{ea3e}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ArrowDownR = 59966,
/// <button class='icomoon' id='\u{ea37}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ArrowDownLeft = 59959,
/// <button class='icomoon' id='\u{ea3f}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ArrowDownLeftR = 59967,
/// <button class='icomoon' id='\u{ea35}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ArrowDownRight = 59957,
/// <button class='icomoon' id='\u{ea3d}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ArrowDownRightR = 59965,
/// <button class='icomoon' id='\u{ea38}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ArrowLeft = 59960,
/// <button class='icomoon' id='\u{ea40}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ArrowLeftR = 59968,
/// <button class='icomoon' id='\u{ea34}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ArrowRight = 59956,
/// <button class='icomoon' id='\u{ea3c}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ArrowRightR = 59964,
/// <button class='icomoon' id='\u{ea32}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ArrowUp = 59954,
/// <button class='icomoon' id='\u{ea3a}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ArrowUpR = 59962,
/// <button class='icomoon' id='\u{ea31}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ArrowUpLeft = 59953,
/// <button class='icomoon' id='\u{ea39}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ArrowUpLeftR = 59961,
/// <button class='icomoon' id='\u{ea33}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ArrowUpRight = 59955,
/// <button class='icomoon' id='\u{ea3b}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ArrowUpRightR = 59963,
/// <button class='icomoon' id='\u{e9cd}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Attachment = 59853,
/// <button class='icomoon' id='\u{ea1a}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Backward = 59930,
/// <button class='icomoon' id='\u{ea1f}' onclick='navigator.clipboard.writeText(this.id)'></button>
	BackwardR = 59935,
/// <button class='icomoon' id='\u{e9f3}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Baffled = 59891,
/// <button class='icomoon' id='\u{e9f4}' onclick='navigator.clipboard.writeText(this.id)'></button>
	BaffledR = 59892,
/// <button class='icomoon' id='\u{e937}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Barcode = 59703,
/// <button class='icomoon' id='\u{eab2}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Basecamp = 60082,
/// <button class='icomoon' id='\u{eaa8}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Behance = 60072,
/// <button class='icomoon' id='\u{eaa9}' onclick='navigator.clipboard.writeText(this.id)'></button>
	BehanceR = 60073,
/// <button class='icomoon' id='\u{e951}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Bell = 59729,
/// <button class='icomoon' id='\u{e9ac}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Bin = 59820,
/// <button class='icomoon' id='\u{e9ad}' onclick='navigator.clipboard.writeText(this.id)'></button>
	BinR = 59821,
/// <button class='icomoon' id='\u{e985}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Binoculars = 59781,
/// <button class='icomoon' id='\u{ea0e}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Blocked = 59918,
/// <button class='icomoon' id='\u{e909}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Blog = 59657,
/// <button class='icomoon' id='\u{eab7}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Blogger = 60087,
/// <button class='icomoon' id='\u{eab8}' onclick='navigator.clipboard.writeText(this.id)'></button>
	BloggerR = 60088,
/// <button class='icomoon' id='\u{ea62}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Bold = 60002,
/// <button class='icomoon' id='\u{e91f}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Book = 59679,
/// <button class='icomoon' id='\u{e9d2}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Bookmark = 59858,
/// <button class='icomoon' id='\u{e9d3}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Bookmarks = 59859,
/// <button class='icomoon' id='\u{e920}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Books = 59680,
/// <button class='icomoon' id='\u{e95e}' onclick='navigator.clipboard.writeText(this.id)'></button>
	BoxAdd = 59742,
/// <button class='icomoon' id='\u{e95f}' onclick='navigator.clipboard.writeText(this.id)'></button>
	BoxRemove = 59743,
/// <button class='icomoon' id='\u{e9ae}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Briefcase = 59822,
/// <button class='icomoon' id='\u{e9d6}' onclick='navigator.clipboard.writeText(this.id)'></button>
	BrightnessContrast = 59862,
/// <button class='icomoon' id='\u{e96b}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Bubble = 59755,
/// <button class='icomoon' id='\u{e96e}' onclick='navigator.clipboard.writeText(this.id)'></button>
	BubbleR = 59758,
/// <button class='icomoon' id='\u{e96c}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Bubbles = 59756,
/// <button class='icomoon' id='\u{e96d}' onclick='navigator.clipboard.writeText(this.id)'></button>
	BubblesR = 59757,
/// <button class='icomoon' id='\u{e96f}' onclick='navigator.clipboard.writeText(this.id)'></button>
	BubblesS = 59759,
/// <button class='icomoon' id='\u{e970}' onclick='navigator.clipboard.writeText(this.id)'></button>
	BubblesT = 59760,
/// <button class='icomoon' id='\u{e999}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Bug = 59801,
/// <button class='icomoon' id='\u{e91a}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Bullhorn = 59674,
/// <button class='icomoon' id='\u{e940}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Calculator = 59712,
/// <button class='icomoon' id='\u{e953}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Calendar = 59731,
/// <button class='icomoon' id='\u{e90f}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Camera = 59663,
/// <button class='icomoon' id='\u{ea0d}' onclick='navigator.clipboard.writeText(this.id)'></button>
	CancelCircle = 59917,
/// <button class='icomoon' id='\u{e93a}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Cart = 59706,
/// <button class='icomoon' id='\u{ea52}' onclick='navigator.clipboard.writeText(this.id)'></button>
	CheckboxChecked = 59986,
/// <button class='icomoon' id='\u{ea53}' onclick='navigator.clipboard.writeText(this.id)'></button>
	CheckboxUnchecked = 59987,
/// <button class='icomoon' id='\u{ea10}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Checkmark = 59920,
/// <button class='icomoon' id='\u{ea11}' onclick='navigator.clipboard.writeText(this.id)'></button>
	CheckmarkR = 59921,
/// <button class='icomoon' id='\u{ead9}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Chrome = 60121,
/// <button class='icomoon' id='\u{ea43}' onclick='navigator.clipboard.writeText(this.id)'></button>
	CircleDown = 59971,
/// <button class='icomoon' id='\u{ea44}' onclick='navigator.clipboard.writeText(this.id)'></button>
	CircleLeft = 59972,
/// <button class='icomoon' id='\u{ea42}' onclick='navigator.clipboard.writeText(this.id)'></button>
	CircleRight = 59970,
/// <button class='icomoon' id='\u{ea41}' onclick='navigator.clipboard.writeText(this.id)'></button>
	CircleUp = 59969,
/// <button class='icomoon' id='\u{ea6f}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ClearFormatting = 60015,
/// <button class='icomoon' id='\u{e9b8}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Clipboard = 59832,
/// <button class='icomoon' id='\u{e94e}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Clock = 59726,
/// <button class='icomoon' id='\u{e94f}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ClockR = 59727,
/// <button class='icomoon' id='\u{e9c1}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Cloud = 59841,
/// <button class='icomoon' id='\u{e9c4}' onclick='navigator.clipboard.writeText(this.id)'></button>
	CloudCheck = 59844,
/// <button class='icomoon' id='\u{e9c2}' onclick='navigator.clipboard.writeText(this.id)'></button>
	CloudDownload = 59842,
/// <button class='icomoon' id='\u{e9c3}' onclick='navigator.clipboard.writeText(this.id)'></button>
	CloudUpload = 59843,
/// <button class='icomoon' id='\u{e918}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Clubs = 59672,
/// <button class='icomoon' id='\u{eae8}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Codepen = 60136,
/// <button class='icomoon' id='\u{e994}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Cog = 59796,
/// <button class='icomoon' id='\u{e995}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Cogs = 59797,
/// <button class='icomoon' id='\u{e93b}' onclick='navigator.clipboard.writeText(this.id)'></button>
	CoinDollar = 59707,
/// <button class='icomoon' id='\u{e93c}' onclick='navigator.clipboard.writeText(this.id)'></button>
	CoinEuro = 59708,
/// <button class='icomoon' id='\u{e93d}' onclick='navigator.clipboard.writeText(this.id)'></button>
	CoinPound = 59709,
/// <button class='icomoon' id='\u{e93e}' onclick='navigator.clipboard.writeText(this.id)'></button>
	CoinYen = 59710,
/// <button class='icomoon' id='\u{ea4e}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Command = 59982,
/// <button class='icomoon' id='\u{e949}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Compass = 59721,
/// <button class='icomoon' id='\u{e94a}' onclick='navigator.clipboard.writeText(this.id)'></button>
	CompassR = 59722,
/// <button class='icomoon' id='\u{e9f5}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Confused = 59893,
/// <button class='icomoon' id='\u{e9f6}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ConfusedR = 59894,
/// <button class='icomoon' id='\u{e91b}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Connection = 59675,
/// <button class='icomoon' id='\u{e9d5}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Contrast = 59861,
/// <button class='icomoon' id='\u{e9eb}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Cool = 59883,
/// <button class='icomoon' id='\u{e9ec}' onclick='navigator.clipboard.writeText(this.id)'></button>
	CoolR = 59884,
/// <button class='icomoon' id='\u{e92c}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Copy = 59692,
/// <button class='icomoon' id='\u{e93f}' onclick='navigator.clipboard.writeText(this.id)'></button>
	CreditCard = 59711,
/// <button class='icomoon' id='\u{ea57}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Crop = 59991,
/// <button class='icomoon' id='\u{ea0f}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Cross = 59919,
/// <button class='icomoon' id='\u{ea01}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Crying = 59905,
/// <button class='icomoon' id='\u{ea02}' onclick='navigator.clipboard.writeText(this.id)'></button>
	CryingR = 59906,
/// <button class='icomoon' id='\u{eae6}' onclick='navigator.clipboard.writeText(this.id)'></button>
	CssS = 60134,
/// <button class='icomoon' id='\u{ea50}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Ctrl = 59984,
/// <button class='icomoon' id='\u{e964}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Database = 59748,
/// <button class='icomoon' id='\u{eacd}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Delicious = 60109,
/// <button class='icomoon' id='\u{eaaa}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Deviantart = 60074,
/// <button class='icomoon' id='\u{e919}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Diamonds = 59673,
/// <button class='icomoon' id='\u{e915}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Dice = 59669,
/// <button class='icomoon' id='\u{e956}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Display = 59734,
/// <button class='icomoon' id='\u{e960}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Download = 59744,
/// <button class='icomoon' id='\u{e9c5}' onclick='navigator.clipboard.writeText(this.id)'></button>
	DownloadR = 59845,
/// <button class='icomoon' id='\u{e9c7}' onclick='navigator.clipboard.writeText(this.id)'></button>
	DownloadS = 59847,
/// <button class='icomoon' id='\u{e95c}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Drawer = 59740,
/// <button class='icomoon' id='\u{e95d}' onclick='navigator.clipboard.writeText(this.id)'></button>
	DrawerR = 59741,
/// <button class='icomoon' id='\u{eaa7}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Dribbble = 60071,
/// <button class='icomoon' id='\u{e963}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Drive = 59747,
/// <button class='icomoon' id='\u{eaae}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Dropbox = 60078,
/// <button class='icomoon' id='\u{e90b}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Droplet = 59659,
/// <button class='icomoon' id='\u{e9ca}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Earth = 59850,
/// <button class='icomoon' id='\u{eadc}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Edge = 60124,
/// <button class='icomoon' id='\u{ea25}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Eject = 59941,
/// <button class='icomoon' id='\u{eab6}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Ello = 60086,
/// <button class='icomoon' id='\u{ea7f}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Embed = 60031,
/// <button class='icomoon' id='\u{ea80}' onclick='navigator.clipboard.writeText(this.id)'></button>
	EmbedR = 60032,
/// <button class='icomoon' id='\u{e989}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Enlarge = 59785,
/// <button class='icomoon' id='\u{e98b}' onclick='navigator.clipboard.writeText(this.id)'></button>
	EnlargeR = 59787,
/// <button class='icomoon' id='\u{ea13}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Enter = 59923,
/// <button class='icomoon' id='\u{e945}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Envelop = 59717,
/// <button class='icomoon' id='\u{e992}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Equalizer = 59794,
/// <button class='icomoon' id='\u{e993}' onclick='navigator.clipboard.writeText(this.id)'></button>
	EqualizerR = 59795,
/// <button class='icomoon' id='\u{e9ef}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Evil = 59887,
/// <button class='icomoon' id='\u{e9f0}' onclick='navigator.clipboard.writeText(this.id)'></button>
	EvilR = 59888,
/// <button class='icomoon' id='\u{ea14}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Exit = 59924,
/// <button class='icomoon' id='\u{e9ce}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Eye = 59854,
/// <button class='icomoon' id='\u{e90a}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Eyedropper = 59658,
/// <button class='icomoon' id='\u{e9d1}' onclick='navigator.clipboard.writeText(this.id)'></button>
	EyeBlocked = 59857,
/// <button class='icomoon' id='\u{e9d0}' onclick='navigator.clipboard.writeText(this.id)'></button>
	EyeMinus = 59856,
/// <button class='icomoon' id='\u{e9cf}' onclick='navigator.clipboard.writeText(this.id)'></button>
	EyePlus = 59855,
/// <button class='icomoon' id='\u{ea90}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Facebook = 60048,
/// <button class='icomoon' id='\u{ea91}' onclick='navigator.clipboard.writeText(this.id)'></button>
	FacebookR = 60049,
/// <button class='icomoon' id='\u{e91d}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Feed = 59677,
/// <button class='icomoon' id='\u{e925}' onclick='navigator.clipboard.writeText(this.id)'></button>
	FilesEmpty = 59685,
/// <button class='icomoon' id='\u{e924}' onclick='navigator.clipboard.writeText(this.id)'></button>
	FileEmpty = 59684,
/// <button class='icomoon' id='\u{eae2}' onclick='navigator.clipboard.writeText(this.id)'></button>
	FileExcel = 60130,
/// <button class='icomoon' id='\u{e928}' onclick='navigator.clipboard.writeText(this.id)'></button>
	FileMusic = 59688,
/// <button class='icomoon' id='\u{eae0}' onclick='navigator.clipboard.writeText(this.id)'></button>
	FileOpenoffice = 60128,
/// <button class='icomoon' id='\u{eadf}' onclick='navigator.clipboard.writeText(this.id)'></button>
	FilePdf = 60127,
/// <button class='icomoon' id='\u{e927}' onclick='navigator.clipboard.writeText(this.id)'></button>
	FilePicture = 59687,
/// <button class='icomoon' id='\u{e929}' onclick='navigator.clipboard.writeText(this.id)'></button>
	FilePlay = 59689,
/// <button class='icomoon' id='\u{e922}' onclick='navigator.clipboard.writeText(this.id)'></button>
	FileText = 59682,
/// <button class='icomoon' id='\u{e926}' onclick='navigator.clipboard.writeText(this.id)'></button>
	FileTextR = 59686,
/// <button class='icomoon' id='\u{e92a}' onclick='navigator.clipboard.writeText(this.id)'></button>
	FileVideo = 59690,
/// <button class='icomoon' id='\u{eae1}' onclick='navigator.clipboard.writeText(this.id)'></button>
	FileWord = 60129,
/// <button class='icomoon' id='\u{e92b}' onclick='navigator.clipboard.writeText(this.id)'></button>
	FileZip = 59691,
/// <button class='icomoon' id='\u{e913}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Film = 59667,
/// <button class='icomoon' id='\u{ea5b}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Filter = 59995,
/// <button class='icomoon' id='\u{eabf}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Finder = 60095,
/// <button class='icomoon' id='\u{e9a9}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Fire = 59817,
/// <button class='icomoon' id='\u{eada}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Firefox = 60122,
/// <button class='icomoon' id='\u{ea21}' onclick='navigator.clipboard.writeText(this.id)'></button>
	First = 59937,
/// <button class='icomoon' id='\u{e9cc}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Flag = 59852,
/// <button class='icomoon' id='\u{ead5}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Flattr = 60117,
/// <button class='icomoon' id='\u{eaa3}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Flickr = 60067,
/// <button class='icomoon' id='\u{eaa4}' onclick='navigator.clipboard.writeText(this.id)'></button>
	FlickrR = 60068,
/// <button class='icomoon' id='\u{eaa5}' onclick='navigator.clipboard.writeText(this.id)'></button>
	FlickrS = 60069,
/// <button class='icomoon' id='\u{eaa6}' onclick='navigator.clipboard.writeText(this.id)'></button>
	FlickrT = 60070,
/// <button class='icomoon' id='\u{e962}' onclick='navigator.clipboard.writeText(this.id)'></button>
	FloppyDisk = 59746,
/// <button class='icomoon' id='\u{e92f}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Folder = 59695,
/// <button class='icomoon' id='\u{e933}' onclick='navigator.clipboard.writeText(this.id)'></button>
	FolderDownload = 59699,
/// <button class='icomoon' id='\u{e932}' onclick='navigator.clipboard.writeText(this.id)'></button>
	FolderMinus = 59698,
/// <button class='icomoon' id='\u{e930}' onclick='navigator.clipboard.writeText(this.id)'></button>
	FolderOpen = 59696,
/// <button class='icomoon' id='\u{e931}' onclick='navigator.clipboard.writeText(this.id)'></button>
	FolderPlus = 59697,
/// <button class='icomoon' id='\u{e934}' onclick='navigator.clipboard.writeText(this.id)'></button>
	FolderUpload = 59700,
/// <button class='icomoon' id='\u{ea5c}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Font = 59996,
/// <button class='icomoon' id='\u{ea61}' onclick='navigator.clipboard.writeText(this.id)'></button>
	FontSize = 60001,
/// <button class='icomoon' id='\u{e969}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Forward = 59753,
/// <button class='icomoon' id='\u{ea1b}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ForwardR = 59931,
/// <button class='icomoon' id='\u{ea20}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ForwardS = 59936,
/// <button class='icomoon' id='\u{ead6}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Foursquare = 60118,
/// <button class='icomoon' id='\u{e9ff}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Frustrated = 59903,
/// <button class='icomoon' id='\u{ea00}' onclick='navigator.clipboard.writeText(this.id)'></button>
	FrustratedR = 59904,
/// <button class='icomoon' id='\u{e99f}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Gift = 59807,
/// <button class='icomoon' id='\u{eae7}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Git = 60135,
/// <button class='icomoon' id='\u{eab0}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Github = 60080,
/// <button class='icomoon' id='\u{e9a0}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Glass = 59808,
/// <button class='icomoon' id='\u{e9a1}' onclick='navigator.clipboard.writeText(this.id)'></button>
	GlassR = 59809,
/// <button class='icomoon' id='\u{ea88}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Google = 60040,
/// <button class='icomoon' id='\u{ea89}' onclick='navigator.clipboard.writeText(this.id)'></button>
	GoogleR = 60041,
/// <button class='icomoon' id='\u{ea8a}' onclick='navigator.clipboard.writeText(this.id)'></button>
	GoogleS = 60042,
/// <button class='icomoon' id='\u{ea8f}' onclick='navigator.clipboard.writeText(this.id)'></button>
	GoogleDrive = 60047,
/// <button class='icomoon' id='\u{ea8b}' onclick='navigator.clipboard.writeText(this.id)'></button>
	GooglePlus = 60043,
/// <button class='icomoon' id='\u{ea8c}' onclick='navigator.clipboard.writeText(this.id)'></button>
	GooglePlusR = 60044,
/// <button class='icomoon' id='\u{ea8d}' onclick='navigator.clipboard.writeText(this.id)'></button>
	GooglePlusS = 60045,
/// <button class='icomoon' id='\u{e9e9}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Grin = 59881,
/// <button class='icomoon' id='\u{e9ea}' onclick='navigator.clipboard.writeText(this.id)'></button>
	GrinR = 59882,
/// <button class='icomoon' id='\u{eac7}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Hackernews = 60103,
/// <button class='icomoon' id='\u{e996}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Hammer = 59798,
/// <button class='icomoon' id='\u{e9a8}' onclick='navigator.clipboard.writeText(this.id)'></button>
	HammerR = 59816,
/// <button class='icomoon' id='\u{ea8e}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Hangouts = 60046,
/// <button class='icomoon' id='\u{e9df}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Happy = 59871,
/// <button class='icomoon' id='\u{e9e0}' onclick='navigator.clipboard.writeText(this.id)'></button>
	HappyR = 59872,
/// <button class='icomoon' id='\u{e910}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Headphones = 59664,
/// <button class='icomoon' id='\u{e9da}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Heart = 59866,
/// <button class='icomoon' id='\u{e9db}' onclick='navigator.clipboard.writeText(this.id)'></button>
	HeartBroken = 59867,
/// <button class='icomoon' id='\u{e9f9}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Hipster = 59897,
/// <button class='icomoon' id='\u{e9fa}' onclick='navigator.clipboard.writeText(this.id)'></button>
	HipsterR = 59898,
/// <button class='icomoon' id='\u{e94d}' onclick='navigator.clipboard.writeText(this.id)'></button>
	History = 59725,
/// <button class='icomoon' id='\u{e900}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Home = 59648,
/// <button class='icomoon' id='\u{e901}' onclick='navigator.clipboard.writeText(this.id)'></button>
	HomeR = 59649,
/// <button class='icomoon' id='\u{e902}' onclick='navigator.clipboard.writeText(this.id)'></button>
	HomeS = 59650,
/// <button class='icomoon' id='\u{e979}' onclick='navigator.clipboard.writeText(this.id)'></button>
	HourGlass = 59769,
/// <button class='icomoon' id='\u{eae4}' onclick='navigator.clipboard.writeText(this.id)'></button>
	HtmlFive = 60132,
/// <button class='icomoon' id='\u{eae5}' onclick='navigator.clipboard.writeText(this.id)'></button>
	HtmlFiveR = 60133,
/// <button class='icomoon' id='\u{eaea}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Icomoon = 60138,
/// <button class='icomoon' id='\u{eadb}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Ie = 60123,
/// <button class='icomoon' id='\u{e90d}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Image = 59661,
/// <button class='icomoon' id='\u{e90e}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Images = 59662,
/// <button class='icomoon' id='\u{ea7c}' onclick='navigator.clipboard.writeText(this.id)'></button>
	IndentDecrease = 60028,
/// <button class='icomoon' id='\u{ea7b}' onclick='navigator.clipboard.writeText(this.id)'></button>
	IndentIncrease = 60027,
/// <button class='icomoon' id='\u{ea2f}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Infinite = 59951,
/// <button class='icomoon' id='\u{ea0c}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Info = 59916,
/// <button class='icomoon' id='\u{ea72}' onclick='navigator.clipboard.writeText(this.id)'></button>
	InsertTemplate = 60018,
/// <button class='icomoon' id='\u{ea92}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Instagram = 60050,
/// <button class='icomoon' id='\u{ea64}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Italic = 60004,
/// <button class='icomoon' id='\u{eab5}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Joomla = 60085,
/// <button class='icomoon' id='\u{e98d}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Key = 59789,
/// <button class='icomoon' id='\u{e98e}' onclick='navigator.clipboard.writeText(this.id)'></button>
	KeyR = 59790,
/// <button class='icomoon' id='\u{e955}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Keyboard = 59733,
/// <button class='icomoon' id='\u{e9aa}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Lab = 59818,
/// <button class='icomoon' id='\u{eaa2}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Lanyrd = 60066,
/// <button class='icomoon' id='\u{e957}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Laptop = 59735,
/// <button class='icomoon' id='\u{ea22}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Last = 59938,
/// <button class='icomoon' id='\u{eacb}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Lastfm = 60107,
/// <button class='icomoon' id='\u{eacc}' onclick='navigator.clipboard.writeText(this.id)'></button>
	LastfmR = 60108,
/// <button class='icomoon' id='\u{e9a4}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Leaf = 59812,
/// <button class='icomoon' id='\u{e921}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Library = 59681,
/// <button class='icomoon' id='\u{eae3}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Libreoffice = 60131,
/// <button class='icomoon' id='\u{e941}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Lifebuoy = 59713,
/// <button class='icomoon' id='\u{ea5d}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Ligature = 59997,
/// <button class='icomoon' id='\u{ea5e}' onclick='navigator.clipboard.writeText(this.id)'></button>
	LigatureR = 59998,
/// <button class='icomoon' id='\u{e9cb}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Link = 59851,
/// <button class='icomoon' id='\u{eac9}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Linkedin = 60105,
/// <button class='icomoon' id='\u{eaca}' onclick='navigator.clipboard.writeText(this.id)'></button>
	LinkedinR = 60106,
/// <button class='icomoon' id='\u{e9ba}' onclick='navigator.clipboard.writeText(this.id)'></button>
	List = 59834,
/// <button class='icomoon' id='\u{e9bb}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ListR = 59835,
/// <button class='icomoon' id='\u{e9b9}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ListNumbered = 59833,
/// <button class='icomoon' id='\u{e947}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Location = 59719,
/// <button class='icomoon' id='\u{e948}' onclick='navigator.clipboard.writeText(this.id)'></button>
	LocationR = 59720,
/// <button class='icomoon' id='\u{e98f}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Lock = 59791,
/// <button class='icomoon' id='\u{ea2d}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Loop = 59949,
/// <button class='icomoon' id='\u{ea2e}' onclick='navigator.clipboard.writeText(this.id)'></button>
	LoopR = 59950,
/// <button class='icomoon' id='\u{ea74}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Ltr = 60020,
/// <button class='icomoon' id='\u{e997}' onclick='navigator.clipboard.writeText(this.id)'></button>
	MagicWand = 59799,
/// <button class='icomoon' id='\u{e9ab}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Magnet = 59819,
/// <button class='icomoon' id='\u{ea83}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Mail = 60035,
/// <button class='icomoon' id='\u{ea84}' onclick='navigator.clipboard.writeText(this.id)'></button>
	MailR = 60036,
/// <button class='icomoon' id='\u{ea85}' onclick='navigator.clipboard.writeText(this.id)'></button>
	MailS = 60037,
/// <button class='icomoon' id='\u{ea86}' onclick='navigator.clipboard.writeText(this.id)'></button>
	MailT = 60038,
/// <button class='icomoon' id='\u{ea58}' onclick='navigator.clipboard.writeText(this.id)'></button>
	MakeGroup = 59992,
/// <button class='icomoon' id='\u{e9dc}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Man = 59868,
/// <button class='icomoon' id='\u{e9de}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ManWoman = 59870,
/// <button class='icomoon' id='\u{e94b}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Map = 59723,
/// <button class='icomoon' id='\u{e94c}' onclick='navigator.clipboard.writeText(this.id)'></button>
	MapR = 59724,
/// <button class='icomoon' id='\u{e9bd}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Menu = 59837,
/// <button class='icomoon' id='\u{e9be}' onclick='navigator.clipboard.writeText(this.id)'></button>
	MenuR = 59838,
/// <button class='icomoon' id='\u{e9bf}' onclick='navigator.clipboard.writeText(this.id)'></button>
	MenuS = 59839,
/// <button class='icomoon' id='\u{e9c0}' onclick='navigator.clipboard.writeText(this.id)'></button>
	MenuT = 59840,
/// <button class='icomoon' id='\u{e9a6}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Meter = 59814,
/// <button class='icomoon' id='\u{e9a7}' onclick='navigator.clipboard.writeText(this.id)'></button>
	MeterR = 59815,
/// <button class='icomoon' id='\u{e91e}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Mic = 59678,
/// <button class='icomoon' id='\u{ea0b}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Minus = 59915,
/// <button class='icomoon' id='\u{e958}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Mobile = 59736,
/// <button class='icomoon' id='\u{e959}' onclick='navigator.clipboard.writeText(this.id)'></button>
	MobileR = 59737,
/// <button class='icomoon' id='\u{ea47}' onclick='navigator.clipboard.writeText(this.id)'></button>
	MoveDown = 59975,
/// <button class='icomoon' id='\u{ea46}' onclick='navigator.clipboard.writeText(this.id)'></button>
	MoveUp = 59974,
/// <button class='icomoon' id='\u{e9a2}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Mug = 59810,
/// <button class='icomoon' id='\u{e911}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Music = 59665,
/// <button class='icomoon' id='\u{e9f7}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Neutral = 59895,
/// <button class='icomoon' id='\u{e9f8}' onclick='navigator.clipboard.writeText(this.id)'></button>
	NeutralR = 59896,
/// <button class='icomoon' id='\u{e904}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Newspaper = 59652,
/// <button class='icomoon' id='\u{ea7e}' onclick='navigator.clipboard.writeText(this.id)'></button>
	NewTab = 60030,
/// <button class='icomoon' id='\u{ea19}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Next = 59929,
/// <button class='icomoon' id='\u{ea24}' onclick='navigator.clipboard.writeText(this.id)'></button>
	NextR = 59940,
/// <button class='icomoon' id='\u{ea08}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Notification = 59912,
/// <button class='icomoon' id='\u{eab1}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Npm = 60081,
/// <button class='icomoon' id='\u{e903}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Office = 59651,
/// <button class='icomoon' id='\u{ea66}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Omega = 60006,
/// <button class='icomoon' id='\u{eaaf}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Onedrive = 60079,
/// <button class='icomoon' id='\u{eade}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Opera = 60126,
/// <button class='icomoon' id='\u{ea51}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Opt = 59985,
/// <button class='icomoon' id='\u{e916}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Pacman = 59670,
/// <button class='icomoon' id='\u{ea6e}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Pagebreak = 60014,
/// <button class='icomoon' id='\u{ea68}' onclick='navigator.clipboard.writeText(this.id)'></button>
	PageBreak = 60008,
/// <button class='icomoon' id='\u{e90c}' onclick='navigator.clipboard.writeText(this.id)'></button>
	PaintFormat = 59660,
/// <button class='icomoon' id='\u{ea78}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ParagraphCenter = 60024,
/// <button class='icomoon' id='\u{ea7a}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ParagraphJustify = 60026,
/// <button class='icomoon' id='\u{ea77}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ParagraphLeft = 60023,
/// <button class='icomoon' id='\u{ea79}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ParagraphRight = 60025,
/// <button class='icomoon' id='\u{e92d}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Paste = 59693,
/// <button class='icomoon' id='\u{ea16}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Pause = 59926,
/// <button class='icomoon' id='\u{ea1d}' onclick='navigator.clipboard.writeText(this.id)'></button>
	PauseR = 59933,
/// <button class='icomoon' id='\u{ead8}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Paypal = 60120,
/// <button class='icomoon' id='\u{e908}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Pen = 59656,
/// <button class='icomoon' id='\u{e905}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Pencil = 59653,
/// <button class='icomoon' id='\u{e906}' onclick='navigator.clipboard.writeText(this.id)'></button>
	PencilR = 59654,
/// <button class='icomoon' id='\u{e942}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Phone = 59714,
/// <button class='icomoon' id='\u{e943}' onclick='navigator.clipboard.writeText(this.id)'></button>
	PhoneHangUp = 59715,
/// <button class='icomoon' id='\u{e99a}' onclick='navigator.clipboard.writeText(this.id)'></button>
	PieChart = 59802,
/// <button class='icomoon' id='\u{ea73}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Pilcrow = 60019,
/// <button class='icomoon' id='\u{ead1}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Pinterest = 60113,
/// <button class='icomoon' id='\u{ead2}' onclick='navigator.clipboard.writeText(this.id)'></button>
	PinterestR = 60114,
/// <button class='icomoon' id='\u{e912}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Play = 59666,
/// <button class='icomoon' id='\u{ea15}' onclick='navigator.clipboard.writeText(this.id)'></button>
	PlayR = 59925,
/// <button class='icomoon' id='\u{ea1c}' onclick='navigator.clipboard.writeText(this.id)'></button>
	PlayS = 59932,
/// <button class='icomoon' id='\u{ea0a}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Plus = 59914,
/// <button class='icomoon' id='\u{e91c}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Podcast = 59676,
/// <button class='icomoon' id='\u{ea05}' onclick='navigator.clipboard.writeText(this.id)'></button>
	PointDown = 59909,
/// <button class='icomoon' id='\u{ea06}' onclick='navigator.clipboard.writeText(this.id)'></button>
	PointLeft = 59910,
/// <button class='icomoon' id='\u{ea04}' onclick='navigator.clipboard.writeText(this.id)'></button>
	PointRight = 59908,
/// <button class='icomoon' id='\u{ea03}' onclick='navigator.clipboard.writeText(this.id)'></button>
	PointUp = 59907,
/// <button class='icomoon' id='\u{e9b5}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Power = 59829,
/// <button class='icomoon' id='\u{e9b7}' onclick='navigator.clipboard.writeText(this.id)'></button>
	PowerCord = 59831,
/// <button class='icomoon' id='\u{ea18}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Previous = 59928,
/// <button class='icomoon' id='\u{ea23}' onclick='navigator.clipboard.writeText(this.id)'></button>
	PreviousR = 59939,
/// <button class='icomoon' id='\u{e935}' onclick='navigator.clipboard.writeText(this.id)'></button>
	PriceTag = 59701,
/// <button class='icomoon' id='\u{e936}' onclick='navigator.clipboard.writeText(this.id)'></button>
	PriceTags = 59702,
/// <button class='icomoon' id='\u{e954}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Printer = 59732,
/// <button class='icomoon' id='\u{e923}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Profile = 59683,
/// <button class='icomoon' id='\u{e946}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Pushpin = 59718,
/// <button class='icomoon' id='\u{e938}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Qrcode = 59704,
/// <button class='icomoon' id='\u{ea09}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Question = 59913,
/// <button class='icomoon' id='\u{e907}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Quill = 59655,
/// <button class='icomoon' id='\u{e977}' onclick='navigator.clipboard.writeText(this.id)'></button>
	QuotesLeft = 59767,
/// <button class='icomoon' id='\u{e978}' onclick='navigator.clipboard.writeText(this.id)'></button>
	QuotesRight = 59768,
/// <button class='icomoon' id='\u{ea54}' onclick='navigator.clipboard.writeText(this.id)'></button>
	RadioChecked = 59988,
/// <button class='icomoon' id='\u{ea55}' onclick='navigator.clipboard.writeText(this.id)'></button>
	RadioCheckedR = 59989,
/// <button class='icomoon' id='\u{ea56}' onclick='navigator.clipboard.writeText(this.id)'></button>
	RadioUnchecked = 59990,
/// <button class='icomoon' id='\u{eac6}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Reddit = 60102,
/// <button class='icomoon' id='\u{e966}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Redo = 59750,
/// <button class='icomoon' id='\u{e968}' onclick='navigator.clipboard.writeText(this.id)'></button>
	RedoR = 59752,
/// <button class='icomoon' id='\u{ea99}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Renren = 60057,
/// <button class='icomoon' id='\u{e96a}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Reply = 59754,
/// <button class='icomoon' id='\u{e9b1}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Road = 59825,
/// <button class='icomoon' id='\u{e9a5}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Rocket = 59813,
/// <button class='icomoon' id='\u{ea9b}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Rss = 60059,
/// <button class='icomoon' id='\u{ea9c}' onclick='navigator.clipboard.writeText(this.id)'></button>
	RssR = 60060,
/// <button class='icomoon' id='\u{ea75}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Rtl = 60021,
/// <button class='icomoon' id='\u{eaab}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SUPPpx = 60075,
/// <button class='icomoon' id='\u{e9e5}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Sad = 59877,
/// <button class='icomoon' id='\u{e9e6}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SadR = 59878,
/// <button class='icomoon' id='\u{eadd}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Safari = 60125,
/// <button class='icomoon' id='\u{ea5a}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Scissors = 59994,
/// <button class='icomoon' id='\u{e986}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Search = 59782,
/// <button class='icomoon' id='\u{ea76}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Section = 60022,
/// <button class='icomoon' id='\u{ea7d}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Share = 60029,
/// <button class='icomoon' id='\u{ea82}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ShareR = 60034,
/// <button class='icomoon' id='\u{e9b4}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Shield = 59828,
/// <button class='icomoon' id='\u{ea4f}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Shift = 59983,
/// <button class='icomoon' id='\u{e9f1}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Shocked = 59889,
/// <button class='icomoon' id='\u{e9f2}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ShockedR = 59890,
/// <button class='icomoon' id='\u{e98a}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Shrink = 59786,
/// <button class='icomoon' id='\u{e98c}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ShrinkR = 59788,
/// <button class='icomoon' id='\u{ea30}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Shuffle = 59952,
/// <button class='icomoon' id='\u{ea67}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Sigma = 60007,
/// <button class='icomoon' id='\u{ea9a}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SinaWeibo = 60058,
/// <button class='icomoon' id='\u{eac5}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Skype = 60101,
/// <button class='icomoon' id='\u{e9fd}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Sleepy = 59901,
/// <button class='icomoon' id='\u{e9fe}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SleepyR = 59902,
/// <button class='icomoon' id='\u{e9e1}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Smile = 59873,
/// <button class='icomoon' id='\u{e9e2}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SmileR = 59874,
/// <button class='icomoon' id='\u{ea48}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SortAlphaAsc = 59976,
/// <button class='icomoon' id='\u{ea49}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SortAlphaDesc = 59977,
/// <button class='icomoon' id='\u{ea4c}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SortAmountAsc = 59980,
/// <button class='icomoon' id='\u{ea4d}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SortAmountDesc = 59981,
/// <button class='icomoon' id='\u{ea4b}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SortNumbericDesc = 59979,
/// <button class='icomoon' id='\u{ea4a}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SortNumericAsc = 59978,
/// <button class='icomoon' id='\u{eac3}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Soundcloud = 60099,
/// <button class='icomoon' id='\u{eac4}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SoundcloudR = 60100,
/// <button class='icomoon' id='\u{e917}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Spades = 59671,
/// <button class='icomoon' id='\u{ea12}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SpellCheck = 59922,
/// <button class='icomoon' id='\u{e9c9}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Sphere = 59849,
/// <button class='icomoon' id='\u{e97a}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Spinner = 59770,
/// <button class='icomoon' id='\u{e983}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SpinnerQP = 59779,
/// <button class='icomoon' id='\u{e984}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SpinnerQQ = 59780,
/// <button class='icomoon' id='\u{e97b}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SpinnerR = 59771,
/// <button class='icomoon' id='\u{e97c}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SpinnerS = 59772,
/// <button class='icomoon' id='\u{e97d}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SpinnerT = 59773,
/// <button class='icomoon' id='\u{e97e}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SpinnerU = 59774,
/// <button class='icomoon' id='\u{e97f}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SpinnerV = 59775,
/// <button class='icomoon' id='\u{e980}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SpinnerW = 59776,
/// <button class='icomoon' id='\u{e981}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SpinnerX = 59777,
/// <button class='icomoon' id='\u{e982}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SpinnerY = 59778,
/// <button class='icomoon' id='\u{e9a3}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SpoonKnife = 59811,
/// <button class='icomoon' id='\u{ea94}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Spotify = 60052,
/// <button class='icomoon' id='\u{e92e}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Stack = 59694,
/// <button class='icomoon' id='\u{ead0}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Stackoverflow = 60112,
/// <button class='icomoon' id='\u{e9d7}' onclick='navigator.clipboard.writeText(this.id)'></button>
	StarEmpty = 59863,
/// <button class='icomoon' id='\u{e9d9}' onclick='navigator.clipboard.writeText(this.id)'></button>
	StarFull = 59865,
/// <button class='icomoon' id='\u{e9d8}' onclick='navigator.clipboard.writeText(this.id)'></button>
	StarHalf = 59864,
/// <button class='icomoon' id='\u{e99c}' onclick='navigator.clipboard.writeText(this.id)'></button>
	StatsBars = 59804,
/// <button class='icomoon' id='\u{e99d}' onclick='navigator.clipboard.writeText(this.id)'></button>
	StatsBarsR = 59805,
/// <button class='icomoon' id='\u{e99b}' onclick='navigator.clipboard.writeText(this.id)'></button>
	StatsDots = 59803,
/// <button class='icomoon' id='\u{eaac}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Steam = 60076,
/// <button class='icomoon' id='\u{eaad}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SteamR = 60077,
/// <button class='icomoon' id='\u{ea17}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Stop = 59927,
/// <button class='icomoon' id='\u{ea1e}' onclick='navigator.clipboard.writeText(this.id)'></button>
	StopR = 59934,
/// <button class='icomoon' id='\u{e952}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Stopwatch = 59730,
/// <button class='icomoon' id='\u{ea65}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Strikethrough = 60005,
/// <button class='icomoon' id='\u{eace}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Stumbleupon = 60110,
/// <button class='icomoon' id='\u{eacf}' onclick='navigator.clipboard.writeText(this.id)'></button>
	StumbleuponR = 60111,
/// <button class='icomoon' id='\u{ea6a}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Subscript = 60010,
/// <button class='icomoon' id='\u{ea6c}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SubscriptR = 60012,
/// <button class='icomoon' id='\u{e9d4}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Sun = 59860,
/// <button class='icomoon' id='\u{ea69}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Superscript = 60009,
/// <button class='icomoon' id='\u{ea6b}' onclick='navigator.clipboard.writeText(this.id)'></button>
	SuperscriptR = 60011,
/// <button class='icomoon' id='\u{eae9}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Svg = 60137,
/// <button class='icomoon' id='\u{e9b6}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Switch = 59830,
/// <button class='icomoon' id='\u{ea45}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Tab = 59973,
/// <button class='icomoon' id='\u{ea70}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Table = 60016,
/// <button class='icomoon' id='\u{ea71}' onclick='navigator.clipboard.writeText(this.id)'></button>
	TableR = 60017,
/// <button class='icomoon' id='\u{e95a}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Tablet = 59738,
/// <button class='icomoon' id='\u{e9b3}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Target = 59827,
/// <button class='icomoon' id='\u{ea95}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Telegram = 60053,
/// <button class='icomoon' id='\u{ea81}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Terminal = 60033,
/// <button class='icomoon' id='\u{ea6d}' onclick='navigator.clipboard.writeText(this.id)'></button>
	TextColor = 60013,
/// <button class='icomoon' id='\u{ea5f}' onclick='navigator.clipboard.writeText(this.id)'></button>
	TextHeight = 59999,
/// <button class='icomoon' id='\u{ea60}' onclick='navigator.clipboard.writeText(this.id)'></button>
	TextWidth = 60000,
/// <button class='icomoon' id='\u{e939}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Ticket = 59705,
/// <button class='icomoon' id='\u{e9e3}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Tongue = 59875,
/// <button class='icomoon' id='\u{e9e4}' onclick='navigator.clipboard.writeText(this.id)'></button>
	TongueR = 59876,
/// <button class='icomoon' id='\u{e9bc}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Tree = 59836,
/// <button class='icomoon' id='\u{eab3}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Trello = 60083,
/// <button class='icomoon' id='\u{e99e}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Trophy = 59806,
/// <button class='icomoon' id='\u{e9b0}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Truck = 59824,
/// <button class='icomoon' id='\u{eab9}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Tumblr = 60089,
/// <button class='icomoon' id='\u{eaba}' onclick='navigator.clipboard.writeText(this.id)'></button>
	TumblrR = 60090,
/// <button class='icomoon' id='\u{eabd}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Tux = 60093,
/// <button class='icomoon' id='\u{e95b}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Tv = 59739,
/// <button class='icomoon' id='\u{ea9f}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Twitch = 60063,
/// <button class='icomoon' id='\u{ea96}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Twitter = 60054,
/// <button class='icomoon' id='\u{ea63}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Underline = 60003,
/// <button class='icomoon' id='\u{e965}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Undo = 59749,
/// <button class='icomoon' id='\u{e967}' onclick='navigator.clipboard.writeText(this.id)'></button>
	UndoR = 59751,
/// <button class='icomoon' id='\u{ea59}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Ungroup = 59993,
/// <button class='icomoon' id='\u{e990}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Unlocked = 59792,
/// <button class='icomoon' id='\u{e961}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Upload = 59745,
/// <button class='icomoon' id='\u{e9c6}' onclick='navigator.clipboard.writeText(this.id)'></button>
	UploadR = 59846,
/// <button class='icomoon' id='\u{e9c8}' onclick='navigator.clipboard.writeText(this.id)'></button>
	UploadS = 59848,
/// <button class='icomoon' id='\u{e971}' onclick='navigator.clipboard.writeText(this.id)'></button>
	User = 59761,
/// <button class='icomoon' id='\u{e972}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Users = 59762,
/// <button class='icomoon' id='\u{e975}' onclick='navigator.clipboard.writeText(this.id)'></button>
	UserCheck = 59765,
/// <button class='icomoon' id='\u{e974}' onclick='navigator.clipboard.writeText(this.id)'></button>
	UserMinus = 59764,
/// <button class='icomoon' id='\u{e973}' onclick='navigator.clipboard.writeText(this.id)'></button>
	UserPlus = 59763,
/// <button class='icomoon' id='\u{e976}' onclick='navigator.clipboard.writeText(this.id)'></button>
	UserTie = 59766,
/// <button class='icomoon' id='\u{e914}' onclick='navigator.clipboard.writeText(this.id)'></button>
	VideoCamera = 59668,
/// <button class='icomoon' id='\u{eaa0}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Vimeo = 60064,
/// <button class='icomoon' id='\u{eaa1}' onclick='navigator.clipboard.writeText(this.id)'></button>
	VimeoR = 60065,
/// <button class='icomoon' id='\u{ea97}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Vine = 60055,
/// <button class='icomoon' id='\u{ea98}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Vk = 60056,
/// <button class='icomoon' id='\u{ea2c}' onclick='navigator.clipboard.writeText(this.id)'></button>
	VolumeDecrease = 59948,
/// <button class='icomoon' id='\u{ea26}' onclick='navigator.clipboard.writeText(this.id)'></button>
	VolumeHigh = 59942,
/// <button class='icomoon' id='\u{ea2b}' onclick='navigator.clipboard.writeText(this.id)'></button>
	VolumeIncrease = 59947,
/// <button class='icomoon' id='\u{ea28}' onclick='navigator.clipboard.writeText(this.id)'></button>
	VolumeLow = 59944,
/// <button class='icomoon' id='\u{ea27}' onclick='navigator.clipboard.writeText(this.id)'></button>
	VolumeMedium = 59943,
/// <button class='icomoon' id='\u{ea29}' onclick='navigator.clipboard.writeText(this.id)'></button>
	VolumeMute = 59945,
/// <button class='icomoon' id='\u{ea2a}' onclick='navigator.clipboard.writeText(this.id)'></button>
	VolumeMuteR = 59946,
/// <button class='icomoon' id='\u{ea07}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Warning = 59911,
/// <button class='icomoon' id='\u{ea93}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Whatsapp = 60051,
/// <button class='icomoon' id='\u{eac8}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Wikipedia = 60104,
/// <button class='icomoon' id='\u{eac1}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Windows = 60097,
/// <button class='icomoon' id='\u{eac2}' onclick='navigator.clipboard.writeText(this.id)'></button>
	WindowsX = 60098,
/// <button class='icomoon' id='\u{e9e7}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Wink = 59879,
/// <button class='icomoon' id='\u{e9e8}' onclick='navigator.clipboard.writeText(this.id)'></button>
	WinkR = 59880,
/// <button class='icomoon' id='\u{e9dd}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Woman = 59869,
/// <button class='icomoon' id='\u{e9fb}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Wondering = 59899,
/// <button class='icomoon' id='\u{e9fc}' onclick='navigator.clipboard.writeText(this.id)'></button>
	WonderingR = 59900,
/// <button class='icomoon' id='\u{eab4}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Wordpress = 60084,
/// <button class='icomoon' id='\u{e991}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Wrench = 59793,
/// <button class='icomoon' id='\u{ead3}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Xing = 60115,
/// <button class='icomoon' id='\u{ead4}' onclick='navigator.clipboard.writeText(this.id)'></button>
	XingR = 60116,
/// <button class='icomoon' id='\u{eabb}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Yahoo = 60091,
/// <button class='icomoon' id='\u{eabc}' onclick='navigator.clipboard.writeText(this.id)'></button>
	YahooR = 60092,
/// <button class='icomoon' id='\u{ead7}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Yelp = 60119,
/// <button class='icomoon' id='\u{ea9d}' onclick='navigator.clipboard.writeText(this.id)'></button>
	Youtube = 60061,
/// <button class='icomoon' id='\u{ea9e}' onclick='navigator.clipboard.writeText(this.id)'></button>
	YoutubeR = 60062,
/// <button class='icomoon' id='\u{e987}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ZoomIn = 59783,
/// <button class='icomoon' id='\u{e988}' onclick='navigator.clipboard.writeText(this.id)'></button>
	ZoomOut = 59784,
}

impl From<IcoMoon> for char{
    fn from(icon: IcoMoon) -> char {
        unsafe{char::from_u32_unchecked(icon as u32)}
    }
}
