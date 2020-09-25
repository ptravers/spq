import pytest
import grpc
from proto import spq_pb2_grpc, spq_pb2


@pytest.fixture(scope="session")
def channel():
    return grpc.insecure_channel("spq:9090")


@pytest.fixture(scope="session")
def health_client(channel):
    return spq_pb2_grpc.HealthServiceStub(channel)


@pytest.fixture(scope="session")
def spq_client(channel):
    return spq_pb2_grpc.SortingPriorityQueueServiceStub(channel)


@pytest.fixture(scope="session")
def queue_name(spq_client):
    request = spq_pb2.CreateQueueRequest(
        name="test queue", queueType=spq_pb2.DURABLE, features=["feature_name"]
    )

    created_queue_response = spq_client.CreateQueue(request)

    return created_queue_response.name
