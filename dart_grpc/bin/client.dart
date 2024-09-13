import 'package:dart_grpc/proto/helloworld.pbgrpc.dart';
import 'package:grpc/grpc.dart';

Future<void> main(List<String> args) async {
  final channel = ClientChannel(
    'localhost',
    port: 50051,
    options: const ChannelOptions(credentials: ChannelCredentials.insecure()),
  );

  final stub = GreeterClient(channel);

  try {
    final response = await stub.sayHello(HelloRequest()..name = 'Leo');
    print('Greeter client received: ${response.message}');
  } catch (e) {
    print('Caught error: $e');
  }
  await channel.shutdown();
}
