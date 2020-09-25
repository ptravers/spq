from proto import spq_pb2


def test_create_queue(spq_client):
    request = spq_pb2.CreateQueueRequest(
        name="other queue",
        queueType=spq_pb2.IN_MEMORY,
        features=["first_feature", "second_feature"],
    )

    created_queue_response = spq_client.CreateQueue(request)

    assert created_queue_response.name == "other queue"
