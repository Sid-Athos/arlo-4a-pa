import 'package:dio/dio.dart';
import 'package:miku/model/game/game_model.dart';
import 'package:miku/model/lobby/lobby_model.dart';
import 'package:miku/mapper/game/game_response_mapper.dart';
import 'package:miku/mapper/lobby/lobby_response_mapper.dart';
import 'package:miku/mapper/user/user_response_mapper.dart';
import 'package:miku/model/user/user_model.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:web_socket_channel/io.dart';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'dart:developer' as developer;

import '../../mapper/hisotry/game_history_response_mapper.dart';
import '../../mapper/hisotry/game_move_history_mapper.dart';
import '../../model/history/game_history_model.dart';
import '../../model/history/game_move_history_model.dart';

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

  static Future<List<User>> joinRtcSession() async {
    try {
      SharedPreferences prefs = await SharedPreferences.getInstance();
      dio.options.headers["Authorization"] = "Bearer ${prefs.getString('login_token')}";
      final response = await dio.post('$baseURL/rtc/join_rtc', data: "");
      final data = response.data as List<dynamic>;
      return data.map((json) => UserResponseMapper.fromJson(json)).toList();
    } catch (e) {
      developer.log(e.toString());
      return [];
    }
  }

  static Future<void> leaveRtcSession() async {
    try {
      SharedPreferences prefs = await SharedPreferences.getInstance();
      dio.options.headers["Authorization"] = "Bearer ${prefs.getString('login_token')}";
      await dio.post('$baseURL/rtc/leave_rtc', data: "");
    } catch (e) {
      developer.log(e.toString());
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

  static Future<List<GameHistory>> getGameHistory(int gameId) async {
    try {
      final response = await dio.get('$baseURL/history/games/$gameId');
      final data = response.data as List<dynamic>;
      return data.map((json) => GameHistoryResponseMapper.fromJson(json)).toList();
    } catch (e) {
      developer.log(e.toString());
      return [];
    }
  }

  static Future<List<GameMoveHistory>> getGameMoveHistory(int gameHistoryId) async {
    try {
      final response = await dio.get('$baseURL/history/moves/$gameHistoryId');
      final data = response.data as List<dynamic>;
      return data.map((json) => GameMoveHistoryResponseMapper.fromJson(json)).toList();
    } catch (e) {
      developer.log(e.toString());
      return [];
    }
  }
}