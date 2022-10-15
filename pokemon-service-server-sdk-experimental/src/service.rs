use crate::operation_shape::{
    CapturePokemon, CheckHealth, DoNothing, GetPokemonSpecies, GetServerStatistics, GetStorage,
};
use aws_smithy_http_server::operation::Operation;
use aws_smithy_http_server::operation::{
    FailOnMissingOperation, IntoService, MissingFailure, OperationShape, Upgradable,
};
use aws_smithy_http_server::proto::rest::router::RestRouter;
use aws_smithy_http_server::proto::rest_json_1::AwsRestJson1;
use aws_smithy_http_server::routers::RoutingService;
use aws_smithy_http_server::routing::request_spec::{
    PathAndQuerySpec, PathSegment, PathSpec, QuerySpec, RequestSpec, UriSpec,
};
use aws_smithy_http_server::routing::Route;
use std::fmt::{Display, Formatter};

pub struct PokemonServiceBuilder<Body, Plugin> {
    check_health: Option<Route<Body>>,
    do_nothing: Option<Route<Body>>,
    get_pokemon_species: Option<Route<Body>>,
    get_server_statistics: Option<Route<Body>>,
    capture_pokemon: Option<Route<Body>>,
    get_storage: Option<Route<Body>>,
    plugin: Plugin,
}

impl<Body, Plugin> PokemonServiceBuilder<Body, Plugin> {
    pub fn check_health<Handler, Extensions>(mut self, handler: Handler) -> Self
    where
        Handler: RouteHandler<Extensions, Body, CheckHealth, Plugin>,
    {
        self.check_health = Some(handler.into_route(&self.plugin));
        self
    }

    pub fn do_nothing<Handler, Extensions>(mut self, handler: Handler) -> Self
    where
        Handler: RouteHandler<Extensions, Body, DoNothing, Plugin>,
    {
        self.do_nothing = Some(handler.into_route(&self.plugin));
        self
    }

    pub fn get_pokemon_species<Handler, Extensions>(mut self, handler: Handler) -> Self
    where
        Handler: RouteHandler<Extensions, Body, GetPokemonSpecies, Plugin>,
    {
        self.get_pokemon_species = Some(handler.into_route(&self.plugin));
        self
    }

    pub fn get_server_statistics<Handler, Extensions>(mut self, handler: Handler) -> Self
    where
        Handler: RouteHandler<Extensions, Body, GetServerStatistics, Plugin>,
    {
        self.get_server_statistics = Some(handler.into_route(&self.plugin));
        self
    }

    pub fn capture_pokemon<Handler, Extensions>(mut self, handler: Handler) -> Self
    where
        Handler: RouteHandler<Extensions, Body, CapturePokemon, Plugin>,
    {
        self.capture_pokemon = Some(handler.into_route(&self.plugin));
        self
    }

    pub fn get_storage<Handler, Extensions>(mut self, handler: Handler) -> Self
    where
        Handler: RouteHandler<Extensions, Body, GetStorage, Plugin>,
    {
        self.get_storage = Some(handler.into_route(&self.plugin));
        self
    }

    pub fn build_unchecked(
        self,
    ) -> Result<PokemonService<Route<Body>>, Box<dyn std::error::Error>> {
        let router = RestRouter::from_iter([
            (
                RequestSpec::new(
                    http::Method::GET,
                    UriSpec::new(PathAndQuerySpec::new(
                        PathSpec::from_vector_unchecked(vec![PathSegment::Literal(String::from(
                            "ping",
                        ))]),
                        QuerySpec::from_vector_unchecked(vec![]),
                    )),
                ),
                self.check_health.unwrap_or_else(|| {
                    Route::new::<MissingFailure<AwsRestJson1>>(
                        <FailOnMissingOperation as Upgradable<
                            AwsRestJson1,
                            CheckHealth,
                            (),
                            Body,
                            Plugin,
                        >>::upgrade(FailOnMissingOperation, &self.plugin),
                    )
                }),
            ),
            (
                RequestSpec::new(
                    http::Method::GET,
                    UriSpec::new(PathAndQuerySpec::new(
                        PathSpec::from_vector_unchecked(vec![PathSegment::Literal(String::from(
                            "do-nothing",
                        ))]),
                        QuerySpec::from_vector_unchecked(vec![]),
                    )),
                ),
                self.do_nothing.unwrap_or_else(|| {
                    Route::new::<MissingFailure<AwsRestJson1>>(
                        <FailOnMissingOperation as Upgradable<
                            AwsRestJson1,
                            DoNothing,
                            (),
                            Body,
                            Plugin,
                        >>::upgrade(FailOnMissingOperation, &self.plugin),
                    )
                }),
            ),
            (
                RequestSpec::new(
                    http::Method::GET,
                    UriSpec::new(PathAndQuerySpec::new(
                        PathSpec::from_vector_unchecked(vec![
                            PathSegment::Literal(String::from("pokemon-species")),
                            PathSegment::Label,
                        ]),
                        QuerySpec::from_vector_unchecked(vec![]),
                    )),
                ),
                self.get_pokemon_species.unwrap_or_else(|| {
                    Route::new::<MissingFailure<AwsRestJson1>>(
                        <FailOnMissingOperation as Upgradable<
                            AwsRestJson1,
                            GetPokemonSpecies,
                            (),
                            Body,
                            Plugin,
                        >>::upgrade(FailOnMissingOperation, &self.plugin),
                    )
                }),
            ),
            (
                RequestSpec::new(
                    http::Method::GET,
                    UriSpec::new(PathAndQuerySpec::new(
                        PathSpec::from_vector_unchecked(vec![PathSegment::Literal(String::from(
                            "stats",
                        ))]),
                        QuerySpec::from_vector_unchecked(vec![]),
                    )),
                ),
                self.get_server_statistics.unwrap_or_else(|| {
                    Route::new::<MissingFailure<AwsRestJson1>>(
                        <FailOnMissingOperation as Upgradable<
                            AwsRestJson1,
                            GetServerStatistics,
                            (),
                            Body,
                            Plugin,
                        >>::upgrade(FailOnMissingOperation, &self.plugin),
                    )
                }),
            ),
            (
                RequestSpec::new(
                    http::Method::POST,
                    UriSpec::new(PathAndQuerySpec::new(
                        PathSpec::from_vector_unchecked(vec![
                            PathSegment::Literal(String::from("capture-pokemon-event")),
                            PathSegment::Label,
                        ]),
                        QuerySpec::from_vector_unchecked(vec![]),
                    )),
                ),
                self.capture_pokemon.unwrap_or_else(|| {
                    Route::new::<MissingFailure<AwsRestJson1>>(
                        <FailOnMissingOperation as Upgradable<
                            AwsRestJson1,
                            CapturePokemon,
                            (),
                            Body,
                            Plugin,
                        >>::upgrade(FailOnMissingOperation, &self.plugin),
                    )
                }),
            ),
            (
                RequestSpec::new(
                    http::Method::GET,
                    UriSpec::new(PathAndQuerySpec::new(
                        PathSpec::from_vector_unchecked(vec![
                            PathSegment::Literal(String::from("pokedex")),
                            PathSegment::Label,
                        ]),
                        QuerySpec::from_vector_unchecked(vec![]),
                    )),
                ),
                self.get_storage.unwrap_or_else(|| {
                    Route::new::<MissingFailure<AwsRestJson1>>(
                        <FailOnMissingOperation as Upgradable<
                            AwsRestJson1,
                            GetStorage,
                            (),
                            Body,
                            Plugin,
                        >>::upgrade(FailOnMissingOperation, &self.plugin),
                    )
                }),
            ),
        ]);
        Ok(PokemonService {
            router: RoutingService::new(router),
        })
    }

    pub fn build(self) -> Result<PokemonService<Route<Body>>, MissingOperationsError> {
        let mut routes = Vec::with_capacity(6);
        let mut missing_operation_names = Vec::new();
        if let Some(r) = self.check_health {
            routes.push((
                RequestSpec::new(
                    http::Method::GET,
                    UriSpec::new(PathAndQuerySpec::new(
                        PathSpec::from_vector_unchecked(vec![PathSegment::Literal(String::from(
                            "ping",
                        ))]),
                        QuerySpec::from_vector_unchecked(vec![]),
                    )),
                ),
                r,
            ))
        } else {
            missing_operation_names.push(CheckHealth::NAME)
        }
        if let Some(r) = self.do_nothing {
            routes.push((
                RequestSpec::new(
                    http::Method::GET,
                    UriSpec::new(PathAndQuerySpec::new(
                        PathSpec::from_vector_unchecked(vec![PathSegment::Literal(String::from(
                            "do-nothing",
                        ))]),
                        QuerySpec::from_vector_unchecked(vec![]),
                    )),
                ),
                r,
            ))
        } else {
            missing_operation_names.push(DoNothing::NAME)
        }
        if let Some(r) = self.get_pokemon_species {
            routes.push((
                RequestSpec::new(
                    http::Method::GET,
                    UriSpec::new(PathAndQuerySpec::new(
                        PathSpec::from_vector_unchecked(vec![
                            PathSegment::Literal(String::from("pokemon-species")),
                            PathSegment::Label,
                        ]),
                        QuerySpec::from_vector_unchecked(vec![]),
                    )),
                ),
                r,
            ))
        } else {
            missing_operation_names.push(GetPokemonSpecies::NAME)
        }
        if let Some(r) = self.get_server_statistics {
            routes.push((
                RequestSpec::new(
                    http::Method::GET,
                    UriSpec::new(PathAndQuerySpec::new(
                        PathSpec::from_vector_unchecked(vec![PathSegment::Literal(String::from(
                            "stats",
                        ))]),
                        QuerySpec::from_vector_unchecked(vec![]),
                    )),
                ),
                r,
            ))
        } else {
            missing_operation_names.push(GetServerStatistics::NAME)
        }
        if let Some(r) = self.capture_pokemon {
            routes.push((
                RequestSpec::new(
                    http::Method::POST,
                    UriSpec::new(PathAndQuerySpec::new(
                        PathSpec::from_vector_unchecked(vec![
                            PathSegment::Literal(String::from("capture-pokemon-event")),
                            PathSegment::Label,
                        ]),
                        QuerySpec::from_vector_unchecked(vec![]),
                    )),
                ),
                r,
            ))
        } else {
            missing_operation_names.push(CapturePokemon::NAME)
        }
        if let Some(r) = self.get_storage {
            routes.push((
                RequestSpec::new(
                    http::Method::GET,
                    UriSpec::new(PathAndQuerySpec::new(
                        PathSpec::from_vector_unchecked(vec![
                            PathSegment::Literal(String::from("pokedex")),
                            PathSegment::Label,
                        ]),
                        QuerySpec::from_vector_unchecked(vec![]),
                    )),
                ),
                r,
            ))
        } else {
            missing_operation_names.push(GetStorage::NAME)
        }
        if !missing_operation_names.is_empty() {
            Err(MissingOperationsError {
                operation_names: missing_operation_names,
            })
        } else {
            Ok(PokemonService {
                router: RoutingService::new(RestRouter::from_iter(routes)),
            })
        }
    }
}

/// A trait alias to have manageable trait bounds in the builder
pub trait RouteHandler<Extensions, Body, Operation, Plugin> {
    fn into_route(self, plugin: &Plugin) -> Route<Body>;
}

impl<Handler, Extensions, Body, Plugin, OperationShape>
    RouteHandler<Extensions, Body, OperationShape, Plugin> for Handler
where
    Handler: aws_smithy_http_server::operation::Handler<OperationShape, Extensions>,
    Operation<IntoService<OperationShape, Handler>>:
        Upgradable<AwsRestJson1, OperationShape, Extensions, Body, Plugin>,
    // This highlights that we should probably have more restrictive trait bounds
    // on `Upgradable`'s associated types
    <Operation<IntoService<OperationShape, Handler>> as Upgradable<
        AwsRestJson1,
        OperationShape,
        Extensions,
        Body,
        Plugin,
    >>::Service: Clone + Send + 'static,
    <<Operation<IntoService<OperationShape, Handler>> as Upgradable<
        AwsRestJson1,
        OperationShape,
        Extensions,
        Body,
        Plugin,
    >>::Service as tower::Service<http::Request<Body>>>::Future: Send + 'static,
    <Operation<IntoService<OperationShape, Handler>> as Upgradable<
        AwsRestJson1,
        OperationShape,
        Extensions,
        Body,
        Plugin,
    >>::Service: tower::Service<http::Request<Body>, Error = std::convert::Infallible>,
    OperationShape: aws_smithy_http_server::operation::OperationShape,
{
    fn into_route(self, plugin: &Plugin) -> Route<Body> {
        Route::new(Operation::from_handler(self).upgrade(&plugin))
    }
}

/// The Pokémon Service allows you to retrieve information about Pokémon species.
#[derive(Clone)]
pub struct PokemonService<S> {
    router: RoutingService<RestRouter<S>, AwsRestJson1>,
}

impl PokemonService<()> {
    /// Constructs a builder for [`PokemonService`].
    pub fn builder<Body, Plugin>(plugin: Plugin) -> PokemonServiceBuilder<Body, Plugin> {
        PokemonServiceBuilder {
            check_health: None,
            do_nothing: None,
            get_pokemon_species: None,
            get_server_statistics: None,
            capture_pokemon: None,
            get_storage: None,
            plugin,
        }
    }
}

impl<S> PokemonService<S> {
    /// Converts [`PokemonService`] into a [`MakeService`](tower::make::MakeService).
    pub fn into_make_service(self) -> aws_smithy_http_server::routing::IntoMakeService<Self> {
        aws_smithy_http_server::routing::IntoMakeService::new(self)
    }

    /// Applies a layer uniformly to all routes.
    pub fn layer<L>(self, layer: &L) -> PokemonService<L::Service>
    where
        L: tower::Layer<S>,
    {
        PokemonService {
            router: self.router.map(|s| s.layer(layer)),
        }
    }
}

impl<B, RespB, S> tower::Service<http::Request<B>> for PokemonService<S>
where
    S: tower::Service<http::Request<B>, Response = http::Response<RespB>> + Clone,
    RespB: http_body::Body<Data = bytes::Bytes> + Send + 'static,
    RespB::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    type Response = http::Response<aws_smithy_http_server::body::BoxBody>;
    type Error = S::Error;
    type Future = aws_smithy_http_server::routers::RoutingFuture<S, B>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.router.poll_ready(cx)
    }

    fn call(&mut self, request: http::Request<B>) -> Self::Future {
        self.router.call(request)
    }
}

#[derive(Debug)]
pub struct MissingOperationsError {
    operation_names: Vec<&'static str>,
}

impl Display for MissingOperationsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "You have not registered a handler for all operations specified by the service.\nWe are missing handlers for fhe following operations: {}", self.operation_names.join(","))
    }
}
