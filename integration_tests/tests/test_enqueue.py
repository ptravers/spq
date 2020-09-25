from proto import spq_pb2


def test_enqueue(spq_client, queue_name):
    request = spq_pb2.EnqueueRequest(
        item=bytes("item", "utf-8"),
        features=[{"name": "feature_name", "value": 0}],
        queueName=queue_name,
    )
    result = spq_client.Enqueue(request)

    assert result.size == 1
