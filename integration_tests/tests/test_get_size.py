from proto import spq_pb2


def test_get_size(spq_client, queue_name):
    request = spq_pb2.EnqueueRequest(
        item=bytes("item", "utf-8"),
        features=[{"name": "feature_name", "value": 0}],
        queueName=queue_name,
    )
    add_item_result = spq_client.Enqueue(request)

    get_size_result = spq_client.GetSize(spq_pb2.GetSizeRequest(queueName=queue_name))

    assert add_item_result.size == get_size_result.size
