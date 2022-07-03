from python.src import lib

alphabet = "abcdefghijklmnopqrstuvwxyz"
alphabet_grouped_by_three = [alphabet[i: i + 3] for i in range(len(alphabet) - 2)]


def day_eleven(part: int) -> str:
    content = lib.read_input(year=2015, day=11, part=part)
    if part == 1:
        return _day_eleven_calculate_part_one(content)

    return _day_eleven_calculate_part_two(content)


def _is_valid_part_one(password: str) -> bool:
    contain_alphabet_group = any([x in password for x in alphabet_grouped_by_three])
    contain_disallowed_characters = not any([x in password for x in ['i', 'o', 'l']])
    contain_two_different_char_sets = len(
        set([x for x in [alphabet[i] * 2 for i in range(0, len(alphabet))] if x in password])
    )
    return contain_alphabet_group and contain_disallowed_characters and contain_two_different_char_sets == 2


def increment_character(x: int, characters: [str]) -> [str]:
    current_character_index = alphabet.index(characters[x])
    next_character_index = (current_character_index + 1) % len(alphabet)
    next_character = alphabet[next_character_index]
    characters[x] = next_character
    if next_character_index == 0:
        characters = increment_character(max(0, x - 1), characters)

    return characters


def calculate_next_password(content):
    characters = [c for c in content]
    password = ""
    while not _is_valid_part_one(password):
        characters = increment_character(len(characters) - 1, characters)
        password = "".join(characters)

    return password


def _day_eleven_calculate_part_one(content: str) -> str:
    return calculate_next_password(content)


def _day_eleven_calculate_part_two(content: str) -> str:
    return calculate_next_password(calculate_next_password(content))
