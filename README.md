# Foo

## Foo template for the following:

* Protocol buffer
* Tonic integration for gRPC
* gRPC UI for dev testing - https://github.com/fullstorydev/grpcui
* Autometrics - https://autometrics.dev/
* Prometheus - https://prometheus.io/

## Autometrics and Prometheus integration

The simplest way of monitoring at the function level is to annotate the function with the `#[autometrics]` macro

Below is a simple example:

```#[tonic::async_trait]
impl Calculator for CalculatorService {

    #[autometrics] // This is the only annotation needed
    async fn add(&self, request: Request<CalculationRequest>) -> Result<Response<CalculationResponse>, Status> {
        println!("Got a request: {:?}", request);

        let input = request.get_ref();

        let response = CalculationResponse {
            result: input.a + input.b,
        };
        Ok(Response::new(response))
    }
}
```

Once the gRPC server and the http server are started, the Autometrics server and embedded Prometheus server can be run using the following command

```
> am start localhost:3000
```

In the above example, 3000 port is where the http server is running and 50051 is where the gRPC server is running.

You should see an output similar as below:
```
Checking if provided metrics endpoints work...
Failed to make request to http://localhost:3000/metrics (job am_0)
Now sampling the following endpoints for metrics: http://localhost:3000/metrics
Using Prometheus version: 2.47.2
Starting prometheus

  am v0.6.0   press ctrl + c to shutdown

  Explorer         http://127.0.0.1:6789
  Prometheus       http://127.0.0.1:9090/prometheus
```

If you navigate to the `http://127.0.0.1:6789` URL, you will see the metrics as below screenshot for Autometrics:

![image](https://github.com/user-attachments/assets/dfc13975-cef1-4b31-85a1-a21808e8efd0)


If you navigate to the `http://127.0.0.1:9090/prometheus` URL, you will see the metrics in the Prometheus dashboard:

![image](https://github.com/user-attachments/assets/52796df2-34ae-4cd1-bf91-97d153c6dc4b)

