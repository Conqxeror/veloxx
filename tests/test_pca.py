import pytest
from veloxx import PCA

def test_pca_first_component():
    matrix = [
        [1.0, 2.0, 3.0],
        [2.0, 3.0, 4.0],
        [3.0, 4.0, 5.0]
    ]
    pc = PCA.first_component(matrix)
    assert isinstance(pc, list)
    assert len(pc) == 3
