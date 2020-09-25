use std::pin::Pin;
use std::time::Duration;
use tokio::stream::Stream;
use tokio::stream::StreamExt;
use tokio::time;
use tonic::{transport::Server, Code, Request, Response, Status};
mod spq_generated {
    tonic::include_proto!("spq_generated");
}
use sp_queue::error::Error;
use sp_queue::feature_space::FeatureValue;
use sp_queue::SortingPriorityQueue;
use spq_generated::health_check_response::ServingStatus;
use spq_generated::health_service_server::{HealthService, HealthServiceServer};
use spq_generated::sorting_priority_queue_service_server::{
    SortingPriorityQueueService, SortingPriorityQueueServiceServer,
};
use spq_generated::Feature;
use spq_generated::{
    CreateQueueRequest, DequeueRequest, EnqueueRequest, EnqueueResponse, GetEpochRequest,
    GetEpochResponse, GetSizeRequest, GetSizeResponse, HealthCheckRequest, HealthCheckResponse,
    ItemResponse, PeekRequest, QueueResponse,
};
use std::collections::HashMap;
use std::sync::RwLock;

pub struct DefaultSortingPriorityQueueService {
    queues: RwLock<HashMap<String, RwLock<SortingPriorityQueue>>>,
}

impl DefaultSortingPriorityQueueService {
    fn get_queue_run_read_op<Res>(
        &self,
        queue_name: &str,
        f: fn(queue: &SortingPriorityQueue) -> Result<Response<Res>, Status>,
    ) -> Result<Response<Res>, Status> {
        let queues = self
            .queues
            .try_read()
            .map_err(|_| Status::new(Code::Unavailable, "Update in progress please retry"))?;

        match queues.get(queue_name) {
            Some(queue_lock) => {
                let queue = queue_lock.try_read().map_err(|_| {
                    Status::new(Code::Unavailable, "Update in progress please retry")
                })?;

                (f)(&queue)
            }
            None => Err(Status::new(
                Code::NotFound,
                format!("Queue {:?} could not be found", queue_name),
            )),
        }
    }
    fn get_queue_run_op<Req, Res>(
        &self,
        queue_name: &str,
        request: &Req,
        f: fn(request: &Req, queue: &mut SortingPriorityQueue) -> Result<Response<Res>, Status>,
    ) -> Result<Response<Res>, Status> {
        let queues = self
            .queues
            .try_read()
            .map_err(|_| Status::new(Code::Unavailable, "Update in progress please retry"))?;

        match queues.get(queue_name) {
            Some(queue_lock) => {
                let mut queue = queue_lock.try_write().map_err(|_| {
                    Status::new(Code::Unavailable, "Update in progress please retry")
                })?;

                (f)(request, &mut queue)
            }
            None => Err(Status::new(
                Code::NotFound,
                format!("Queue {:?} could not be found", queue_name),
            )),
        }
    }
}

fn to_feature_value(feature: Feature) -> FeatureValue {
    FeatureValue::new(feature.name, feature.value as usize)
}

fn to_status<V>(result: Result<V, Error>) -> Result<V, Status> {
    result.map_err(|err| match err {
        Error::Standard { message } => Status::new(Code::Internal, message),
        Error::Empty { message } => Status::new(Code::Internal, message),
    })
}

#[tonic::async_trait]
impl SortingPriorityQueueService for DefaultSortingPriorityQueueService {
    async fn create_queue(
        &self,
        _request: Request<CreateQueueRequest>,
    ) -> Result<Response<QueueResponse>, Status> {
        let create_queue_request = _request.get_ref();
        let mut queues = self
            .queues
            .try_write()
            .map_err(|_| Status::new(Code::Unavailable, "Update in progress please retry"))?;

        let queue = to_status(SortingPriorityQueue::new_durable(
            create_queue_request.features.clone(),
            "/var/lib/spqr/".to_string() + &create_queue_request.name,
        ))?;

        queues
            .entry(create_queue_request.name.clone())
            .or_insert_with(|| RwLock::new(queue));

        Ok(Response::new(QueueResponse {
            name: create_queue_request.name.clone(),
        }))
    }

    async fn enqueue(
        &self,
        _request: Request<EnqueueRequest>,
    ) -> Result<Response<EnqueueResponse>, Status> {
        fn op(
            request: &EnqueueRequest,
            queue: &mut SortingPriorityQueue,
        ) -> Result<Response<EnqueueResponse>, Status> {
            queue
                .enqueue(
                    request.item.clone(),
                    request
                        .features
                        .clone()
                        .into_iter()
                        .map(to_feature_value)
                        .collect(),
                )
                .map_err(|err| Status::new(Code::Internal, err))?;
            let size = to_status(queue.size())?;

            Ok(Response::new(EnqueueResponse { size: size as i64 }))
        }

        let enqueue_request = _request.get_ref();
        self.get_queue_run_op::<EnqueueRequest, EnqueueResponse>(
            &enqueue_request.queue_name,
            &enqueue_request,
            op,
        )
    }

    async fn dequeue(
        &self,
        _request: Request<DequeueRequest>,
    ) -> Result<Response<ItemResponse>, Status> {
        fn op(
            _request: &DequeueRequest,
            queue: &mut SortingPriorityQueue,
        ) -> Result<Response<ItemResponse>, Status> {
            let (maybe_next, _) = to_status(queue.dequeue())?;
            let size = to_status(queue.size())?;

            Ok(Response::new(ItemResponse {
                has_item: maybe_next.is_some(),
                item: maybe_next.unwrap_or_default(),
                size: size as i64,
            }))
        }

        let request = _request.get_ref();
        self.get_queue_run_op::<DequeueRequest, ItemResponse>(&request.queue_name, &request, op)
    }

    async fn peek(&self, _request: Request<PeekRequest>) -> Result<Response<ItemResponse>, Status> {
        fn op(queue: &SortingPriorityQueue) -> Result<Response<ItemResponse>, Status> {
            let maybe_next = to_status(queue.peek())?;
            let size = to_status(queue.size())?;

            Ok(Response::new(ItemResponse {
                has_item: maybe_next.is_some(),
                item: maybe_next.unwrap_or_default(),
                size: size as i64,
            }))
        }

        let request = _request.get_ref();
        self.get_queue_run_read_op::<ItemResponse>(&request.queue_name, op)
    }

    async fn get_size(
        &self,
        _request: Request<GetSizeRequest>,
    ) -> Result<Response<GetSizeResponse>, Status> {
        fn op(queue: &SortingPriorityQueue) -> Result<Response<GetSizeResponse>, Status> {
            let size = to_status(queue.size())?;

            Ok(Response::new(GetSizeResponse { size: size as i64 }))
        }

        let request = _request.get_ref();
        self.get_queue_run_read_op::<GetSizeResponse>(&request.queue_name, op)
    }

    async fn get_epoch(
        &self,
        _request: Request<GetEpochRequest>,
    ) -> Result<Response<GetEpochResponse>, Status> {
        fn op(queue: &SortingPriorityQueue) -> Result<Response<GetEpochResponse>, Status> {
            let epoch = to_status(queue.get_epoch())?;

            Ok(Response::new(GetEpochResponse {
                epoch: epoch as i64,
            }))
        }

        let request = _request.get_ref();
        self.get_queue_run_read_op::<GetEpochResponse>(&request.queue_name, op)
    }
}

#[derive(Default)]
pub struct DefaultHealthService {}

#[tonic::async_trait]
impl HealthService for DefaultHealthService {
    async fn check(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        Ok(Response::new(HealthCheckResponse {
            status: ServingStatus::Serving as i32,
        }))
    }

    type WatchStream =
        Pin<Box<dyn Stream<Item = Result<HealthCheckResponse, Status>> + Send + Sync + 'static>>;
    async fn watch(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<Self::WatchStream>, Status> {
        let health_stream: Self::WatchStream = Box::pin({
            time::interval(Duration::from_secs(1)).map(move |_| {
                Ok(HealthCheckResponse {
                    status: ServingStatus::Serving as i32,
                })
            })
        });

        Ok(Response::new(health_stream))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::0]:9090".parse()?;

    let spq_service = DefaultSortingPriorityQueueService {
        queues: RwLock::new(HashMap::new()),
    };
    let health_service = DefaultHealthService::default();

    println!("Booting");

    Server::builder()
        .add_service(SortingPriorityQueueServiceServer::new(spq_service))
        .add_service(HealthServiceServer::new(health_service))
        .serve(addr)
        .await?;

    println!("Shutting down");

    Ok(())
}
