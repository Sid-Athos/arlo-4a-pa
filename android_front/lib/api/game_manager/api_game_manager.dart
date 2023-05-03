import 'package:dio/dio.dart';
import 'package:miku/model/game_model.dart';
import 'package:miku/model/mapper/game_response_mapper.dart';
import 'package:web_socket_channel/io.dart';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'dart:developer' as developer;

class ApiGameManager {
  static const String baseURLWS = "ws://192.168.137.117:7589";
  static const String baseURL = "http://192.168.137.117:7589";
  static final dio = Dio();

  static WebSocketChannel openWebSocketConnection(String token) {
    final headers = {'Authorization': 'Bearer $token'};
    final channel = IOWebSocketChannel.connect('$baseURLWS/ws', headers: headers);
    return channel;
  }

  static Future<List<Game>> getAllGames() async {
    try {
      final response = await dio.get('$baseURL/game/all');
      final data = response.data as List<dynamic>;
      return data.map((json) => GameResponseMapper.fromJson(json)).toList();
    } catch (e) {
      developer.log(e.toString());
      return [];
    }
  }
}