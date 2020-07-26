FROM python:3.8.1-buster

WORKDIR /app/

COPY integration_tests/requirements-test.txt /app/requirements-test.txt

RUN pip install --upgrade pip
RUN pip install -r requirements-test.txt

COPY server/proto/spq.proto /app/tests/proto/spq.proto
COPY integration_tests/tests/proto/ /app/tests/proto/

RUN cd /app/tests/ && \
  python3 -m grpc_tools.protoc -I . --python_out=. --grpc_python_out=. proto/spq.proto

COPY integration_tests/tests/ /app/tests/

COPY integration_tests/entrypoint.sh /app/tests/entrypoint.sh
RUN chmod +x /app/tests/entrypoint.sh

COPY integration_tests/run_tests.sh /app/tests/run_tests.sh
RUN chmod +x /app/tests/run_tests.sh

RUN touch /app/tests/__init__.py


ENV PYTHONPATH /app/tests

WORKDIR /app/tests/

# The below command is overidden in docker-compose.yml
CMD echo "should not be seen"
