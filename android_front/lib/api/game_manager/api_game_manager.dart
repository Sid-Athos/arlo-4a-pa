import 'package:dio/dio.dart';
import 'package:miku/model/game_model.dart';
import 'package:miku/model/lobby_model.dart';
import 'package:miku/model/mapper/game_response_mapper.dart';
import 'package:miku/model/mapper/lobby_response_mapper.dart';
import 'package:miku/model/mapper/user_response_mapper.dart';
import 'package:miku/model/user_model.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:web_socket_channel/io.dart';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'dart:developer' as developer;

class ApiGameManager {
  static const String baseURLWS = "ws://dev.mikusupremacy.fr:7589";
  static const String baseURL = "https://dev.mikusupremacy.fr/gamemanager";
  static final dio = Dio();

  static WebSocketChannel? openWebSocketConnection(String token) {
    try {
      return IOWebSocketChannel.connect('$baseURLWS/ws?token=$token');
    } catch (e) {
      return null;
    }
  }

  static Future<List<User>> getConnectedFriends() async {
    try {
      SharedPreferences prefs = await SharedPreferences.getInstance();
      dio.options.headers["Authorization"] = "Bearer ${prefs.getString('login_token')}";
      final response = await dio.get('$baseURL/friends/connected_friends');
      final data = response.data as List<dynamic>;
      return data.map((json) => UserResponseMapper.fromJson(json)).toList();
    } catch (e) {
      developer.log(e.toString());
      return [];
    }
  }

  static Future<List<Game>> getAllGames() async {
    try {
      final response = await dio.get('$baseURL/games/all');
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