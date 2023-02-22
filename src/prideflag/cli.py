from argparse import ArgumentParser
from argparse import Namespace

from prideflag.flag import make_flag
from prideflag.pride import choices
from prideflag.pride import get_flag


def get_args(argv: list[str] | None) -> Namespace:
	parser = ArgumentParser()
	parser.add_argument("--flag", choices=choices, required=False)

	return parser.parse_args(argv)


def main_debug(argv: list[str] | None = None) -> int:
	args = get_args(argv)

	print(make_flag(*get_flag(args.flag)))

	return 0


def main() -> int:
	return main_debug()
