import 'package:dio/dio.dart';
import 'package:web_socket_channel/io.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

class ApiGameManager {
  static const String baseURL = "ws://dev.mikusupremacy.fr:7589";
  static final dio = Dio();

  static WebSocketChannel openWebSocketConnection(String token) {
    final headers = {'Authorization': 'Bearer $token'};
    final channel = IOWebSocketChannel.connect('$baseURL/ws', headers: headers);
    return channel;
  }
}