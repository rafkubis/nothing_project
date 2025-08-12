#pragma once
#include "hello.grpc.pb.h"
#include "hello.pb.h"

extern std::vector<size_t> kernels(const std::string& str);

class HellowerImpl : public hello::Hellower::Service {
public:
    grpc::Status say_hallo(grpc::ServerContext* context, const hello::request* request, hello::response* response) override {
        auto kernels_out = kernels(request->greet());
        std::cout << "Received request: " << request->greet() << std::endl;
        return grpc::Status::OK;
    }
};
