from proto import spq_pb2


def test_peek_item(spq_client):
    sent_item = bytes("peek_item", "utf-8")
    request = spq_pb2.EnqueueRequest(
        item=sent_item, features=[{"name": "feature_name", "value": 0}]
    )
    add_item_result = spq_client.Enqueue(request)

    peek_item_result = spq_client.PeekNextItem(spq_pb2.PeekItemRequest())

    assert peek_item_result.hasItem == True
    assert peek_item_result.item == sent_item
    assert peek_item_result.size == add_item_result.size


def test_peek_item_does_not_alter_queue(spq_client):
    request = spq_pb2.EnqueueRequest(
        item=bytes("item", "utf-8"), features=[{"name": "feature_name", "value": 0}]
    )
    spq_client.Enqueue(request)

    peek_item_result = spq_client.PeekNextItem(spq_pb2.PeekItemRequest())
    second_peek_item_result = spq_client.PeekNextItem(spq_pb2.PeekItemRequest())

    assert peek_item_result == second_peek_item_result
