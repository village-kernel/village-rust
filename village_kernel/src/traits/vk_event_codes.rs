//###########################################################################
// vk_event_codes.rs
// Declarations of the interface
// https://github.com/torvalds/linux/blob/master/include/uapi/linux/input-event-codes.h
//
// $Copyright: Copyright (C) village
//###########################################################################

// EventCode
pub enum EventCode
{
    KeyReserved             = 0,
    KeyESC                  = 1,
    Key1                    = 2,
    Key2                    = 3,
    Key3                    = 4,
    Key4                    = 5,
    Key5                    = 6,
    Key6                    = 7,
    Key7                    = 8,
    Key8                    = 9,
    Key9                    = 10,
    Key0                    = 11,
    KeyMinus                = 12, /* '-' */
    KeyEqual                = 13, /* '=' */
    KeyBackSpace            = 14,
    KeyTab                  = 15,
    KeyQ                    = 16,
    KeyW                    = 17,
    KeyE                    = 18,
    KeyR                    = 19,
    KeyT                    = 20,
    KeyY                    = 21,
    KeyU                    = 22,
    KeyI                    = 23,
    KeyO                    = 24,
    KeyP                    = 25,
    KeyLeftBrace            = 26, /* '[' */
    KeyRightBrace           = 27, /* ']' */
    KeyEnter                = 28,
    KeyLeftCtrl             = 29,
    KeyA                    = 30,
    KeyS                    = 31,
    KeyD                    = 32,
    KeyF                    = 33,
    KeyG                    = 34,
    KeyH                    = 35,
    KeyJ                    = 36,
    KeyK                    = 37,
    KeyL                    = 38,
    KeySemicolon            = 39, /* ';' */
    KeyApostrophe           = 40, /* ''' */
    KeyGrave                = 41, /* '`' */
    KeyLeftShift            = 42,
    KeyBackSlash            = 43, /* '\' */
    KeyZ                    = 44,
    KeyX                    = 45,
    KeyC                    = 46,
    KeyV                    = 47,
    KeyB                    = 48,
    KeyN                    = 49,
    KeyM                    = 50,
    KeyComma                = 51, /* ',' */
    KeyDot                  = 52, /* '.' */
    KeySlash                = 53, /* '/' */
    KeyRightShift           = 54,
    KeyKPAsterisk           = 55, /* '*' */
    KeyLeftAlt              = 56,
    KeySpace                = 57,
    KeyCapsLock             = 58,
    KeyF1                   = 59,
    KeyF2                   = 60,
    KeyF3                   = 61,
    KeyF4                   = 62,
    KeyF5                   = 63,
    KeyF6                   = 64,
    KeyF7                   = 65,
    KeyF8                   = 66,
    KeyF9                   = 67,
    KeyF10                  = 68,
    KeyNumLock              = 69,
    KeyScrollLock           = 70,
    KeyKP7                  = 71,
    KeyKP8                  = 72,
    KeyKP9                  = 73,
    KeyKPMinus              = 74, /* '-' */
    KeyKP4                  = 75,
    KeyKP5                  = 76,
    KeyKP6                  = 77,
    KeyKPPlus               = 78, /* '+' */
    KeyKP1                  = 79,
    KeyKP2                  = 80,
    KeyKP3                  = 81,
    KeyKP0                  = 82,
    KeyKPDot                = 83,
    KeyZenkakuHankaku       = 84,

    Key102ND                = 86,
    KeyF11                  = 87,
    KeyF12                  = 88,
    KeyRO                   = 89,
    KeyKatakana             = 90,
    KeyHiragana             = 91,
    KeyHenkan               = 92,
    KeyKatakanaHiragana     = 93,
    KeyMuhenkan             = 94,
    KeyKPJPcomma            = 95, /* ',' */
    KeyKPEnter              = 96,
    KeyRightCtrl            = 97,
    KeyKPSlash              = 98, /* '/' */
    KeySysrq                = 99,
    KeyRightAlt             = 100,
    KeyLineFeed             = 101,
    KeyHome                 = 102,
    KeyUp                   = 103,
    KeyPageUp               = 104,
    KeyLeft                 = 105,
    KeyRight                = 106,
    KeyEnd                  = 107,
    KeyDown                 = 108,
    KeyPageDown             = 109,
    KeyInsert               = 110,
    KeyDelete               = 111,
    KeyMacro                = 112,
    KeyMute                 = 113,
    KeyVolumeDown           = 114,
    KeyVolumeUp             = 115,
    KeyPower                = 116, /* SC System Power Down */
    KeyKPEqual              = 117,
    KeyKPPlusMinus          = 118,
    KeyPause                = 119,
    KeyScale                = 120, /* AL Compiz Scale (Expose) */

    KeyKPComma              = 121, /* ',' */
    KeyHangeul              = 122,
    //KeyHanguel              = KeyHangeul,
    KeyHanja                = 123,
    KeyYen                  = 124,
    KeyLeftMeta             = 125,
    KeyRightMeta            = 126,
    KeyCompose              = 127,

    KeyStop                 = 128, /* AC Stop */
    KeyAgain                = 129,
    KeyProps                = 130, /* AC Properties */
    KeyUndo                 = 131, /* AC Undo */
    KeyFront                = 132,
    KeyCopy                 = 133, /* AC Copy */
    KeyOpen                 = 134, /* AC Open */
    KeyPaste                = 135, /* AC Paste */
    KeyFind                 = 136, /* AC Search */
    KeyCut                  = 137, /* AC Cut */
    KeyHelp                 = 138, /* AL Integrated Help Center */
    KeyMenu                 = 139, /* Menu (show menu) */
    KeyCalc                 = 140, /* AL Calculator */
    KeySetup                = 141,
    KeySleep                = 142, /* SC System Sleep */
    KeyWakeup               = 143, /* System Wake Up */
    KeyFile                 = 144, /* AL Local Machine Browser */
    KeySendFile             = 145,
    KeyDeleteFile           = 146,
    KeyXFER                 = 147,
    KeyProg1                = 148,
    KeyProg2                = 149,
    KeyWWW                  = 150, /* AL Internet Browser */
    KeyMSDOS                = 151, 
    KeyCoffee               = 152, /* AL Terminal Lock/Screen saver */
    //KeyScreenLock           = KeyCoffee,
    KeyRotateDisplay        = 153, /* Display orientation for e.g. tablets */
    //KeyDirection            = KeyRotateDisplay,
    KeyCycleWindows         = 154,
    KeyMail                 = 155,
    KeyBookMarks            = 156, /* AC Bookmarks */
    KeyComputer             = 157,
    KeyBack                 = 158, /* AC Back */
    KeyForward              = 159, /* AC Forward */
    KeyCloseCD              = 160,
    KeyEjectCD              = 161,
    KeyEjectCloseCD         = 162,
    KeyNextSong             = 163,
    KeyPlayPause            = 164,
    KeyPreviousSong         = 165,
    KeyStopCD               = 166,
    KeyRecord               = 167,
    KeyRewind               = 168,
    KeyPhone                = 169, /* Media Select Telephone */
    KeyISO                  = 170,
    KeyConfig               = 171, /* AL Consumer Control Configuration */
    KeyHomePage             = 172, /* AC Home */
    KeyRefresh              = 173, /* AC Refresh */
    KeyExit                 = 174, /* AC Exit */
    KeyMove                 = 175,
    KeyEdit                 = 176,
    KeyScrollUp             = 177,
    KeyScrollDown           = 178,
    KeyKPLeftParen          = 179,
    KeyKPRightParen         = 180,
    KeyNew                  = 181, /* AC New */
    KeyRedo                 = 182, /* AC Redo/Repeat */

    KeyF13                  = 183,
    KeyF14                  = 184,
    KeyF15                  = 185,
    KeyF16                  = 186,
    KeyF17                  = 187,
    KeyF18                  = 188,
    KeyF19                  = 189,
    KeyF20                  = 190,
    KeyF21                  = 191,
    KeyF22                  = 192,
    KeyF23                  = 193,
    KeyF24                  = 194,

    KeyPlayCD               = 200,
    KeyPauseCD              = 201,
    KeyProg3                = 202,
    KeyProg4                = 203,
    KeyAllApplications      = 204,    /* AC Desktop Show All Applications */
    //KeyDashboard            = KeyAllApplications,
    KeySuspend              = 205,
    KeyClose                = 206, /* AC Close */
    KeyPlay                 = 207,
    KeyFastForward          = 208,
    KeyBassBoost            = 209,
    KeyPrint                = 210, /* AC Print */
    KeyHP                   = 211,
    KeyCamera               = 212,
    KeySound                = 213,
    KeyQuestion             = 214,
    KeyEmail                = 215,
    KeyChat                 = 216,
    KeySearch               = 217,
    KeyConnect              = 218,
    KeyFinance              = 219, /* AL Checkbook/Finance */
    KeySport                = 220,
    KeyShop                 = 221,
    KeyAlterase             = 222,
    KeyCancel               = 223, /* AC Cancel */
    KeyBrightnessDown       = 224,
    KeyBrightnessUp         = 225,
    KeyMedia                = 226,
    
    KeySwitchVideoMode      = 227, /* Cycle between available video outputs (Monitor/LCD/TV-out/etc) */
    KeyKBDillumToggle       = 228,
    KeyKBDillumDown         = 229,
    KeyKBDillumUp           = 230,

    KeySend                 = 231, /* AC Send */
    KeyReply                = 232, /* AC Reply */
    KeyForwardMail          = 233, /* AC Forward Msg */
    KeySave                 = 234, /* AC Save */
    KeyDocuments            = 235,

    KeyBattery              = 236,

    KeyBluetooth            = 237,
    KeyWlan                 = 238,
    KeyUWB                  = 239,
    
    KeyUnknow               = 240,
    
    KeyVideoNext            = 241, /* drive next video source */
    KeyVideoPrev            = 242, /* drive previous video source */
    KeyBrightnessCycle      = 243, /* brightness up, after max is min */
    KeyBrightnessAuto       = 244, /* Set Auto Brightness: manual brightness control is off, rely on ambient */
    //KeyBrightnessZero       = KeyBrightnessAuto,
    KeyDisplayOff           = 245, /* display device to off state */
    KeyWWAN                 = 246, /* Wireless WAN (LTE, UMTS, GSM, etc.) */
    //KeyWIMAX                = KeyWWAN,
    KeyRFKill               = 247, /* Key that controls all radios */
    KeyMicMute              = 248, /* Mute / unmute the microphone */

    //BtnMouse                = 0x110,
    BtnLeft                 = 0x110,
    BtnRight                = 0x111,
    BtnMiddle               = 0x112,
    BtnSide                 = 0x113,
    BtnExtra                = 0x114,
    BtnForward              = 0x115,
    BtnBack                 = 0x116,
    BtnTask                 = 0x117,
}

// KeyStatus
pub enum KeyStatus
{
    KeyReleased = 0,
    KeyPressed,
}
