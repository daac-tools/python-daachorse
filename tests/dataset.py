import os.path

DIRNAME = os.path.dirname(__file__)


def load_words_100000():
    patterns = []
    with open(DIRNAME + '/data/words_100000') as fp:
        for line in fp:
            patterns.append(line.rstrip('\n'))
    return patterns


def load_unidic():
    patterns = []
    with open(DIRNAME + '/data/unidic/unidic') as fp:
        for line in fp:
            patterns.append(line.rstrip('\n'))
    return patterns


def load_sherlock():
    with open(DIRNAME + '/data/sherlock.txt') as fp:
        return fp.read()


def load_wagahaiwa_nekodearu():
    with open(DIRNAME + '/data/wagahaiwa_nekodearu.txt') as fp:
        return fp.read()
