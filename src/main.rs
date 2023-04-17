#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
//log4rs for logging
#[macro_use]
extern crate log;
extern crate log4rs;

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;
use actix_rt::signal;
use skywalking::{
    logging::{
        logger::Logger,
        record::{LogRecord, RecordType}
    },
    metrics::{meter::Counter, metricer::Metricer},
    reporter::grpc::GrpcReporter,
    trace::tracer::Tracer,
};
use std::error::Error;

mod db;
mod employees;
mod error_handler;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    log4rs::init_file("log.yaml",Default::default()).unwrap();

    let oap_endpoint = std::env::var("SW_AGENT_COLLECTOR_BACKEND_SERVICES").unwrap();

    /////////////////////////////////////////////////
    // Connect to skywalking oap server.
    let reporter = GrpcReporter::connect(oap_endpoint).await.unwrap();
    // Optional authentication, based on backend setting.
    //let reporter = reporter.with_authentication("<TOKEN>");

    // Spawn the reporting in background, with listening the graceful shutdown signal.
    let handle = reporter
        .reporting()
        .await
        .with_graceful_shutdown(async move {
            signal::ctrl_c().await.expect("failed to listen for event");
        })
        .spawn();

    let tracer = Tracer::new("fsdemo_emplyservice", "instance", reporter.clone());
    let logger = Logger::new("fsdemo_emplyservice", "instance", reporter.clone());
    let metricer = Metricer::new("fsdemo_emplyservice", "instance", reporter);

    

    /////////////////////////////////////////////////
    //just for skywalking demo
    let mut ctx = tracer.create_trace_context();

    {
        // Generate an Entry Span when a request is received.
        // An Entry Span is generated only once per context.
        // Assign a variable name to guard the span not to be dropped immediately.
        let _span = ctx.create_entry_span("op1");

        // Something...

        {
            // Generates an Exit Span when executing an RPC.
            let span2 = ctx.create_exit_span("op2", "remote_peer");

            // Something...

            // Do logging.
            logger.log(
                LogRecord::new()
                    .add_tag("level", "INFO")
                    .with_tracing_context(&ctx)
                    .with_span(&span2)
                    .record_type(RecordType::Text)
                    .content("Something...")
            );

            

            // Auto close span2 when dropped.
        }

        // Auto close span when dropped.
    }

    // Auto report ctx when dropped.

    // skywalking demo end
    /////////////////////////////////////////////////
    
    //let swtool = skywalkingTool(tracer, logger);

    let swtool = employees::skywalkingTool{tracer, logger};
    
    let swtoolArc = std::sync::Arc::new(swtool);

    info!("emplyservice is starting...");

    dotenv().ok();
    db::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new( move || {App::new()
        .app_data(web::Data::new(swtoolArc.clone()))
        .configure(employees::init_routes)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env");
            let port = env::var("PORT").expect("Please set port in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    //handle_metric(metricer).await;

    //handle.await.unwrap();

    server.run().await
}

async fn handle_metric(mut metricer: Metricer) {
    let counter = metricer.register(
        Counter::new("instance_trace_count")
            .add_label("region", "cn-east")
            .add_label("az", "az-1"),
    );

    counter.increment(1.);
    metricer.boot().await.unwrap();

}