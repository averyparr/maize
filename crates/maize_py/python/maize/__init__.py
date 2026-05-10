from . import _bindings, tile


def hello(name: str) -> str:
    return _bindings.hello(name)
