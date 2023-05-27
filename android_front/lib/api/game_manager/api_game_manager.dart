import 'package:dio/dio.dart';
import 'package:miku/model/game_model.dart';
import 'package:miku/model/lobby_model.dart';
import 'package:miku/model/mapper/game_response_mapper.dart';
import 'package:miku/model/mapper/lobby_response_mapper.dart';
import 'package:web_socket_channel/io.dart';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'dart:developer' as developer;

class ApiGameManager {
  static const String baseURLWS = "ws://dev.mikusupremacy.fr:7589";
  static const String baseURL = "https://dev.mikusupremacy.fr/gamemanager";
  static final dio = Dio();

  static WebSocketChannel? openWebSocketConnection(String token) {
    try {
      final headers = {'Authorization': 'Bearer $token'};
      return IOWebSocketChannel.connect('$baseURLWS/ws', headers: headers);
    } catch (e) {
      return null;
    }
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

  static Future<List<Lobby>> getPublicLobbyForGame(int gameId) async {
    try {
      final response = await dio.get('$baseURL/lobby/get_public/$gameId');
      final data = response.data as List<dynamic>;
      return data.map((json) => LobbyResponseMapper.fromJson(json)).toList();
    } catch (e) {
      developer.log(e.toString());
      return [];
    }
  }
}