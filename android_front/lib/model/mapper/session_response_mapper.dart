
import '../../../model/session_model.dart';

class SessionResponseMapper {

  static Session fromJson(Map<String, dynamic> json) {
    return Session(token: json['token']);
  }
}