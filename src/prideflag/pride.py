from itertools import chain

from prideflag.colors import AMARANTH_PINK
from prideflag.colors import ATOMIC_TANGERINE
from prideflag.colors import BLUE
from prideflag.colors import COBALT_BLUE
from prideflag.colors import FLIRT
from prideflag.colors import GREEN
from prideflag.colors import INDIGO
from prideflag.colors import MAJORELLE_BLUE
from prideflag.colors import MAUVE
from prideflag.colors import MAYA_BLUE
from prideflag.colors import MINT
from prideflag.colors import ORANGE
from prideflag.colors import PURPLE
from prideflag.colors import RED
from prideflag.colors import RUBY
from prideflag.colors import SINOPIA
from prideflag.colors import SUPER_PINK
from prideflag.colors import TEAL
from prideflag.colors import TURQUOISE
from prideflag.colors import VIOLET
from prideflag.colors import WHITE
from prideflag.colors import YELLOW
from prideflag.flag import FlagBands


flags: dict[tuple[str, ...], FlagBands] = {
	("p", "pride"): (
		RED,
		ORANGE,
		YELLOW,
		GREEN,
		INDIGO,
		VIOLET,
	),
	("l", "lesbian"): (
		SINOPIA,
		ATOMIC_TANGERINE,
		WHITE,
		SUPER_PINK,
		FLIRT,
	),
	("g", "gay"): (
		TEAL,
		TURQUOISE,
		MINT,
		WHITE,
		COBALT_BLUE,
		MAJORELLE_BLUE,
		PURPLE,
	),
	("b", "bi", "bisexual"): (
		RUBY,
		RUBY,
		MAUVE,
		BLUE,
		BLUE,
	),
	("t", "trans", "transgender"): (
		MAYA_BLUE,
		AMARANTH_PINK,
		WHITE,
		AMARANTH_PINK,
		MAYA_BLUE,
	),
}

choices = list(chain(*flags.keys()))


def get_flag(flag_name: str) -> FlagBands:
	for aliases, bands in flags.items():
		if flag_name in aliases:
			return bands
	return flags[("p", "pride")]
