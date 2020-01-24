package main

import (
	"google.golang.org/grpc"
	"log"
	"net"

	pb "user-service/protobuf"
	"user-service/service"
)

func main() {
	lis, err := net.Listen("tcp", ":8800")
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}
	s := grpc.NewServer()
	pb.RegisterUserServer(s, &service.UserServer{})
	if err := s.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
