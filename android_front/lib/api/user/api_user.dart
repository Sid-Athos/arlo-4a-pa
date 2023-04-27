import 'package:dio/dio.dart';
import 'package:miku/api/user/request/create_user_request.dart';
import 'package:miku/api/user/request/login_request.dart';
import 'package:miku/model/mapper/user_response_mapper.dart';

import '../../model/mapper/session_response_mapper.dart';
import '../../model/session_model.dart';
import '../../model/user_model.dart';

class ApiUser {
  final String baseURL = "http://dev.mikusupremacy.fr:7590";
  final dio = Dio();

  Future<User?> getById(int id) async {
    final response = await dio.get('$baseURL/user/$id');
    if(response.statusCode == 200) {
      return UserResponseMapper.fromJson(response.data);
    } else {
      return null;
    }
  }

  Future<User?> create(CreateUserRequest createUserRequest) async {
    final response = await dio.post('$baseURL/user/create', data: createUserRequest.toJson());
    if(response.statusCode == 200) {
      return UserResponseMapper.fromJson(response.data);
    } else {
      return null;
    }
  }

  Future<Session?> login(LoginRequest loginRequest) async {
    try {
      final response = await dio.post('$baseURL/user/login', data: loginRequest.toJson());
      return SessionResponseMapper.fromJson(response.data);
    } catch (e) {
      return null;
    }
  }

  Future<Session?> logout(String token) async {
    dio.options.headers["Authorization"] = "Bearer $token";
    final response = await dio.post('$baseURL/user/login');
    if(response.statusCode == 200) {
      return SessionResponseMapper.fromJson(response.data);
    } else {
      return null;
    }
  }


}