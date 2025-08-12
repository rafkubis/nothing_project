
#include <grpcpp/grpcpp.h>
#include "app.hpp"
#include "hellower_impl.hpp"


void runServer() {
    kernels("Hello from server");

    std::unique_ptr<grpc::Server> server;
    grpc::ServerBuilder builder;
    HellowerImpl service;
    builder.AddListeningPort("localhost:50051", grpc::InsecureServerCredentials());
    builder.RegisterService(&service);
    server = builder.BuildAndStart();
    std::cout << "Server listening on localhost:50051" << std::endl;    

    server->Wait();
}
