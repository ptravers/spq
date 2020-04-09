from proto import spq_pb2


def test_get_epoch(spq_client):
    sent_item = bytes("item", "utf-8")

    request = spq_pb2.EnqueueRequest(
        item=sent_item, features=[{"name": "feature_name", "value": 0}]
    )

    spq_client.Enqueue(request)

    result = spq_client.GetEpoch(spq_pb2.GetEpochRequest())

    assert result.epoch > 0, result.epoch
