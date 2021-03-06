import pytest
from proto import spq_pb2


@pytest.mark.durability
def test_starts_with_incremented_epoch(spq_client, queue_name):
    result = spq_client.GetEpoch(spq_pb2.GetEpochRequest(queueName=queue_name))

    assert result.epoch > 0, result.epoch


@pytest.mark.durability
def test_starts_with_non_zero_size(spq_client, queue_name):
    result = spq_client.GetSize(spq_pb2.GetSizeRequest(queueName=queue_name))

    assert result.size > 0, result.size


@pytest.mark.durability
def test_starts_with_existing_items(spq_client, queue_name):
    result = spq_client.Dequeue(spq_pb2.DequeueRequest(queueName=queue_name))

    assert result.hasItem == True
