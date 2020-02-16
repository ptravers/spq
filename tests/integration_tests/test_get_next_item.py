from proto import spq_pb2


def test_get_next_item(spq_client):
    sent_item = bytes("item", 'utf-8')

    request = spq_pb2.AddItemRequest(
            item=sent_item,
            features=[{"name":"feature_name", "value":0}]
            )

    add_item_result = spq_client.AddItem(request)

    result = spq_client.GetNextItem(spq_pb2.GetItemRequest())

    assert result.hasItem == True
    assert result.item == sent_item
    assert result.size == add_item_result.size - 1
