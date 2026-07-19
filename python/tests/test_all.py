import pytest
import nea


def test_sum_as_string():
    assert nea.sum_as_string(1, 1) == "2"
