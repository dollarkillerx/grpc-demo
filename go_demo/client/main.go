package main

import (
	"context"
	"log"
	"os"
	"time"

	pb "go_demo_grpc/proto"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func main() {
	// 创建 gRPC 连接
	conn, err := grpc.NewClient("localhost:7748", grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("Failed to connect: %v", err)
	}
	defer conn.Close()

	// 创建 gRPC 客户端
	client := pb.NewGreeterClient(conn)

	// 准备请求数据
	name := "world"
	if len(os.Args) > 1 {
		name = os.Args[1]
	}

	// 设置带有超时的上下文
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()

	// 调用 SayHello 方法
	response, err := client.SayHello(ctx, &pb.HelloRequest{Name: name})
	if err != nil {
		log.Fatalf("Could not greet: %v", err)
	}

	// 打印响应
	log.Printf("Greeting: %s", response.GetMessage())
}
