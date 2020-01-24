package service

import (
	"context"

	pb "user-service/protobuf"
)

type UserServer struct {
	pb.UnimplementedUserServer
}

func (u *UserServer) VerifyAuthToken(ctx context.Context, in *pb.AuthTokenReq) (*pb.AuthTokenResp, error) {
	return &pb.AuthTokenResp{Ok: true}, nil
}
