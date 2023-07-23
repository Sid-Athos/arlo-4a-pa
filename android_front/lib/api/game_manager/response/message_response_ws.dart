import 'dart:developer' as developer;

import 'package:miku/mapper/user/user_response_mapper.dart';
import 'package:miku/model/user/user_model.dart';

class MessageResponseWS {
  final String message;
  final User fromUser;

  MessageResponseWS({
    required this.message,
    required this.fromUser,
  });

  factory MessageResponseWS.fromJson(Map<String, dynamic> json) {
    return MessageResponseWS(
      message: json['message'],
      fromUser: UserResponseMapper.fromJson(json['from_user']),
    );
  }

  static void compute(Map<String, dynamic> json) {

    MessageResponseWS messageResponse = MessageResponseWS.fromJson(json['Message']);

    developer.log(messageResponse.message);
    developer.log(messageResponse.fromUser.toString());
  }
}