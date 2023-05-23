import 'package:dio/dio.dart';
import 'package:miku/api/user/request/change_password_request.dart';
import 'package:miku/api/user/request/create_user_request.dart';
import 'package:miku/api/user/request/login_request.dart';
import 'package:miku/api/user/request/update_user_request.dart';
import 'package:miku/model/mapper/user_response_mapper.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'dart:developer' as developer;

import '../../model/mapper/session_response_mapper.dart';
import '../../model/session_model.dart';
import '../../model/user_model.dart';

class ApiUser {
  static const String baseURL = "http://192.168.137.117:7590";
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
      final response = await dio.post('$baseURL/user/create', data: createUserRequest.toJson());
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

  static Future<Session?> logout(String token) async {
    try {
      dio.options.headers["api-key"] = "coding_games";
      dio.options.headers["Authorization"] = "Bearer $token";
      final response = await dio.post('$baseURL/user/login');
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
      final response = await dio.delete('$baseURL/user/delete');
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
      final response = await dio.put('$baseURL/user/update', data: updateUserRequest.toJson());
      return UserResponseMapper.fromJson(response.data);
    } catch (e) {
      developer.log(e.toString());
      return null;
    }
  }

  static Future<List<User>?> search(String searched) async {
    try {
      dio.options.headers["api-key"] = "coding_games";
      final response = await dio.get('$baseURL/user/search?pseudo=$searched');
      final data = response.data as List<dynamic>;
      return data.map((json) => UserResponseMapper.fromJson(json)).toList();
    } catch (e) {
      developer.log(e.toString());
      return null;
    }
  }

  static Future<List<User>> getFriendList() async {
    try {
      SharedPreferences prefs = await SharedPreferences.getInstance();
      dio.options.headers["api-key"] = "coding_games";
      dio.options.headers["Authorization"] = "Bearer ${prefs.getString('login_token')}";
      final response = await dio.get('$baseURL/friend_list/');
      final data = response.data as List<dynamic>;
      return data.map((json) => UserResponseMapper.fromJson(json)).toList();
    } catch (e) {
      developer.log(e.toString());
      return [];
    }
  }

}