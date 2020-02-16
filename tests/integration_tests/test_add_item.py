from proto import spq_pb2

def test_add_an_item(spq_client):
    request = spq_pb2.AddItemRequest(
            item=bytes("item", 'utf-8'),
            features=[{"name":"feature_name", "value":0}]
            )
    result = spq_client.AddItem(request)

    assert result.size == 1
