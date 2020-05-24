import pytest
from proto import spq_pb2

@pytest.mark.durability
def test_starts_with_incremented_epoch(spq_client):
    result = spq_client.GetEpoch(spq_pb2.GetEpochRequest())

    assert result.epoch > 0, result.epoch
