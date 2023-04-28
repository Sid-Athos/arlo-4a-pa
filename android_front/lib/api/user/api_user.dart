import 'package:dio/dio.dart';
import 'package:miku/api/user/mapper/session_response_mapper.dart';

import '../../model/session_model.dart';

class ApiUser {
  final String baseURL = "http://127.0.0.1:3000";
  final dio = Dio();

  Future<Session?> login(String email, String password) async {
    final response = await dio.get('$baseURL/user/login');
    if(response.statusCode == 200) {
      return SessionResponseMapper.fromJson(response.data);
    } else {
      return null;
    }
  }
}