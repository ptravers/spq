from proto import spq_pb2


def test_check_health(health_client):
    response = health_client.Check(spq_pb2.HealthCheckRequest())
    assert response.status == 1


def test_watch_health(health_client):
    total_events = 0

    for response in health_client.Watch(spq_pb2.HealthCheckRequest()):
        total_events += 1
        assert response.status == 1

        if total_events >= 2:
            break
