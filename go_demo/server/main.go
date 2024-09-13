package main

import (
	"context"
	"log"
	"net"

	pb "go_demo_grpc/proto"
	"google.golang.org/grpc"
)

// 定义服务结构体
type server struct {
	pb.UnimplementedGreeterServer
}

// 实现 SayHello 方法
func (s *server) SayHello(ctx context.Context, in *pb.HelloRequest) (*pb.HelloReply, error) {
	log.Printf("Received: %v", in.GetName())
	return &pb.HelloReply{Message: "Hello " + in.GetName()}, nil
}

func main() {
	// 创建监听器
	lis, err := net.Listen("tcp", ":7748")
	if err != nil {
		log.Fatalf("Failed to listen: %v", err)
	}
	// 创建 gRPC 服务器
	s := grpc.NewServer()
	pb.RegisterGreeterServer(s, &server{})
	log.Println("Server is running on port 7748...")
	if err := s.Serve(lis); err != nil {
		log.Fatalf("Failed to serve: %v", err)
	}
}
