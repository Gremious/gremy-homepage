use super::*;

macro_rules! def_colors {
    ($($name:ident => $color:expr);+$(;)?) => {$(pub static $name: css::Color = css::Color::from_hex($color);)+};
}

def_colors! {
    debug_blue => 0x00_87_FF_47;

    bg_black => 0x00_0F_14_FF;

	//  probably rgb(255, 0, 90) 20px,
    pink_magenta => 0xE7_5A_7C_FF;

	// softer one that doesn't hurt ur eyes as much
	// rgba(190, 25, 90, 0.5) 20px,
}

