from proto import spq_pb2


def test_get_epoch(spq_client, queue_name):
    sent_item = bytes("item", "utf-8")

    request = spq_pb2.EnqueueRequest(
        item=sent_item,
        features=[{"name": "feature_name", "value": 0}],
        queueName=queue_name,
    )

    spq_client.Enqueue(request)

    result = spq_client.GetEpoch(spq_pb2.GetEpochRequest(queueName=queue_name))

    assert result.epoch > 0, result.epoch
