from proto import spq_pb2


def test_dequeue(spq_client, queue_name):
    sent_item = bytes("item", "utf-8")

    request = spq_pb2.EnqueueRequest(
        queueName=queue_name,
        item=sent_item,
        features=[{"name": "feature_name", "value": 0}],
    )

    add_item_result = spq_client.Enqueue(request)

    result = spq_client.Dequeue(spq_pb2.DequeueRequest(queueName=queue_name))

    assert result.hasItem == True
    assert result.item == sent_item
    assert result.size == add_item_result.size - 1
