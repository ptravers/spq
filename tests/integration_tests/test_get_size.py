from proto import spq_pb2


def test_get_size(spq_client):
    request = spq_pb2.AddItemRequest(
        item=bytes("item", "utf-8"), features=[{"name": "feature_name", "value": 0}]
    )
    add_item_result = spq_client.AddItem(request)

    get_size_result = spq_client.GetSize(spq_pb2.GetSizeRequest())

    assert add_item_result.size == get_size_result.size
