import os
from shutil import get_terminal_size


Band = tuple[int, int, int]
FlagBands = tuple[Band, ...]


def make_esc(r: int, g: int, b: int) -> str:
	return f"\x1b[22;48;2;{r};{g};{b}m"


def make_band(r: int, g: int, b: int) -> str:
	col, _ = get_terminal_size()
	return make_esc(r, g, b) + " " * col + "\x1b[m"


def make_flag(*bands: Band) -> str:
	return os.linesep.join(make_band(*band) for band in bands)
