from proto import spq_pb2
from helpers import drain_queue


def test_peek_item(spq_client, queue_name):
    drain_queue(spq_client, queue_name)

    sent_item = bytes("peek_item", "utf-8")
    request = spq_pb2.EnqueueRequest(
        item=sent_item,
        features=[{"name": "feature_name", "value": 0}],
        queueName=queue_name,
    )
    add_item_result = spq_client.Enqueue(request)

    peek_item_result = spq_client.Peek(spq_pb2.PeekRequest(queueName=queue_name))

    assert peek_item_result.hasItem == True
    assert peek_item_result.item == sent_item
    assert peek_item_result.size == add_item_result.size


def test_peek_item_does_not_alter_queue(spq_client, queue_name):
    request = spq_pb2.EnqueueRequest(
        item=bytes("item", "utf-8"),
        features=[{"name": "feature_name", "value": 0}],
        queueName=queue_name,
    )
    spq_client.Enqueue(request)

    peek_item_result = spq_client.Peek(spq_pb2.PeekRequest(queueName=queue_name))
    second_peek_item_result = spq_client.Peek(spq_pb2.PeekRequest(queueName=queue_name))

    assert peek_item_result == second_peek_item_result
