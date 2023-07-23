import 'package:dio/dio.dart';
import 'package:miku/api/user/request/change_password_request.dart';
import 'package:miku/api/user/request/create_user_request.dart';
import 'package:miku/api/user/request/login_request.dart';
import 'package:miku/api/user/request/send_friend_request_request.dart';
import 'package:miku/api/user/request/update_user_request.dart';
import 'package:miku/model/friend/friend_list_model.dart';
import 'package:miku/mapper/friend/friend_list_response_mapper.dart';
import 'package:miku/mapper/ranking/ranking_response_mapper.dart';
import 'package:miku/mapper/user/user_response_mapper.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'dart:developer' as developer;

import '../../mapper/user/session_response_mapper.dart';
import '../../model/ranking/ranking_model.dart';
import '../../model/user/session_model.dart';
import '../../model/user/user_model.dart';

class ApiUser {
  static const String baseURL = "https://dev.mikusupremacy.fr/api";
  static final dio = Dio();

  static Future<User?> getById(int id) async {
    try {
      dio.options.headers["api-key"] = "coding_games";
      final response = await dio.get('$baseURL/user/$id');
      return UserResponseMapper.fromJson(response.data);
    } catch (e) {
      return null;
    }
  }

  static Future<User?> createUser(CreateUserRequest createUserRequest) async {
    try {
      dio.options.headers["api-key"] = "coding_games";
      final response = await dio.post('$baseURL/user', data: createUserRequest.toJson());
      return UserResponseMapper.fromJson(response.data);
    } catch (e) {
      developer.log(e.toString());
      return null;
    }
  }

  static Future<Session?> login(LoginRequest loginRequest) async {
    try {
      dio.options.headers["api-key"] = "coding_games";
      final response = await dio.post('$baseURL/user/login', data: loginRequest.toJson());
      return SessionResponseMapper.fromJson(response.data);
    } catch (e) {
      developer.log(e.toString());
      return null;
    }
  }

  static Future<Session?> logout() async {
    try {
      SharedPreferences prefs = await SharedPreferences.getInstance();
      dio.options.headers["api-key"] = "coding_games";
      dio.options.headers["Authorization"] = "Bearer ${prefs.getString('login_token')}";
      final response = await dio.post('$baseURL/user/logout');
      return SessionResponseMapper.fromJson(response.data);
    } catch (e) {
      developer.log(e.toString());
      return null;
    }
  }

  static Future<User?> delete(String token) async {
    try {
      dio.options.headers["api-key"] = "coding_games";
      dio.options.headers["Authorization"] = "Bearer $token";
      final response = await dio.delete('$baseURL/user/');
      return UserResponseMapper.fromJson(response.data);
    } catch (e) {
      developer.log(e.toString());
      return null;
    }
  }

  static Future<User?> me(String token) async {
    try {
      dio.options.headers["api-key"] = "coding_games";
      dio.options.headers["Authorization"] = "Bearer $token";
      final response = await dio.get('$baseURL/user/me');
      return UserResponseMapper.fromJson(response.data);
    } catch (e) {
      developer.log(e.toString());
      return null;
    }
  }

  static Future<User?> changePassword(ChangePasswordRequest changePasswordRequest, String token) async {
    try {
      dio.options.headers["api-key"] = "coding_games";
      dio.options.headers["Authorization"] = "Bearer $token";
      final response = await dio.put('$baseURL/user/change_password', data: changePasswordRequest.toJson());
      return UserResponseMapper.fromJson(response.data);
    } catch (e) {
      developer.log(e.toString());
      return null;
    }
  }

  static Future<User?> updateUser(UpdateUserRequest updateUserRequest, String token) async {
    try {
      dio.options.headers["api-key"] = "coding_games";
      dio.options.headers["Authorization"] = "Bearer $token";
      final response = await dio.put('$baseURL/user', data: updateUserRequest.toJson());
      return UserResponseMapper.fromJson(response.data);
    } catch (e) {
      developer.log(e.toString());
      return null;
    }
  }

  static Future<List<User>> search(String searched) async {
    try {
      dio.options.headers["api-key"] = "coding_games";
      final response = await dio.get('$baseURL/user/search/$searched');
      final data = response.data as List<dynamic>;
      return data.map((json) => UserResponseMapper.fromJson(json)).toList();
    } catch (e) {
      developer.log(e.toString());
      return [];
    }
  }

  static Future<List<FriendList>> getFriendList() async {
    try {
      SharedPreferences prefs = await SharedPreferences.getInstance();
      dio.options.headers["api-key"] = "coding_games";
      dio.options.headers["Authorization"] = "Bearer ${prefs.getString('login_token')}";
      final response = await dio.get('$baseURL/friend_list');
      final data = response.data as List<dynamic>;
      return data.map((json) => FriendListResponseMapper.fromJson(json)).toList();
    } catch (e) {
      developer.log(e.toString());
      return [];
    }
  }

  static Future<FriendList?> deleteFriend(int user_id) async {
    try {
      SharedPreferences prefs = await SharedPreferences.getInstance();
      dio.options.headers["api-key"] = "coding_games";
      dio.options.headers["Authorization"] = "Bearer ${prefs.getString('login_token')}";
      final response = await dio.delete('$baseURL/friend_list/$user_id');
      return FriendListResponseMapper.fromJson(response.data);
    } catch (e) {
      developer.log(e.toString());
      return null;
    }
  }

  static Future<FriendList?> sendFriendRequest(SendFriendRequestRequest sendFriendRequestRequest) async {
    try {
      dio.options.headers["api-key"] = "coding_games";
      final response = await dio.post('$baseURL/friend_list', data: sendFriendRequestRequest.toJson());
      return FriendListResponseMapper.fromJson(response.data);
    } catch (e) {
      developer.log(e.toString());
      return null;
    }
  }

  static Future<List<FriendList>> getAllUnacceptedRequestWithApplicant(int userId) async {
    try {
      SharedPreferences prefs = await SharedPreferences.getInstance();
      dio.options.headers["api-key"] = "coding_games";
      dio.options.headers["Authorization"] = "Bearer ${prefs.getString('login_token')}";
      final response = await dio.get('$baseURL/friend_list/requests');
      final data = response.data as List<dynamic>;
      List<FriendList> friendsRequest =  data.map((json) => FriendListResponseMapper.fromJson(json)).toList();
      List<FriendList> friendsRequestFiltered = [];
      for (FriendList friend in friendsRequest) {
        if (friend.applicantId == userId) {
          friendsRequestFiltered.add(friend);
        }
      }
      return friendsRequestFiltered;
    } catch (e) {
      developer.log(e.toString());
      return [];
    }
  }

  static Future<List<FriendList>> getAllUnacceptedRequestWithRecipient(int userId) async {
    try {
      SharedPreferences prefs = await SharedPreferences.getInstance();
      dio.options.headers["api-key"] = "coding_games";
      dio.options.headers["Authorization"] = "Bearer ${prefs.getString('login_token')}";
      final response = await dio.get('$baseURL/friend_list/requests');
      final data = response.data as List<dynamic>;
      List<FriendList> friendsRequest =  data.map((json) => FriendListResponseMapper.fromJson(json)).toList();
      List<FriendList> friendsRequestFiltered = [];
      for (FriendList friend in friendsRequest) {
        if (friend.recipientId == userId) {
          friendsRequestFiltered.add(friend);
        }
      }
      return friendsRequestFiltered;
    } catch (e) {
      developer.log(e.toString());
      return [];
    }
  }

  static Future<User?> acceptFriendRequest(int requestId) async {
    try {
      SharedPreferences prefs = await SharedPreferences.getInstance();
      dio.options.headers["api-key"] = "coding_games";
      dio.options.headers["Authorization"] = "Bearer ${prefs.getString('login_token')}";
      final response = await dio.put('$baseURL/friend_list/$requestId');
      return UserResponseMapper.fromJson(response.data);
    } catch (e) {
      developer.log(e.toString());
      return null;
    }
  }

  static Future<List<Ranking>> getRankingForGame(int gameId) async {
    try {
      SharedPreferences prefs = await SharedPreferences.getInstance();
      dio.options.headers["api-key"] = "coding_games";
      dio.options.headers["Authorization"] = "Bearer ${prefs.getString('login_token')}";
      final response = await dio.get('$baseURL/ranking/game/$gameId');
      final data = response.data as List<dynamic>;
      List<Ranking> rankings =  data.map((json) => RankingResponseMapper.fromJson(json)).toList();
      return rankings;
    } catch (e) {
      developer.log(e.toString());
      return [];
    }
  }

  static Future<List<Ranking>> getRankingFriendsForGame(int gameId) async {
    try {
      SharedPreferences prefs = await SharedPreferences.getInstance();
      dio.options.headers["api-key"] = "coding_games";
      dio.options.headers["Authorization"] = "Bearer ${prefs.getString('login_token')}";
      final response = await dio.get('$baseURL/ranking/friend/$gameId');
      final data = response.data as List<dynamic>;
      List<Ranking> rankings =  data.map((json) => RankingResponseMapper.fromJson(json)).toList();
      return rankings;
    } catch (e) {
      developer.log(e.toString());
      return [];
    }
  }

  static Future<List<Ranking>> getRankingForUser(int userId) async {
    try {
      SharedPreferences prefs = await SharedPreferences.getInstance();
      dio.options.headers["api-key"] = "coding_games";
      dio.options.headers["Authorization"] = "Bearer ${prefs.getString('login_token')}";
      final response = await dio.get('$baseURL/ranking/user/$userId');
      final data = response.data as List<dynamic>;
      List<Ranking> rankings =  data.map((json) => RankingResponseMapper.fromJson(json)).toList();
      return rankings;
    } catch (e) {
      developer.log(e.toString());
      return [];
    }
  }

}