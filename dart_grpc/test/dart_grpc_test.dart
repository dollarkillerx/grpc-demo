import 'package:dart_grpc/dart_grpc.dart';
import 'package:test/test.dart';

void main() {
  test('calculate', () {
    expect(calculate(), 42);
  });
}
